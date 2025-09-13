use gloo_timers::callback::Interval;
use std::cell::RefCell;
use web_sys::CanvasRenderingContext2d;

use libre_game_of_life_lib::{
    preset::{Preset, get_preset, get_preset_groups, try_get_preset},
    render::{RenderSettings, get_values_to_render},
    universe::{
        CartesianPoint, MatrixPoint, Universe, universe_get_camera, universe_iterate,
        universe_toggle, universe_toggle_by_matrix_point,
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
    get_preset_groups()
        .iter()
        .flat_map(|group| {
            group
                .sub_groups
                .iter()
                .flat_map(|sub_group| sub_group.items.clone())
                .collect::<Vec<Preset>>()
        })
        .collect()
}

pub fn build_preset_option_groups() -> Vec<PresetOptionGroup> {
    get_preset_groups()
        .iter()
        .map(|group| PresetOptionGroup {
            label: group.info.name.clone(),
            value: group.info.id.clone(),
            options: group
                .sub_groups
                .iter()
                .flat_map(|sub_group| sub_group.items.clone())
                .map(|item| PresetOptionItem { label: item.name, value: item.id })
                .collect(),
        })
        .collect()
}

#[derive(Clone)]
pub struct Holder {
    context: CanvasRenderingContext2d,
}

impl Holder {
    fn draw_square(&self, r: manfredo::cartesian::rect::rect_f64::Rect, color: String) {
        if self.context.is_undefined() || self.context.is_null() {
            return;
        }
        self.context.set_fill_style(&color.into());
        self.context.fill_rect(
            r.min.x,
            r.min.y,
            manfredo::cartesian::rect::rect_f64::delta_x(&r),
            manfredo::cartesian::rect::rect_f64::delta_y(&r),
        );
    }
}

unsafe impl Send for Holder {}

#[derive(Debug, PartialEq, Clone)]
pub enum Status {
    Resumed,
    Paused,
}

#[derive(Debug, PartialEq, Clone)]
pub struct AppSettings {
    pub preset: Option<String>,
    pub fps: u16,
    pub status: Status,
    pub render_settings: RenderSettings,
}

pub struct Model {
    pub universe: Universe,
    pub settings: AppSettings,
    pub holder: Option<Holder>,
}

impl Default for Model {
    fn default() -> Self {
        let universe = get_preset("block");
        let cam = universe_get_camera(&universe);
        Model {
            universe,
            settings: AppSettings {
                preset: Some("block".into()),
                fps: 4,
                status: Status::Paused,
                render_settings: RenderSettings { cam, dim: 0, gap: 0 },
            },
            holder: None,
        }
    }
}

thread_local! {
    static MODEL: RefCell<Model> = RefCell::new(Model::default());
}

thread_local! {
    static LISTENERS: RefCell<Vec<Box<dyn FnMut(Prop) + 'static>>> = RefCell::new(Vec::new());
}

#[derive(Debug, Clone)]
pub enum Prop {
    Universe,
    Preset,
    Gap,
    FPS,
    Status,
    Dim,
    Cam,
}

pub fn add_on_change_listener<F>(cb: F)
where
    F: FnMut(Prop) + 'static,
{
    LISTENERS.with_borrow_mut(|l| l.push(Box::new(cb)));
}

fn on_change(param: Prop) {
    LISTENERS.with_borrow_mut(|l| {
        for cb in l.iter_mut() {
            cb(param.clone());
        }
    });
}

fn fps_to_mili(fps: u16) -> u16 {
    1000 / fps
}

const DEAD_COLOR: &str = "#dbdbdb";
const ALIVE_COLOR: &str = "#2e2e2e";

fn render() {
    let (universe, settings, holder) = MODEL.with(|m| {
        let model = m.borrow();
        (model.universe.clone(), model.settings.clone(), model.holder.clone())
    });
    if settings.render_settings.dim == 0 {
        return;
    }
    if let Some(holder) = holder {
        let bg = manfredo::cartesian::rect::rect_f64::Rect::of(
            0.0,
            0.0,
            f64::from(settings.render_settings.dim),
            f64::from(settings.render_settings.dim),
        );
        holder.draw_square(bg, DEAD_COLOR.into());
        let values_to_render = get_values_to_render(&universe, &settings.render_settings);
        for sq in values_to_render {
            holder.draw_square(sq, ALIVE_COLOR.into());
        }
    }
}

pub enum Command {
    Start,
    Stop,
}

pub fn app_init(context: CanvasRenderingContext2d) {
    MODEL.with(|m| m.borrow_mut().holder = Some(Holder { context }));
    let mut interval: Option<Interval> = None;
    add_on_change_listener({
        move |prop| {
            let status = MODEL.with(|m| m.borrow().settings.status.clone());
            match status {
                Status::Resumed => match prop {
                    Prop::Status | Prop::FPS => {
                        if interval.is_none() {
                            if let Some(i) = interval.take() {
                                i.cancel();
                            }
                        }
                        let fps = MODEL.with(|m| m.borrow().settings.fps);
                        interval = Some(Interval::new(u32::from(fps_to_mili(fps)), || {
                            app_iterate();
                            render();
                        }))
                    }
                    _ => {}
                },
                Status::Paused => match prop {
                    Prop::Gap | Prop::Dim | Prop::Universe | Prop::Cam => {
                        render();
                    }
                    Prop::Status => {
                        if let Some(i) = interval.take() {
                            i.cancel();
                        }
                    }
                    _ => {}
                },
            }
        }
    });
    render();
}

pub fn app_pause() {
    MODEL.with(|m| {
        let mut model = m.borrow_mut();
        model.settings.status = Status::Paused;
    });
    on_change(Prop::Status);
}

pub fn app_resume() {
    MODEL.with(|m| {
        let mut model = m.borrow_mut();
        model.settings.status = Status::Resumed;
    });
    on_change(Prop::Status);
}

pub fn app_set_dimension(dim: u16) {
    MODEL.with(|m| {
        let mut model = m.borrow_mut();
        model.settings.render_settings.dim = dim;
    });
    on_change(Prop::Dim);
}

pub fn app_set_gap(gap: u8) {
    MODEL.with(|m| {
        let mut model = m.borrow_mut();
        model.settings.render_settings.gap = gap;
    });
    on_change(Prop::Gap);
}

pub fn app_set_fps(fps: u16) {
    MODEL.with(|m| {
        let mut model = m.borrow_mut();
        model.settings.fps = fps;
    });
    on_change(Prop::FPS);
}

pub fn app_set_preset(preset: String) {
    if let Some(selected_preset) = try_get_preset(&preset) {
        MODEL.with(|m| {
            let mut model = m.borrow_mut();
            model.settings.render_settings.cam = universe_get_camera(&selected_preset);
            model.universe = selected_preset;
            model.settings.preset = Some(preset);
        });
        on_change(Prop::Universe);
        on_change(Prop::Preset);
        on_change(Prop::Cam);
    }
}

pub fn app_single_iteration() {
    MODEL.with(|m| {
        let mut model = m.borrow_mut();
        model.settings.status = Status::Paused;
        universe_iterate(&mut model.universe);
    });
    on_change(Prop::Status);
    on_change(Prop::Universe);
}

pub fn app_iterate() {
    MODEL.with(|m| {
        let mut model = m.borrow_mut();
        universe_iterate(&mut model.universe);
    });
    on_change(Prop::Universe);
}

pub fn app_toggle_by_point(p: CartesianPoint) {
    MODEL.with(|m| {
        let mut model = m.borrow_mut();
        universe_toggle(&mut model.universe, p);
        model.settings.preset = None;
    });
    on_change(Prop::Universe);
    on_change(Prop::Preset);
}

pub fn app_toggle_model_cell_by_absolute_point(p: MatrixPoint) {
    MODEL.with(|m| {
        let mut model = m.borrow_mut();
        let render_settings = model.settings.render_settings.clone();
        universe_toggle_by_matrix_point(&mut model.universe, &render_settings, p);
        model.settings.preset = None;
    });
    on_change(Prop::Universe);
    on_change(Prop::Preset);
}

pub fn app_zoom_in() {
    let cam = MODEL.with(|m| m.borrow().settings.render_settings.cam.clone());
    if manfredo::cartesian::rect::rect_i32::max_len(&cam) <= 2 {
        return;
    }
    MODEL.with(|m| {
        let mut model = m.borrow_mut();
        manfredo::cartesian::rect::rect_i32::assign_deflate(&mut model.settings.render_settings.cam);
    });
    on_change(Prop::Cam);
}

pub fn app_zoom_out() {
    let cam = &MODEL.with(|m| m.borrow().settings.render_settings.cam.clone());
    if manfredo::cartesian::rect::rect_i32::max_len(cam) > 200 {
        return;
    }
    MODEL.with(|m| {
        let mut model = m.borrow_mut();
        manfredo::cartesian::rect::rect_i32::try_saturating_inflate_assign(&mut model.settings.render_settings.cam);
    });
    on_change(Prop::Cam);
}

pub fn app_zoom_to(new_size: u32) {
    if new_size < 2 || new_size > 200 {
        return;
    }
    MODEL.with(|m| {
        let mut model = m.borrow_mut();
        manfredo::cartesian::rect::rect_i32::saturating_resize_assign(
            &mut model.settings.render_settings.cam,
            new_size,
        );
    });
    on_change(Prop::Cam);
}

pub fn app_move_cam(delta: CartesianPoint) {
    MODEL.with(|m| {
        let mut model = m.borrow_mut();
        manfredo::cartesian::rect::rect_i32::saturating_translate_assign(
            &mut model.settings.render_settings.cam,
            &delta,
        );
    });
    on_change(Prop::Universe);
}

#[derive(Debug, PartialEq)]
pub struct AppInfo {
    pub preset: Option<String>,
    pub gap: u8,
    pub size: u32,
    pub fps: u16,
    pub status: Status,
    pub age: u64,
}

pub fn app_get_settings() -> AppInfo {
    MODEL.with(|m| {
        let model = m.borrow();
        let settings = model.settings.clone();
        let universe = model.universe.clone();
        AppInfo {
            preset: settings.preset,
            gap: settings.render_settings.gap,
            size: manfredo::cartesian::rect::rect_i32::max_len(&settings.render_settings.cam),
            fps: settings.fps,
            status: settings.status,
            age: universe.age,
        }
    })
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use libre_game_of_life_lib::{
        cell::State,
        preset::get_preset,
        render::RenderSettings,
        universe::{CartesianPoint, Universe},
    };
    use manfredo::cartesian::rect::rect_i32::Rect;

    use super::{
        AppInfo, AppSettings, MODEL, Status, app_get_settings, app_iterate, app_move_cam,
        app_pause, app_resume, app_set_dimension, app_set_fps, app_set_gap, app_set_preset,
        app_single_iteration, app_toggle_by_point, app_zoom_in, app_zoom_out, app_zoom_to,
    };

    #[test]
    fn test_instance() {
        assert_eq!(
            MODEL.with(|m| m.borrow().settings.clone()),
            AppSettings {
                preset: Some("block".into()),
                fps: 4,
                status: Status::Paused,
                render_settings: RenderSettings { cam: Rect::of(-5, -5, 4, 4), dim: 0, gap: 0 }
            }
        );
        assert_eq!(MODEL.with(|m| m.borrow().universe.clone()), get_preset("block"));
        let settings = app_get_settings();
        assert_eq!(
            AppInfo {
                preset: Some("block".into()),
                gap: 0,
                size: 10,
                fps: 4,
                status: Status::Paused,
                age: 0,
            },
            settings
        );

        app_pause();
        assert_eq!(
            MODEL.with(|m| m.borrow().settings.clone()),
            AppSettings {
                preset: Some("block".into()),
                fps: 4,
                status: Status::Paused,
                render_settings: RenderSettings { cam: Rect::of(-5, -5, 4, 4), dim: 0, gap: 0 }
            }
        );

        app_resume();
        assert_eq!(
            MODEL.with(|m| m.borrow().settings.clone()),
            AppSettings {
                preset: Some("block".into()),
                fps: 4,
                status: Status::Resumed,
                render_settings: RenderSettings { cam: Rect::of(-5, -5, 4, 4), dim: 0, gap: 0 }
            }
        );

        app_set_dimension(1080);
        assert_eq!(
            MODEL.with(|m| m.borrow().settings.clone()),
            AppSettings {
                preset: Some("block".into()),
                fps: 4,
                status: Status::Resumed,
                render_settings: RenderSettings {
                    cam: Rect::of(-5, -5, 4, 4),
                    dim: 1080,
                    gap: 0
                }
            }
        );

        app_set_gap(2);
        assert_eq!(
            MODEL.with(|m| m.borrow().settings.clone()),
            AppSettings {
                preset: Some("block".into()),
                fps: 4,
                status: Status::Resumed,
                render_settings: RenderSettings {
                    cam: Rect::of(-5, -5, 4, 4),
                    dim: 1080,
                    gap: 2
                }
            }
        );

        app_set_fps(60);
        assert_eq!(
            MODEL.with(|m| m.borrow().settings.clone()),
            AppSettings {
                preset: Some("block".into()),
                fps: 60,
                status: Status::Resumed,
                render_settings: RenderSettings {
                    cam: Rect::of(-5, -5, 4, 4),
                    dim: 1080,
                    gap: 2
                }
            }
        );

        app_set_preset("Gaius Julius Caesar".into());
        assert_eq!(
            MODEL.with(|m| m.borrow().settings.clone()),
            AppSettings {
                preset: Some("block".into()),
                fps: 60,
                status: Status::Resumed,
                render_settings: RenderSettings {
                    cam: Rect::of(-5, -5, 4, 4),
                    dim: 1080,
                    gap: 2
                }
            }
        );
        app_set_preset("r_pentomino".into());
        assert_eq!(
            MODEL.with(|m| m.borrow().settings.clone()),
            AppSettings {
                preset: Some("r_pentomino".into()),
                fps: 60,
                status: Status::Resumed,
                render_settings: RenderSettings {
                    cam: Rect::of(-5, -5, 5, 5),
                    dim: 1080,
                    gap: 2
                }
            }
        );
        app_set_preset("block".into());
        app_iterate();
        let block = get_preset("block");
        assert_eq!(
            MODEL.with(|m| m.borrow().universe.clone()),
            Universe { age: 1, value: block.value.clone() }
        );
        assert_eq!(
            MODEL.with(|m| m.borrow().settings.clone()),
            AppSettings {
                preset: Some("block".into()),
                fps: 60,
                status: Status::Resumed,
                render_settings: RenderSettings {
                    cam: Rect::of(-5, -5, 4, 4),
                    dim: 1080,
                    gap: 2
                }
            }
        );

        app_single_iteration();
        assert_eq!(
            MODEL.with(|m| m.borrow().universe.clone()),
            Universe { age: 2, value: block.value.clone() }
        );
        assert_eq!(
            MODEL.with(|m| m.borrow().settings.clone()),
            AppSettings {
                preset: Some("block".into()),
                fps: 60,
                status: Status::Paused,
                render_settings: RenderSettings {
                    cam: Rect::of(-5, -5, 4, 4),
                    dim: 1080,
                    gap: 2
                }
            }
        );

        app_zoom_to(4);
        app_zoom_in();
        app_zoom_in();
        app_zoom_in();
        app_zoom_in();
        assert_eq!(
            MODEL.with(|m| m.borrow().settings.clone()),
            AppSettings {
                preset: Some("block".into()),
                fps: 60,
                status: Status::Paused,
                render_settings: RenderSettings {
                    cam: Rect::of(-1, -1, 0, 0),
                    dim: 1080,
                    gap: 2
                }
            }
        );
        app_zoom_to(198);
        app_zoom_out();
        app_zoom_out();
        app_zoom_out();
        app_zoom_out();
        assert_eq!(
            MODEL.with(|m| m.borrow().settings.clone()),
            AppSettings {
                preset: Some("block".into()),
                fps: 60,
                status: Status::Paused,
                render_settings: RenderSettings {
                    cam: Rect::of(-101, -101, 100, 100),
                    dim: 1080,
                    gap: 2,
                }
            }
        );

        app_zoom_to(40);
        assert_eq!(
            MODEL.with(|m| m.borrow().settings.clone()),
            AppSettings {
                preset: Some("block".into()),
                fps: 60,
                status: Status::Paused,
                render_settings: RenderSettings {
                    cam: Rect::of(-20, -20, 19, 19),
                    dim: 1080,
                    gap: 2,
                }
            }
        );
        app_zoom_in();
        assert_eq!(
            MODEL.with(|m| m.borrow().settings.clone()),
            AppSettings {
                preset: Some("block".into()),
                fps: 60,
                status: Status::Paused,
                render_settings: RenderSettings {
                    cam: Rect::of(-19, -19, 18, 18),
                    dim: 1080,
                    gap: 2,
                }
            }
        );
        app_zoom_out();
        assert_eq!(
            MODEL.with(|m| m.borrow().settings.clone()),
            AppSettings {
                preset: Some("block".into()),
                fps: 60,
                status: Status::Paused,
                render_settings: RenderSettings {
                    cam: Rect::of(-20, -20, 19, 19),
                    dim: 1080,
                    gap: 2,
                }
            }
        );

        app_move_cam(CartesianPoint::of(20, 20));
        assert_eq!(
            MODEL.with(|m| m.borrow().universe.clone()),
            Universe { age: 2, value: block.value.clone() }
        );

        app_toggle_by_point(CartesianPoint::of(0, 0));
        assert_eq!(
            MODEL.with(|m| m.borrow().universe.clone()),
            Universe {
                age: 2,
                value: HashMap::from([
                    (CartesianPoint::of(-1, -1), State::Alive),
                    (CartesianPoint::of(-1, 0), State::Alive),
                    (CartesianPoint::of(0, -1), State::Alive),
                ])
            }
        );
    }
}
