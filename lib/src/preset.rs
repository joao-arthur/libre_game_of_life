use std::{collections::HashMap, sync::LazyLock};

use super::universe::{Universe, from_string};

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
            "boat".into(),
            from_string(vec![
                "⬛⬛⬛⬛⬛".into(),
                "⬛⬜⬜⬛⬛".into(),
                "⬛⬜⬛⬜⬛".into(),
                "⬛⬛⬜⬜⬛".into(),
                "⬛⬛⬛⬛⬛".into(),
            ])
            .unwrap(),
        ),
        (
            "block".into(),
            from_string(vec![
                "⬛⬛⬛⬛".into(),
                "⬛⬜⬜⬛".into(),
                "⬛⬜⬜⬛".into(),
                "⬛⬛⬛⬛".into(),
            ])
            .unwrap(),
        ),
        (
            "blinker".into(),
            from_string(vec![
                "⬛⬛⬛⬛⬛".into(),
                "⬛⬛⬜⬛⬛".into(),
                "⬛⬛⬜⬛⬛".into(),
                "⬛⬛⬜⬛⬛".into(),
                "⬛⬛⬛⬛⬛".into(),
            ])
            .unwrap(),
        ),
        (
            "r_pentomino".into(),
            from_string(vec![
                "⬛⬛⬛⬛⬛".into(),
                "⬛⬛⬜⬜⬛".into(),
                "⬛⬜⬜⬛⬛".into(),
                "⬛⬛⬜⬛⬛".into(),
                "⬛⬛⬛⬛⬛".into(),
            ])
            .unwrap(),
        ),
        (
            "glider".into(),
            from_string(vec![
                "⬛⬛⬛⬛⬛".into(),
                "⬛⬜⬛⬜⬛".into(),
                "⬛⬛⬜⬜⬛".into(),
                "⬛⬛⬜⬛⬛".into(),
                "⬛⬛⬛⬛⬛".into(),
            ])
            .unwrap(),
        ),
        (
            "gosper_glider_gun".into(),
            from_string(vec![
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".into(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".into(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".into(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".into(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".into(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".into(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".into(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".into(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".into(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".into(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".into(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".into(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".into(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".into(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".into(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬜⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".into(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬜⬛⬜⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".into(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬜⬜⬛⬛⬛⬛⬛⬛⬜⬜⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬜⬜⬛".into(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬜⬛⬛⬛⬜⬛⬛⬛⬛⬜⬜⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬜⬜⬛".into(),
                "⬛⬜⬜⬛⬛⬛⬛⬛⬛⬛⬛⬜⬛⬛⬛⬛⬛⬜⬛⬛⬛⬜⬜⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".into(),
                "⬛⬜⬜⬛⬛⬛⬛⬛⬛⬛⬛⬜⬛⬛⬛⬜⬛⬜⬜⬛⬛⬛⬛⬜⬛⬜⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".into(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬜⬛⬛⬛⬛⬛⬜⬛⬛⬛⬛⬛⬛⬛⬜⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".into(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬜⬛⬛⬛⬜⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".into(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬜⬜⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".into(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".into(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".into(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".into(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".into(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".into(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".into(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".into(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".into(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".into(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".into(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".into(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".into(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".into(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".into(),
            ])
            .unwrap(),
        ),
        (
            "puffer1".into(),
            from_string(vec![
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".into(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".into(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".into(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".into(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".into(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".into(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".into(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".into(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".into(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".into(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".into(),
                "⬛⬛⬜⬜⬜⬛⬛⬛⬛⬛⬛⬜⬛⬛⬛⬛⬛⬜⬛⬛⬛⬛⬛⬛⬜⬜⬜⬛⬛".into(),
                "⬛⬜⬛⬛⬜⬛⬛⬛⬛⬛⬜⬜⬜⬛⬛⬛⬜⬜⬜⬛⬛⬛⬛⬛⬜⬛⬛⬜⬛".into(),
                "⬛⬛⬛⬛⬜⬛⬛⬛⬛⬜⬜⬛⬜⬛⬛⬛⬜⬛⬜⬜⬛⬛⬛⬛⬜⬛⬛⬛⬛".into(),
                "⬛⬛⬛⬛⬜⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬜⬛⬛⬛⬛".into(),
                "⬛⬛⬛⬛⬜⬛⬛⬜⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬜⬛⬛⬜⬛⬛⬛⬛".into(),
                "⬛⬛⬛⬛⬜⬛⬛⬜⬜⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬜⬜⬛⬛⬜⬛⬛⬛⬛".into(),
                "⬛⬛⬛⬜⬛⬛⬛⬜⬜⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬜⬜⬛⬛⬛⬜⬛⬛⬛".into(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".into(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".into(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".into(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".into(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".into(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".into(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".into(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".into(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".into(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".into(),
                "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".into(),
            ])
            .unwrap(),
        ),
    ])
});

pub fn get_preset_unsafe(preset: &str) -> Universe {
    PRESETS.get(preset).cloned().unwrap()
}

pub fn get_preset(preset: &String) -> Option<Universe> {
    PRESETS.get(preset).cloned()
}

pub fn get_preset_groups() -> Vec<PresetGroup> {
    vec![
        PresetGroup {
            info: PresetGroupInfo { name: "StillLife".into(), id: "stillLife".into() },
            sub_groups: vec![
                PresetSubGroup { name: "Ship".into(), id: "ship".into(), items: vec![] },
                PresetSubGroup {
                    name: "Boat".into(),
                    id: "boat".into(),
                    items: vec![Preset {
                        name: "Boat".into(),
                        id: "boat".into(),
                        discover: PresetDiscover { name: "JHC group".into(), year: 1970 },
                    }],
                },
                PresetSubGroup { name: "Loaf".into(), id: "loaf".into(), items: vec![] },
                PresetSubGroup {
                    name: "General".into(),
                    id: "general".into(),
                    items: vec![Preset {
                        name: "Block".into(),
                        id: "block".into(),
                        discover: PresetDiscover { name: "John Conway".into(), year: 1969 },
                    }],
                },
            ],
        },
        PresetGroup {
            info: PresetGroupInfo { name: "Oscillators".into(), id: "oscillators".into() },
            sub_groups: vec![PresetSubGroup {
                name: "General".into(),
                id: "general".into(),
                items: vec![Preset {
                    name: "Blinker".into(),
                    id: "blinker".into(),
                    discover: PresetDiscover { name: "John Conway".into(), year: 1969 },
                }],
            }],
        },
        PresetGroup {
            info: PresetGroupInfo { name: "Methuselahs".into(), id: "methuselahs".into() },
            sub_groups: vec![PresetSubGroup {
                name: "General".into(),
                id: "general".into(),
                items: vec![Preset {
                    name: "R-Pentomino".into(),
                    id: "r_pentomino".into(),
                    discover: PresetDiscover { name: "John Conway".into(), year: 1969 },
                }],
            }],
        },
        PresetGroup {
            info: PresetGroupInfo { name: "Spaceships".into(), id: "spaceships".into() },
            sub_groups: vec![PresetSubGroup {
                name: "General".into(),
                id: "general".into(),
                items: vec![Preset {
                    name: "Glider".into(),
                    id: "glider".into(),
                    discover: PresetDiscover { name: "Richard K. Guy".into(), year: 1969 },
                }],
            }],
        },
        PresetGroup {
            info: PresetGroupInfo { name: "Glider gun".into(), id: "gliderGun".into() },
            sub_groups: vec![PresetSubGroup {
                name: "General".into(),
                id: "general".into(),
                items: vec![Preset {
                    name: "Gosper glider gun".into(),
                    id: "gosper_glider_gun".into(),
                    discover: PresetDiscover { name: "Bill Gosper".into(), year: 1970 },
                }],
            }],
        },
        PresetGroup {
            info: PresetGroupInfo { name: "Puffer".into(), id: "puffer".into() },
            sub_groups: vec![PresetSubGroup {
                name: "General".into(),
                id: "general".into(),
                items: vec![Preset {
                    name: "Puffer 1".into(),
                    id: "puffer1".into(),
                    discover: PresetDiscover { name: "Bill Gosper".into(), year: 1971 },
                }],
            }],
        },
    ]
}
