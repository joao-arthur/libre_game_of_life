pub type Cam = manfredo::cartesian::rect::rect_i32::Rect;
pub type Dim = u16;
pub type Gap = u8;

#[derive(Debug, PartialEq, Clone)]
pub struct RenderSettings {
    pub cam: Cam,
    pub dim: Dim,
    pub gap: Gap,
}
