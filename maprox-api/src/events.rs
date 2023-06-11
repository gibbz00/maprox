use geo_types::Geometry;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub enum Event {
    Increment,
    RenderGeometry(Geometry),
    RefreshColors,
}
