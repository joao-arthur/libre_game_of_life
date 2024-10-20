use std::{
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc, Mutex, OnceLock,
    },
    thread,
};

use gloo_timers::callback::Interval;
use web_sys::CanvasRenderingContext2d;

use crate::domain::{
    plane::cartesian::{to_matrix, CartesianPoint},
    preset::{get_preset, get_preset_groups, get_preset_unsafe, Preset},
    universe::{
        get_cell_size, get_length, get_middle_cell, iterate, move_in_plane, toggle_cell, zoom,
        Universe,
    },
};

pub struct PresetOptionItem {
    pub label: String,
    pub value: String,
}

pub struct PresetOptionGroup {
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

pub struct Square {
    pub x: i64,
    pub y: i64,
    pub size: u64,
}

pub trait DrawContext {
    fn clear(&self, s: Square);
    fn draw_square(&self, s: Square, color: String);
}

#[derive(Clone)]
pub struct Holder {
    context: CanvasRenderingContext2d,
}

impl DrawContext for Holder {
    fn clear(&self, s: Square) {
        self.context.set_fill_style_str("white");
        self.context
            .fill_rect(s.x as f64, s.y as f64, s.size as f64, s.size as f64);
    }

    fn draw_square(&self, s: Square, color: String) {
        self.context.set_fill_style_str(&color);
        self.context
            .fill_rect(s.x as f64, s.y as f64, s.size as f64, s.size as f64);
    }
}

unsafe impl Send for Holder {

}

#[derive(Clone, Debug, PartialEq)]
pub enum Status {
    Resumed,
    Paused,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Settings {
    pub preset: Option<String>,
    pub gap: u16,
    pub fps: u16,
    pub status: Status,
    pub dim: u16,
}

pub struct Model {
    pub universe: Universe,
    pub settings: Settings,
    pub holder: Option<Box<dyn DrawContext + Send + 'static>>,
    on_change_listeners: Mutex<Vec<Box<dyn FnMut(Prop) + Send + 'static>>>,
}

impl Default for Model {
    fn default() -> Self {
        Model {
            universe: get_preset_unsafe("block"),
            settings: Settings {
                preset: Some(String::from("block")),
                dim: 0,
                gap: 0,
                fps: 4,
                status: Status::Paused,
            },
            holder: None,
            on_change_listeners: Mutex::new(Vec::new()),
        }
    }
}

static MODEL_INSTANCE: OnceLock<Mutex<Model>> = OnceLock::new();

fn get_instance() -> std::sync::MutexGuard<'static, Model> {
    let instance = MODEL_INSTANCE.get_or_init(|| {
        Mutex::new(Model {
            ..Default::default()
        })
    });
    instance.lock().unwrap()
}

#[derive(Clone)]
pub enum Prop {
    Universe,
    Preset,
    Gap,
    FPS,
    Status,
    Dim,
}

pub fn add_on_change_listener<F>(cb: F)
where
    F: FnMut(Prop) + Send + 'static,
{
    let mut model = get_instance();
    let mut listeners = model.on_change_listeners.lock().unwrap();
    listeners.push(Box::new(cb));
}

fn on_change(param: Prop) {
    let mut model = get_instance();
    let mut listeners = model.on_change_listeners.lock().unwrap();
    for cb in listeners.iter_mut() {
        cb(param.clone());
    }
}

fn fps_to_mili(fps: u16) -> u16 {
    return 1000 / fps;
}

const DEAD_COLOR: &str = "#dbdbdb";
const ALIVE_COLOR: &str = "#2e2e2e";

fn render() {
    let mut model = get_instance();
    let holder = model.holder.as_ref().unwrap();
    let length = get_length(&model.universe);
    let cell_size = get_cell_size(&model.universe, model.settings.dim);
    let middle_cell = get_middle_cell(&model.universe, model.settings.dim);
    let background = Square {
        x: 0,
        y: 0,
        size: model.settings.dim.into(),
    };
    holder.draw_square(background, DEAD_COLOR.to_string());
    model.universe.value.iter().for_each(|point| {
        let arr_index = to_matrix(*point.0, length.into());
        let s = Square {
            x: arr_index.col as i64 * cell_size as i64 + model.settings.gap as i64 - middle_cell.x,
            y: arr_index.row as i64 * cell_size as i64 + model.settings.gap as i64 + middle_cell.y,
            size: cell_size as u64 - model.settings.gap as u64 * 2,
        };
        holder.draw_square(s, ALIVE_COLOR.to_string());
    });
}

pub enum Command {
    Start,
    Stop,
}

pub fn run_interval_controller() -> Sender<Command> {
    let (tx, rx): (Sender<Command>, Receiver<Command>) = channel();
    thread::spawn(move || {
        let mut interval: Option<Interval> = None;
        for command in rx {
            match command {
                Command::Start => {
                    if interval.is_none() {
                        let mut model = get_instance();
                        interval = Some(Interval::new(
                            fps_to_mili(model.settings.fps).into(),
                            || {
                                app_iterate();
                                render();
                            },
                        ));
                    }
                }
                Command::Stop => {
                    if let Some(i) = interval.take() {
                        i.cancel();
                    }
                }
            }
        }
    });
    tx
}

pub fn app_init(
    context: CanvasRenderingContext2d,
) {
    let mut model = get_instance();
    model.holder = Some(Box::new(Holder { context }));
    
    
    let interval: Arc<Mutex<Option<Interval>>> = Arc::new(Mutex::new(None));
    let controller = run_interval_controller();


    
    add_on_change_listener({
        let interval = Arc::clone(&interval);
        move |prop| {
            let mut model = get_instance();
            match model.settings.status {
                Status::Resumed => match prop {
                    Prop::Status | Prop::FPS => {
                        controller.clone().send(Command::Start).unwrap();
                    }
                    _ => {}
                },
                Status::Paused => match prop {
                    Prop::Gap | Prop::Dim | Prop::Universe => {
                        render();
                    }
                    Prop::Status => {
                        controller.clone().send(Command::Start).unwrap();
                    }
                    _ => {}
                },
            }
        }
    });
    render();
    // app_pause();
}

pub fn app_pause() {
    let mut model = get_instance();
    model.settings.status = Status::Paused;

    let mut listeners = model.on_change_listeners.lock().unwrap();
    for cb in listeners.iter_mut() {
        cb(Prop::Status);
    }
}

pub fn app_resume() {
    let mut model = get_instance();
    model.settings.status = Status::Resumed;

    let mut listeners = model.on_change_listeners.lock().unwrap();
    for cb in listeners.iter_mut() {
        cb(Prop::Status);
    }
}

pub fn app_set_dimension(dim: u16) {
    let mut model = get_instance();
    model.settings.dim = dim;

    let mut listeners = model.on_change_listeners.lock().unwrap();
    for cb in listeners.iter_mut() {
        cb(Prop::Dim);
    }
}

pub fn app_set_gap(gap: u16) {
    let mut model = get_instance();
    model.settings.gap = gap;

    let mut listeners = model.on_change_listeners.lock().unwrap();
    for cb in listeners.iter_mut() {
        cb(Prop::Gap);
    }
}

pub fn app_set_fps(fps: u16) {
    let mut model = get_instance();
    model.settings.fps = fps;

    let mut listeners = model.on_change_listeners.lock().unwrap();
    for cb in listeners.iter_mut() {
        cb(Prop::FPS);
    }
}

pub fn app_set_preset(preset: String) {
    let mut model = get_instance();
    if let Some(selected_preset) = get_preset(&preset) {
        model.universe = selected_preset;
        model.settings.preset = Some(preset);

        let mut listeners = model.on_change_listeners.lock().unwrap();
        for cb in listeners.iter_mut() {
            cb(Prop::Universe);
        }
        for cb in listeners.iter_mut() {
            cb(Prop::Preset);
        }
    }
}

pub fn app_single_iteration() {
    let mut model = get_instance();
    model.settings.status = Status::Paused;
    iterate(&mut model.universe);

    let mut listeners = model.on_change_listeners.lock().unwrap();
    for cb in listeners.iter_mut() {
        cb(Prop::Status);
    }
    for cb in listeners.iter_mut() {
        cb(Prop::Universe);
    }
}

pub fn app_iterate() {
    let mut model = get_instance();
    iterate(&mut model.universe);

    let mut listeners = model.on_change_listeners.lock().unwrap();
    for cb in listeners.iter_mut() {
        cb(Prop::Status);
    }
}

pub fn app_toggle_model_cell(point: CartesianPoint) {
    let mut model = get_instance();
    toggle_cell(&mut model.universe, point);
    model.settings.preset = None;

    let mut listeners = model.on_change_listeners.lock().unwrap();
    for cb in listeners.iter_mut() {
        cb(Prop::Universe);
    }
    for cb in listeners.iter_mut() {
        cb(Prop::Preset);
    }
}

pub fn app_zoom(new_size: u16) {
    let mut model = get_instance();
    zoom(&mut model.universe, new_size);

    let mut listeners = model.on_change_listeners.lock().unwrap();
    for cb in listeners.iter_mut() {
        cb(Prop::Universe);
    }
}

pub fn app_move_model(delta: CartesianPoint) {
    let mut model = get_instance();
    move_in_plane(&mut model.universe, delta);

    let mut listeners = model.on_change_listeners.lock().unwrap();
    for cb in listeners.iter_mut() {
        cb(Prop::Universe);
    }
}

pub fn app_get_settings() -> Settings {
    get_instance().settings.clone()
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::domain::{cell::State, plane::Rect};

    use super::*;

    #[test]
    fn test_instance() {
        assert_eq!(
            get_instance().settings,
            Settings {
                preset: Some(String::from("block")),
                dim: 0,
                gap: 0,
                fps: 4,
                status: Status::Paused,
            }
        );
        assert_eq!(get_instance().universe, get_preset_unsafe("block"));
        let settings = app_get_settings();
        assert_eq!(get_instance().settings, settings);

        app_pause();
        assert_eq!(
            get_instance().settings,
            Settings {
                preset: Some(String::from("block")),
                dim: 0,
                gap: 0,
                fps: 4,
                status: Status::Paused,
            }
        );

        app_resume();
        assert_eq!(
            get_instance().settings,
            Settings {
                preset: Some(String::from("block")),
                dim: 0,
                gap: 0,
                fps: 4,
                status: Status::Resumed,
            }
        );

        app_set_dimension(1080);
        assert_eq!(
            get_instance().settings,
            Settings {
                preset: Some(String::from("block")),
                dim: 1080,
                gap: 0,
                fps: 4,
                status: Status::Resumed,
            }
        );

        app_set_gap(2);
        assert_eq!(
            get_instance().settings,
            Settings {
                preset: Some(String::from("block")),
                dim: 1080,
                gap: 2,
                fps: 4,
                status: Status::Resumed,
            }
        );

        app_set_fps(60);
        assert_eq!(
            get_instance().settings,
            Settings {
                preset: Some(String::from("block")),
                dim: 1080,
                gap: 2,
                fps: 60,
                status: Status::Resumed,
            }
        );

        app_set_preset("Gaius Julius Caesar".to_string());
        assert_eq!(
            get_instance().settings,
            Settings {
                preset: Some(String::from("block")),
                dim: 1080,
                gap: 2,
                fps: 60,
                status: Status::Resumed,
            }
        );
        app_set_preset("r_pentomino".to_string());
        assert_eq!(
            get_instance().settings,
            Settings {
                preset: Some(String::from("r_pentomino")),
                dim: 1080,
                gap: 2,
                fps: 60,
                status: Status::Resumed,
            }
        );
        app_set_preset("block".to_string());
        app_iterate();
        let block = get_preset_unsafe("block");
        assert_eq!(
            get_instance().universe,
            Universe {
                iter: 1,
                pos: Rect::from(-10, -10, 10, 10),
                value: block.value.clone()
            }
        );
        assert_eq!(
            get_instance().settings,
            Settings {
                preset: Some(String::from("block")),
                dim: 1080,
                gap: 2,
                fps: 60,
                status: Status::Resumed,
            }
        );

        app_single_iteration();
        assert_eq!(
            get_instance().universe,
            Universe {
                iter: 2,
                pos: Rect::from(-10, -10, 10, 10),
                value: block.value.clone()
            }
        );
        assert_eq!(
            get_instance().settings,
            Settings {
                preset: Some(String::from("block")),
                dim: 1080,
                gap: 2,
                fps: 60,
                status: Status::Paused,
            }
        );

        app_zoom(41);
        assert_eq!(
            get_instance().universe,
            Universe {
                iter: 2,
                pos: Rect::from(-20, -20, 20, 20),
                value: block.value.clone()
            }
        );

        app_move_model(CartesianPoint::from(20, 20));
        assert_eq!(
            get_instance().universe,
            Universe {
                iter: 2,
                pos: Rect::from(0, 0, 40, 40),
                value: block.value.clone()
            }
        );

        app_toggle_model_cell(CartesianPoint::from(0, 0));
        assert_eq!(
            get_instance().universe,
            Universe {
                iter: 2,
                pos: Rect::from(0, 0, 40, 40),
                value: HashMap::from([
                    (CartesianPoint::from(-1, 1), State::Alive),
                    (CartesianPoint::from(0, 1), State::Alive),
                    (CartesianPoint::from(-1, 0), State::Alive),
                ])
            }
        );
    }
}
