use std::{cell::RefCell, rc::Rc, sync::OnceLock};

use gloo_timers::callback::Interval;
use web_sys::CanvasRenderingContext2d;

use crate::domain::{
    plane::cartesian::{to_matrix, CartesianPoint},
    model::{
        get_cell_size, get_length, get_middle_cell, iterate, move_in_plane, toggle_cell, zoom,
        Model, Rect,
    },
    preset::{get_preset_groups, Preset},
};

struct PresetOptionItem {
    pub label: String,
    pub value: String,
}

struct PresetOptionGroup {
    pub label: String,
    pub value: String,
    pub options: Vec<PresetOptionItem>,
}

pub fn build_presets() -> Vec<Preset> {
    return get_preset_groups()
        .iter()
        .flat_map(|group| {
            group
                .sub_groups
                .iter()
                .flat_map(|sub_group| sub_group.items.clone())
                .collect::<Vec<Preset>>()
        })
        .collect();
}

pub fn build_preset_option_groups() -> Vec<PresetOptionGroup> {
    return get_preset_groups()
        .iter()
        .map(|group| PresetOptionGroup {
            label: group.info.name.clone(),
            value: group.info.id.clone(),
            options: group
                .sub_groups
                .iter()
                .flat_map(|sub_group| sub_group.items.clone())
                .map(|item| PresetOptionItem {
                    label: item.name,
                    value: item.id,
                })
                .collect(),
        })
        .collect();
}

struct Square {
    pub x: i64,
    pub y: i64,
    pub size: u64,
}

trait DrawContext {
    fn clear(self, square: Square);
    fn draw_square(self, square: Square, color: String);
}

#[derive(Clone)]
struct Holder {
    context: CanvasRenderingContext2d,
}

impl DrawContext for Holder {
    fn clear(self, square: Square) {
        self.context.set_fill_style_str("white");
        self.context.fill_rect(
            square.x as f64,
            square.y as f64,
            square.size as f64,
            square.size as f64,
        );
    }

    fn draw_square(self, square: Square, color: String) {
        self.context.set_fill_style_str(&color);
        self.context.fill_rect(
            square.x as f64,
            square.y as f64,
            square.size as f64,
            square.size as f64,
        );
    }
}

#[derive(Clone)]
enum Status {
    Resumed,
    Paused,
}

#[derive(Clone)]
struct SystemSettings {
    pub preset: Option<String>,
    pub gap: u16,
    pub fps: u16,
    pub status: Status,
    pub dim: u16,
}

pub struct SystemModel {
    pub model: Model,
    pub settings: SystemSettings,
    on_change_listeners: Vec<Box<dyn FnMut(Prop)>>,
    // pub interval: Option<Interval>,
}

impl Default for SystemModel {
    fn default() -> Self {
        SystemModel {
            model: Model::from_pos(Rect::from(-10, -10, 10, 10)),
            settings: SystemSettings {
                preset: Some(String::from("block")),
                dim: 0,
                gap: 0,
                fps: 4,
                status: Status::Paused,
            },
            on_change_listeners: vec![],
        }
    }
}

pub fn add_on_change_listener<F>(cb: F)
where
    F: FnMut(Prop) + 'static,
{
    let system_model = get_instance();
    system_model.on_change_listeners.push(Box::new(cb));
}

fn on_change(param: Prop) {
    let system_model = get_instance();
    for cb in &mut system_model.on_change_listeners {
        cb(param.clone());
    }
}

fn fps_to_mili(fps: u16) -> u16 {
    return 1000 / fps;
}

#[derive(Clone)]
enum Prop {
    Model,
    Preset,
    Gap,
    FPS,
    Status,
    Dim,
}

static SYSTEM_MODEL_INSTANCE: OnceLock<SystemModel> = OnceLock::new();

fn get_instance() -> &'static mut SystemModel {
    let instance = SYSTEM_MODEL_INSTANCE.get_or_init(|| SystemModel { ..Default::default() } );
    let mut_instance = unsafe { &mut *(instance as *const _ as *mut SystemModel) };
    mut_instance
}

const DEAD_COLOR: &str = "#dbdbdb";
const ALIVE_COLOR: &str = "#2e2e2e";

