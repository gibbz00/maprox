use crate::color::RgbaColor;
use geo_types::{
    Geometry, GeometryCollection, Line, LineString, MultiLineString, MultiPoint, MultiPolygon,
    Point, Polygon, Rect,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub enum Event {
    // TEMP: Used for prototyping
    Increment,
    RefreshColors,

    RenderPoint((Point, RgbaColor)),
    RenderMultiPoint((MultiPoint, RgbaColor)),
    RenderLine((Line, RgbaColor)),
    RenderLineString((LineString, RgbaColor)),
    RenderMultiLineString((MultiLineString, RgbaColor)),
    RenderPolygon((Polygon, RgbaColor)),
    RenderMultiPolygon((MultiPolygon, RgbaColor)),
    RenderRect((Rect, RgbaColor)),
    RenderGeometry((Geometry, RgbaColor)),
    RenderGeometryCollection((GeometryCollection, RgbaColor)),
}
