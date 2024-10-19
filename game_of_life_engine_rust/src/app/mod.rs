use std::{cell::RefCell, rc::Rc, sync::OnceLock};

use gloo_timers::callback::Interval;
use web_sys::CanvasRenderingContext2d;

use crate::domain::{
    cartesian_plane::{point_to_index, Point},
    model::{
        get_cell_size, get_length, get_middle_cell, move_in_plane, toggle_cell, zoom,
        Model, Rect,
    },
    preset::{get_preset_groups, Preset},
};

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

impl DrawContext for Holder  {
    fn clear(self, square: Square) {
        self.context.set_fill_style_str("white");
        self.context.fill_rect(square.x as f64, square.y as f64, square.size as f64, square.size as f64);
    }

    fn draw_square(self, square: Square, color: String) {
        self.context.set_fill_style_str(&color);
        self.context.fill_rect(square.x as f64, square.y as f64, square.size as f64, square.size as f64);
    }
}

#[derive(Clone)]
enum Status {
    Resumed,
    Paused,
}

#[derive(Clone)]
pub struct SystemModel {
    pub model: Model,
    pub gap: u16,
    pub fps: u16,
    pub status: Status,
    pub dim: u16,
    // pub draw_context: DrawContext,
}

pub fn build_model(
    // draw_context: DrawContext,
    dim: u16,
) -> SystemModel {
    SystemModel {
        model: Model::from_pos(Rect::from(-10, -10, 10, 10)),
        dim,
        gap: 1,
        fps: 4,
        status: Status::Paused,
        //draw_context,
    }
}

struct PresetOptionItem {
    pub label: String,
    pub value: String,
}

struct PresetOptionGroup {
    pub label: String,
    pub value: String,
    pub options: Vec<PresetOptionItem>,
}

pub struct SystemController {
    pub system: SystemModelProxy,
    pub interval: Option<Interval>,
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

fn fps_to_mili(fps: u16) -> u16 {
    return 1000 / fps;
}

const DEAD_COLOR: &str = "#dbdbdb";
const ALIVE_COLOR: &str = "#2e2e2e";

fn render(system: &SystemModelProxy, draw_context: Rc<RefCell<Holder>>) {
    let model = system.get_model();
    let length = get_length(&model.model);
    let cell_size = get_cell_size(&model.model, model.dim);
    let middle_cell = get_middle_cell(&model.model, model.dim);
    let dim = Square {
        x: 0,
        y: 0,
        size: model.dim.into(),
    };
    draw_context.borrow().clone().draw_square(dim, DEAD_COLOR.to_string());

    model.model.value.iter().for_each(|point| {
        let arr_index = point_to_index(*point.0, length.into());
        let square = Square {
            x: arr_index.col as i64 * cell_size as i64 + model.gap as i64 - middle_cell.x,
            y: arr_index.row as i64 * cell_size as i64 + model.gap as i64 + middle_cell.y,
            size: cell_size as u64 - model.gap as u64 * 2,
        };
        draw_context.borrow().clone().draw_square(square, ALIVE_COLOR.to_string());
    });
}

pub fn init(system_controller: &mut SystemController, holder: Holder) {
    system_controller
        .system
        .add_on_change_listener(|m, prop| {
            let model = m.get_model();
            match model.status {
                Status::Paused => {
                    match prop {
                        Prop::Status | Prop::FPS => {
                            system_controller.interval = Some(Interval::new(fps_to_mili(model.fps).into(), move || {
                                system_controller.iterate();
                                render(m, &holder);
                            }));
                        }
                        _ => {}
                    }
                }
                Status::Resumed => {
                    match prop {
                        Prop::Gap | Prop::Dim | Prop::Model => {
                            render(m, &holder);
                        }
                        Prop::Status => {
                            if let Some(t) = system_controller.interval {
                                t.cancel();
                            }
                        }
                        _ => {}
                    }
                }
            }
        });
    render(&system_controller.system, &holder);
    system_controller.pause();
}


#[derive(Clone)]
enum Prop {
    Model,
    Gap,
    FPS,
    Status,
    Dim,
    DrawContext,
}

type OnModelChange = Box<dyn Fn(&SystemModelProxy, Prop)>;

pub struct SystemModelProxy {
    model: SystemModel,
    on_change_listeners: Vec<OnModelChange>,
}


pub fn add_on_change_listener<F>(&mut self, cb: F)
where
    F: Fn(&Self, Prop) + 'static,
{
    self.on_change_listeners.push(Box::new(cb));
}

fn on_change(&self, param: Prop) {
    for cb in &self.on_change_listeners {
        cb(&self, param.clone());
    }
}

// exposed functions



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



static SYSTEM_MODEL_INSTANCE: OnceLock<SystemModel> = OnceLock::new();

fn get_instance() -> &'static SystemModel {
    SYSTEM_MODEL_INSTANCE.get_or_init(|| SystemModel {
        model: Model::from_pos(Rect::from(-10, -10, 10, 10)),
        dim: 0,
        gap: 1,
        fps: 4,
        status: Status::Paused,
    })
}

fn wip_get_instance() {
    if let Some(smi) = SYSTEM_MODEL_INSTANCE.get() {
        let mut_instance = unsafe { &mut *(smi as *const _ as *mut SystemModel) };
    }
}

pub fn control_pause(system_model: &mut SystemModel) {
    system_model.status = Status::Paused;
    on_change(Prop::Status);
}

pub fn control_resume(system_model: &mut SystemModel) {
    system_model.status = Status::Resumed;
    on_change(Prop::Status);
}

pub fn control_single_iteration(system_model: &mut SystemModel) {
    system_model.status = Status::Paused;
    iterate(&mut system_model.model);

    on_change(Prop::Status);
    on_change(Prop::Model);
}

pub fn control_iterate(system_model: &mut SystemModel) {
    iterate(&mut self.system.get_model().model);
    self.system.set_model();
}

pub fn control_toggle_model_cell(system_model: &mut SystemModel, point: Point) {
    toggle_cell(&mut self.system.get_model().model, point);
    self.system.set_model();
}

pub fn control_set_preset(system_model: &mut SystemModel, preset: String) {
    if Some(selected_preset) = build_presets().iter().find(|p| p.id == preset) {
        self.system.set_model(selected_preset.model);
    }
    self.system.set_model();
}

pub fn control_set_dimension(system_model: &mut SystemModel, dim: u16) {
    self.system.set_dimension(dim);       
}

pub fn control_set_gap(system_model: &mut SystemModel, gap: u16) {
    self.system.set_gap(gap);
}

pub fn control_set_size(system_model: &mut SystemModel, new_size: u16) {
    zoom(&mut self.system.get_model().model, new_size);
}

pub fn control_move_model(system_model: &mut SystemModel, delta: Point) {
    move_in_plane(&mut self.system.get_model().model, delta);
    self.system.set_model();
}

pub fn control_set_fps(system_model: &mut SystemModel, fps: u16) {
    self.system.set_fps(fps);
}