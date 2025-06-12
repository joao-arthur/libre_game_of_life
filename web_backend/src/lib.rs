pub mod app;

use crate::app::{
    Status, add_on_change_listener, app_get_settings, app_init, app_move_cam, app_pause,
    app_resume, app_set_dimension, app_set_fps, app_set_gap, app_set_preset, app_single_iteration,
    app_toggle_model_cell_by_absolute_point, app_zoom_in, app_zoom_out, app_zoom_to,
};
use libre_game_of_life_lib::{
    geometry::coordinate::{CartesianPoint, MatrixPoint},
    preset::get_preset_groups,
};

use js_sys::Function;
use serde::Serialize;
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

#[wasm_bindgen]
#[derive(Clone)]
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
#[derive(Clone)]
pub struct EngineMatrixPoint {
    pub row: u64,
    pub col: u64,
}

#[wasm_bindgen]
impl EngineMatrixPoint {
    #[wasm_bindgen(constructor)]
    pub fn new(row: u64, col: u64) -> EngineMatrixPoint {
        EngineMatrixPoint { row, col }
    }
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum EngineStatus {
    Resumed,
    Paused,
}

#[wasm_bindgen]
pub struct EngineInfo {
    preset: Option<String>,
    pub gap: u8,
    pub size: u16,
    pub fps: u16,
    pub status: EngineStatus,
    pub age: u64,
}

#[wasm_bindgen]
impl EngineInfo {
    #[wasm_bindgen(getter)]
    pub fn preset(&self) -> Option<String> {
        self.preset.clone()
    }
}

#[derive(Serialize)]
pub struct EnginePresetInfo {
    pub id: String,
    pub name: String,
}

#[derive(Serialize)]
pub struct EnginePresetGroup {
    pub info: EnginePresetInfo,
    pub items: Vec<EnginePresetInfo>,
}

#[wasm_bindgen(js_name = "engineInit")]
pub fn main_init(value: CanvasRenderingContext2d) {
    app_init(value);
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
pub fn main_set_gap(gap: u8) {
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
pub fn main_toggle(point: EngineMatrixPoint) {
    app_toggle_model_cell_by_absolute_point(MatrixPoint { row: point.row, col: point.col });
}

#[wasm_bindgen(js_name = "engineZoomIn")]
pub fn main_zoom_in() {
    app_zoom_in();
}

#[wasm_bindgen(js_name = "engineZoomOut")]
pub fn main_zoom_out() {
    app_zoom_out();
}

#[wasm_bindgen(js_name = "engineZoomTo")]
pub fn main_zoom_by(new_size: u16) {
    app_zoom_to(new_size);
}

#[wasm_bindgen(js_name = "engineMoveBy")]
pub fn main_move_model(delta: EngineCartesianPoint) {
    app_move_cam(CartesianPoint { x: delta.x, y: delta.y });
}

#[wasm_bindgen(js_name = "engineGetSettings")]
pub fn main_get_settings() -> EngineInfo {
    let settings = app_get_settings();
    EngineInfo {
        preset: settings.preset,
        size: settings.size,
        fps: settings.fps,
        gap: settings.gap,
        status: match settings.status {
            Status::Paused => EngineStatus::Paused,
            Status::Resumed => EngineStatus::Resumed,
        },
        age: settings.age,
    }
}

#[wasm_bindgen(js_name = "engineAddOnChangeListener")]
pub fn main_add_on_change_listener(cb: Function) {
    add_on_change_listener(move |_| {
        cb.call0(&JsValue::null()).unwrap();
    });
}

#[wasm_bindgen(js_name = "engineGetPresets")]
pub fn main_get_presets() -> JsValue {
    let groups: Vec<EnginePresetGroup> = get_preset_groups()
        .iter()
        .map(|g| EnginePresetGroup {
            info: EnginePresetInfo { id: g.info.id.clone(), name: g.info.name.clone() },
            items: g
                .sub_groups
                .iter()
                .flat_map(|sub_group| {
                    sub_group
                        .items
                        .iter()
                        .map(|i| EnginePresetInfo { id: i.id.clone(), name: i.name.clone() })
                        .collect::<Vec<EnginePresetInfo>>()
                })
                .collect(),
        })
        .collect();
    serde_wasm_bindgen::to_value(&groups).unwrap()
}
