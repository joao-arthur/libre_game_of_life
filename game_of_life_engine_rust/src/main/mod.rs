use crate::{
    app::{
        app_get_settings, app_init, app_move_model, app_pause, app_resume, app_set_dimension,
        app_set_fps, app_set_gap, app_set_preset, app_single_iteration, app_toggle_model_cell,
        app_zoom, Status,
    },
    domain::plane::cartesian::CartesianPoint,
};
use js_sys::Function;
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

#[wasm_bindgen]
pub struct EngineCartesianPoint {
    pub x: i64,
    pub y: i64,
}

#[wasm_bindgen]
impl EngineCartesianPoint {
    #[wasm_bindgen(constructor)]
    pub fn new(x: i64, y: i64) -> EngineCartesianPoint {
        EngineCartesianPoint { x, y }
    }
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum EngineStatus {
    Resumed,
    Paused,
}

#[wasm_bindgen]
pub struct EngineSettings {
    preset: Option<String>,
    pub gap: u16,
    pub fps: u16,
    pub status: EngineStatus,
    pub dim: u16,
}

#[wasm_bindgen]
impl EngineSettings {
    #[wasm_bindgen(getter)]
    pub fn preset(&self) -> Option<String> {
        self.preset.clone()
    }
}

#[wasm_bindgen(js_name = "engineInit")]
pub fn main_init(value: JsValue) {
    if let Ok(context) = value.dyn_into::<CanvasRenderingContext2d>() {
        app_init(context);
    }
}

#[wasm_bindgen(js_name = "enginePause")]
pub fn main_pause() {
    app_pause();
}

#[wasm_bindgen(js_name = "engineResume")]
pub fn main_resume() {
    app_resume();
}

#[wasm_bindgen(js_name = "engineSetDimension")]
pub fn main_set_dimension(dim: u16) {
    app_set_dimension(dim);
}

#[wasm_bindgen(js_name = "engineSetGap")]
pub fn main_set_gap(gap: u16) {
    app_set_gap(gap);
}

#[wasm_bindgen(js_name = "engineSetFPS")]
pub fn main_set_fps(fps: u16) {
    app_set_fps(fps);
}

#[wasm_bindgen(js_name = "engineSetPreset")]
pub fn main_set_preset(preset: String) {
    app_set_preset(preset);
}

#[wasm_bindgen(js_name = "engineSingleIteration")]
pub fn main_single_iteration() {
    app_single_iteration();
}

#[wasm_bindgen(js_name = "engineToggle")]
pub fn main_toggle_model_cell(point: EngineCartesianPoint) {
    let cp = CartesianPoint {
        x: point.x,
        y: point.y,
    };
    app_toggle_model_cell(cp);
}

#[wasm_bindgen(js_name = "engineZoom")]
pub fn main_zoom(new_size: u16) {
    app_zoom(new_size);
}

#[wasm_bindgen(js_name = "engineMove")]
pub fn main_move_model(delta: EngineCartesianPoint) {
    let cp = CartesianPoint {
        x: delta.x,
        y: delta.y,
    };
    app_move_model(cp);
}

#[wasm_bindgen(js_name = "engineGetSettings")]
pub fn main_get_settings() -> EngineSettings {
    let settings = app_get_settings();
    EngineSettings {
        preset: settings.preset,
        dim: settings.dim,
        fps: settings.fps,
        gap: settings.gap,
        status: match settings.status {
            Status::Paused => EngineStatus::Paused,
            Status::Resumed => EngineStatus::Resumed,
        },
    }
}

#[wasm_bindgen(js_name = "engineAddOnChangeListener")]
pub fn main_add_on_change_listener(cb: JsValue) {
    if let Ok(func_to_exec) = cb.dyn_into::<Function>() {
        let this = JsValue::null();
        let _ = func_to_exec.call0(&this);
        let __ = func_to_exec.call0(&this);
        let ___ = func_to_exec.call0(&this);
    }
}
