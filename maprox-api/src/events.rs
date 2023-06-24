use crate::color::RgbaColor;
use geo_types::Geometry;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub enum Event {
    // TEMP: Used for prototyping
    Increment,
    RefreshColors,
    RenderGeometry((Geometry, RgbaColor)),
}