fn render(draw_context: Rc<RefCell<Holder>>) {
    let system_model = get_instance();
    let length = get_length(&system_model.model);
    let cell_size = get_cell_size(&system_model.model, system_model.settings.dim);
    let middle_cell = get_middle_cell(&system_model.model, system_model.settings.dim);
    let dim = Square {
        x: 0,
        y: 0,
        size: system_model.settings.dim.into(),
    };
    draw_context
        .borrow()
        .clone()
        .draw_square(dim, DEAD_COLOR.to_string());
    system_model.model.value.iter().for_each(|point| {
        let arr_index = to_matrix(*point.0, length.into());
        let square = Square {
            x: arr_index.col as i64 * cell_size as i64 + system_model.settings.gap as i64 - middle_cell.x,
            y: arr_index.row as i64 * cell_size as i64 + system_model.settings.gap as i64 + middle_cell.y,
            size: cell_size as u64 - system_model.settings.gap as u64 * 2,
        };
        draw_context
            .borrow()
            .clone()
            .draw_square(square, ALIVE_COLOR.to_string());
    });
}

pub fn init(holder: Rc<RefCell<Holder>>) {
    let mut interval: Option<Interval> = None;

    add_on_change_listener(|prop| {
        let system_model = get_instance();
        match system_model.settings.status {
            Status::Resumed => match prop {
                Prop::Status | Prop::FPS => {
                    interval = Some(Interval::new(fps_to_mili(system_model.settings.fps).into(), move || {
                        control_iterate();
                        render(holder.clone());
                    }));
                }
                _ => {}
            },
            Status::Paused => match prop {
                Prop::Gap | Prop::Dim | Prop::Model => {
                    render(holder.clone());
                }
                Prop::Status => {
                    if let Some(t) = interval {
                        t.cancel();
                    }
                }
                _ => {}
            },
        }
    });
    render(holder.clone());
    // control_pause();
}

pub fn control_pause() {
    let system_model = get_instance();
    system_model.settings.status = Status::Paused;
    on_change(Prop::Status);
}

pub fn control_resume() {
    let system_model = get_instance();
    system_model.settings.status = Status::Resumed;
    on_change(Prop::Status);
}

pub fn control_set_preset(system_model: &mut SystemModel, preset: String) {
    if let Some(selected_preset) = build_presets().iter().find(|p| p.id == preset) {
        // wip
        system_model.model = Model {
            iter: selected_preset.iter,
             pos: selected_preset.pos,
             value: selected_preset.value.iter()
        }
        system_model.settings.preset = Some(preset);
        on_change(Prop::Model);
        on_change(Prop::Preset);
    }
}

pub fn control_set_dimension(dim: u16) {
    let system_model = get_instance();
    system_model.settings.dim = dim;
    on_change(Prop::Dim);
}

pub fn control_set_gap(gap: u16) {
    let system_model = get_instance();
    system_model.settings.gap = gap;
    on_change(Prop::Gap);
}

pub fn control_set_fps(fps: u16) {
    let system_model = get_instance();
    system_model.settings.fps = fps;
    on_change(Prop::FPS);
}

pub fn control_single_iteration() {
    let system_model = get_instance();
    system_model.settings.status = Status::Paused;
    iterate(&mut system_model.model);
    on_change(Prop::Status);
    on_change(Prop::Model);
}

pub fn control_iterate() {
    let system_model = get_instance();
    iterate(&mut system_model.model);
    on_change(Prop::Model);
}

pub fn control_toggle_model_cell(point: CartesianPoint) {
    let system_model = get_instance();
    toggle_cell(&mut system_model.model, point);
    system_model.settings.preset = None;
    on_change(Prop::Model);
    on_change(Prop::Preset);
}

pub fn control_set_size(new_size: u16) {
    let system_model = get_instance();
    zoom(&mut system_model.model, new_size);
    on_change(Prop::Model);
}

pub fn control_move_model(delta: CartesianPoint) {
    let system_model = get_instance();
    move_in_plane(&mut system_model.model, delta);
    on_change(Prop::Model);
}

pub fn control_get_settings() -> SystemSettings {
    get_instance().settings.clone()
}
