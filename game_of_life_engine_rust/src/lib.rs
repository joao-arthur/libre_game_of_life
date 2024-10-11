pub mod domain;

use domain::{
    cartesian_plane::{point_to_index, Point},
    model::{
        from_string, get_cell_size, get_length, get_middle_cell, iterate, toggle_cell, zoom, Model,
    },
    preset::get_preset_groups,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[wasm_bindgen]
pub fn getModelCellSize(model: &Model, total_size: i64) -> i64 {
    get_cell_size(model, total_size)
}

#[wasm_bindgen]
pub fn getModelLength(model: &Model) -> i64 {
    get_length(model)
}

#[wasm_bindgen]
pub fn getModelMiddleCell(model: &Model, total_size: i64) -> Point {
    get_middle_cell(model, total_size)
}

#[wasm_bindgen]
pub fn iterateModel(model: &mut Model) {
    iterate(model)
}

#[wasm_bindgen]
pub fn modelFromString(model_as_str: Vec<String>) -> Model {
    from_string(model_as_str)
}

#[wasm_bindgen]
pub fn moveModel() {
    //
}

#[wasm_bindgen]
pub fn pointToIndex(point: Point, length: i64) -> ArrPos {
    point_to_index(point, length)
}

#[wasm_bindgen]
pub fn getGresetGroups() {
    get_preset_groups()
}

#[wasm_bindgen]
pub fn toggleModel(model: &mut Model, point: Point) {
    toggle_cell(model, point);
}

#[wasm_bindgen]
pub fn zoomModel(model: Model, new_size: i64) -> Model {
    //
    zoom(model, new_size)
}
