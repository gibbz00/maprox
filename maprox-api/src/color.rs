use serde::{Deserialize, Serialize};

/// Composants range: [0.0, 1.0]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct RgbaColor(pub f32, pub f32, pub f32, pub f32);
