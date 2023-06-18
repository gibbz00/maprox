use geo_types::Geometry;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub enum Event {
    Increment,
    RenderGeometry(Geometry),
    RefreshColors,
}
