pub mod domain;

use domain::{
    cartesian_plane::{absolute_to_relative, index_to_point, point_to_index, ArrPos, Point},
    model::{
        get_cell_size, get_length, get_middle_cell, get_middle_point, iterate, move_in_plane,
        toggle_cell, zoom, Model,
    },
    preset::get_preset_groups,
};
use wasm_bindgen::prelude::*;

fn js_value_to_model(value: JsValue) -> Result<Model, String> {
    serde_wasm_bindgen::from_value(value).map_err(|e| e.to_string())?
}

fn model_to_js_value(model: Model) -> Result<JsValue, String> {
    serde_wasm_bindgen::to_value(&model).map_err(|e| e.to_string())
}

#[wasm_bindgen(js_name = "getModelCellSize")]
pub fn external_get_model_cell_size(model: JsValue, total_size: i64) -> Result<i64, String> {
    let mut model_des = js_value_to_model(model)?;
    Ok(get_cell_size(&mut model_des, total_size))
}

#[wasm_bindgen(js_name = "getModelLength")]
pub fn external_get_model_length(model: JsValue) -> Result<i64, String> {
    let model_des = js_value_to_model(model)?;
    Ok(get_length(&model_des))
}

#[wasm_bindgen(js_name = "getModelMiddleCell")]
pub fn external_get_model_niddle_cell(model: JsValue, total_size: i64) -> Result<Point, String> {
    let model_des = js_value_to_model(model)?;
    let middle_cell = get_middle_cell(&model_des, total_size);
    Ok(Point {
        x: middle_cell.x,
        y: middle_cell.y,
    })
}

#[wasm_bindgen(js_name = "iterateModel")]
pub fn external_iterate_model(model: JsValue) -> Result<JsValue, String> {
    let mut model_des = js_value_to_model(model)?;
    iterate(&mut model_des);
    model_to_js_value(model_des)
}

#[wasm_bindgen(js_name = "moveModel")]
pub fn external_move_model(model: JsValue, delta: Point) -> Result<JsValue, String> {
    let mut model_des = js_value_to_model(model)?;
    move_in_plane(&mut model_des, delta);
    model_to_js_value(model_des)
}

#[wasm_bindgen(js_name = "pointToIndex")]
pub fn external_point_to_index(point: Point, length: i64) -> ArrPos {
    point_to_index(point, length)
}

#[wasm_bindgen(js_name = "getPresetGroups")]
pub fn external_get_preset_groups() -> Vec<JsValue> {
    get_preset_groups()
        .iter()
        .map(|val| serde_wasm_bindgen::to_value(&val).unwrap())
        .collect()
}

#[wasm_bindgen(js_name = "toggleModel")]
pub fn external_toggle_model(model: JsValue, point: Point) -> Result<JsValue, String> {
    let mut model_des = js_value_to_model(model)?;
    toggle_cell(&mut model_des, point);
    model_to_js_value(model_des)
}

#[wasm_bindgen(js_name = "zoomModel")]
pub fn external_zoom_model(model: JsValue, new_size: i64) -> Result<JsValue, String> {
    let mut model_des = js_value_to_model(model)?;
    zoom(&mut model_des, new_size);
    model_to_js_value(model_des)
}

#[wasm_bindgen(js_name = "absoluteToRelative")]
pub fn external_absolute_to_relative(value: i64, unit_size: i64) -> i64 {
    absolute_to_relative(value, unit_size)
}

#[wasm_bindgen(js_name = "getModelMiddlePoint")]
pub fn external_get_model_middle_point(model: JsValue) -> Result<Point, String> {
    let mut model_des = js_value_to_model(model)?;
    Ok(get_middle_point(&mut model_des))
}

#[wasm_bindgen(js_name = "indexToPoint")]
pub fn external_index_to_point(position: ArrPos, length: i64) -> Point {
    index_to_point(position, length)
}
