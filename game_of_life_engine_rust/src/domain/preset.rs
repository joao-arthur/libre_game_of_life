use std::{collections::HashMap, sync::LazyLock};

use super::universe::{from_string, Universe};

#[derive(Clone)]
pub struct PresetDiscover {
    pub name: String,
    pub year: u16,
}

#[derive(Clone)]
pub struct Preset {
    pub name: String,
    pub id: String,
    pub discover: PresetDiscover,
}

#[derive(Clone)]
pub struct PresetSubGroup {
    pub name: String,
    pub id: String,
    pub items: Vec<Preset>,
}

pub struct PresetGroupInfo {
    pub name: String,
    pub id: String,
}

pub struct PresetGroup {
    pub info: PresetGroupInfo,
    pub sub_groups: Vec<PresetSubGroup>,
}

static PRESETS: LazyLock<HashMap<String, Universe>> = LazyLock::new(|| {
    HashMap::from([
        (
            "boat".to_string(),
            from_string(vec![
                "⬛⬛⬛⬛⬛".to_string(),
                "⬛⬜⬜⬛⬛".to_string(),
                "⬛⬜⬛⬜⬛".to_string(),
                "⬛⬛⬜⬜⬛".to_string(),
                "⬛⬛⬛⬛⬛".to_string(),
            ])
            .unwrap(),
        ),
        (
            "block".to_string(),
            from_string(vec![
                "⬛⬛⬛⬛".to_string(),
                "⬛⬜⬜⬛".to_string(),
                "⬛⬜⬜⬛".to_string(),
                "⬛⬛⬛⬛".to_string(),
            ])
            .unwrap(),
        ),
        (
            "blinker".to_string(),
            from_string(vec![
                "⬛⬛⬛⬛⬛".to_string(),
                "⬛⬛⬜⬛⬛".to_string(),
                "⬛⬛⬜⬛⬛".to_string(),
                "⬛⬛⬜⬛⬛".to_string(),
                "⬛⬛⬛⬛⬛".to_string(),
            ])
            .unwrap(),
        ),
        (
            "r_pentomino".to_string(),
            from_string(vec![
                "⬛⬛⬛⬛⬛".to_string(),
                "⬛⬛⬜⬜⬛".to_string(),
                "⬛⬜⬜⬛⬛".to_string(),
                "⬛⬛⬜⬛⬛".to_string(),
                "⬛⬛⬛⬛⬛".to_string(),
            ])
            .unwrap(),
        ),
        (
            "glider".to_string(),
            from_string(vec![
                "⬛⬛⬛⬛⬛".to_string(),
                "⬛⬜⬛⬜⬛".to_string(),
                "⬛⬛⬜⬜⬛".to_string(),
                "⬛⬛⬜⬛⬛".to_string(),
                "⬛⬛⬛⬛⬛".to_string(),
            ])
            .unwrap(),
        ),
        (
            "gosper_glider_gun".to_string(),
            from_string(vec![
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".to_string(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".to_string(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".to_string(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".to_string(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".to_string(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".to_string(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".to_string(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".to_string(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".to_string(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".to_string(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".to_string(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".to_string(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".to_string(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".to_string(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".to_string(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬜⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".to_string(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬜⬛⬜⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".to_string(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬜⬜⬛⬛⬛⬛⬛⬛⬜⬜⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬜⬜⬛".to_string(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬜⬛⬛⬛⬜⬛⬛⬛⬛⬜⬜⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬜⬜⬛".to_string(),
                "⬛⬜⬜⬛⬛⬛⬛⬛⬛⬛⬛⬜⬛⬛⬛⬛⬛⬜⬛⬛⬛⬜⬜⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".to_string(),
                "⬛⬜⬜⬛⬛⬛⬛⬛⬛⬛⬛⬜⬛⬛⬛⬜⬛⬜⬜⬛⬛⬛⬛⬜⬛⬜⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".to_string(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬜⬛⬛⬛⬛⬛⬜⬛⬛⬛⬛⬛⬛⬛⬜⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".to_string(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬜⬛⬛⬛⬜⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".to_string(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬜⬜⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".to_string(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".to_string(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".to_string(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".to_string(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".to_string(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".to_string(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".to_string(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".to_string(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".to_string(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".to_string(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".to_string(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".to_string(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".to_string(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".to_string(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".to_string(),
            ])
            .unwrap(),
        ),
        (
            "puffer1".to_string(),
            from_string(vec![
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".to_string(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".to_string(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".to_string(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".to_string(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".to_string(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".to_string(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".to_string(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".to_string(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".to_string(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".to_string(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".to_string(),
                "⬛⬛⬜⬜⬜⬛⬛⬛⬛⬛⬛⬜⬛⬛⬛⬛⬛⬜⬛⬛⬛⬛⬛⬛⬜⬜⬜⬛⬛".to_string(),
                "⬛⬜⬛⬛⬜⬛⬛⬛⬛⬛⬜⬜⬜⬛⬛⬛⬜⬜⬜⬛⬛⬛⬛⬛⬜⬛⬛⬜⬛".to_string(),
                "⬛⬛⬛⬛⬜⬛⬛⬛⬛⬜⬜⬛⬜⬛⬛⬛⬜⬛⬜⬜⬛⬛⬛⬛⬜⬛⬛⬛⬛".to_string(),
                "⬛⬛⬛⬛⬜⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬜⬛⬛⬛⬛".to_string(),
                "⬛⬛⬛⬛⬜⬛⬛⬜⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬜⬛⬛⬜⬛⬛⬛⬛".to_string(),
                "⬛⬛⬛⬛⬜⬛⬛⬜⬜⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬜⬜⬛⬛⬜⬛⬛⬛⬛".to_string(),
                "⬛⬛⬛⬜⬛⬛⬛⬜⬜⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬜⬜⬛⬛⬛⬜⬛⬛⬛".to_string(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".to_string(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".to_string(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".to_string(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".to_string(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".to_string(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".to_string(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".to_string(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".to_string(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".to_string(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".to_string(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".to_string(),
            ])
            .unwrap(),
        ),
    ])
});

pub fn get_preset_unsafe(preset: &str) -> Universe {
    PRESETS.get(preset).unwrap().clone()
}

pub fn get_preset(preset: &String) -> Option<Universe> {
    match PRESETS.get(preset) {
        None => None,
        Some(u) => Some(u.clone()),
    }
}

pub fn get_preset_groups() -> Vec<PresetGroup> {
    vec![
        PresetGroup {
            info: PresetGroupInfo {
                name: "StillLife".to_string(),
                id: "stillLife".to_string(),
            },
            sub_groups: vec![
                PresetSubGroup {
                    name: "Ship".to_string(),
                    id: "ship".to_string(),
                    items: vec![],
                },
                PresetSubGroup {
                    name: "Boat".to_string(),
                    id: "boat".to_string(),
                    items: vec![Preset {
                        name: "Boat".to_string(),
                        id: "boat".to_string(),
                        discover: PresetDiscover {
                            name: "JHC group".to_string(),
                            year: 1970,
                        },
                    }],
                },
                PresetSubGroup {
                    name: "Loaf".to_string(),
                    id: "loaf".to_string(),
                    items: vec![],
                },
                PresetSubGroup {
                    name: "General".to_string(),
                    id: "general".to_string(),
                    items: vec![Preset {
                        name: "Block".to_string(),
                        id: "block".to_string(),
                        discover: PresetDiscover {
                            name: "John Conway".to_string(),
                            year: 1969,
                        },
                    }],
                },
            ],
        },
        PresetGroup {
            info: PresetGroupInfo {
                name: "Oscillators".to_string(),
                id: "oscillators".to_string(),
            },
            sub_groups: vec![PresetSubGroup {
                name: "General".to_string(),
                id: "general".to_string(),
                items: vec![Preset {
                    name: "Blinker".to_string(),
                    id: "blinker".to_string(),
                    discover: PresetDiscover {
                        name: "John Conway".to_string(),
                        year: 1969,
                    },
                }],
            }],
        },
        PresetGroup {
            info: PresetGroupInfo {
                name: "Methuselahs".to_string(),
                id: "methuselahs".to_string(),
            },
            sub_groups: vec![PresetSubGroup {
                name: "General".to_string(),
                id: "general".to_string(),
                items: vec![Preset {
                    name: "R-Pentomino".to_string(),
                    id: "r_pentomino".to_string(),
                    discover: PresetDiscover {
                        name: "John Conway".to_string(),
                        year: 1969,
                    },
                }],
            }],
        },
        PresetGroup {
            info: PresetGroupInfo {
                name: "Spaceships".to_string(),
                id: "spaceships".to_string(),
            },
            sub_groups: vec![PresetSubGroup {
                name: "General".to_string(),
                id: "general".to_string(),
                items: vec![Preset {
                    name: "Glider".to_string(),
                    id: "glider".to_string(),
                    discover: PresetDiscover {
                        name: "Richard K. Guy".to_string(),
                        year: 1969,
                    },
                }],
            }],
        },
        PresetGroup {
            info: PresetGroupInfo {
                name: "Glider gun".to_string(),
                id: "gliderGun".to_string(),
            },
            sub_groups: vec![PresetSubGroup {
                name: "General".to_string(),
                id: "general".to_string(),
                items: vec![Preset {
                    name: "Gosper glider gun".to_string(),
                    id: "gosper_glider_gun".to_string(),
                    discover: PresetDiscover {
                        name: "Bill Gosper".to_string(),
                        year: 1970,
                    },
                }],
            }],
        },
        PresetGroup {
            info: PresetGroupInfo {
                name: "Puffer".to_string(),
                id: "puffer".to_string(),
            },
            sub_groups: vec![PresetSubGroup {
                name: "General".to_string(),
                id: "general".to_string(),
                items: vec![Preset {
                    name: "Puffer 1".to_string(),
                    id: "puffer1".to_string(),
                    discover: PresetDiscover {
                        name: "Bill Gosper".to_string(),
                        year: 1971,
                    },
                }],
            }],
        },
    ]
}
