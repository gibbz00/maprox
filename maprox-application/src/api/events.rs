pub mod render_geometries;

use bevy::prelude::*;
use maprox_api::color::RgbaColor;

pub struct BevySpawnStructs<'w, 's> {
    pub commands: Commands<'w, 's>,
    pub meshes: ResMut<'w, Assets<Mesh>>,
    pub materials: ResMut<'w, Assets<StandardMaterial>>,
}

// TEMP: was only used for protyping
pub fn refresh_colors(
    query: &mut Query<&mut Handle<StandardMaterial>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    for mut material in query.iter_mut() {
        *material = unlit_standard_material_handle(
            &RgbaColor(fastrand::f32(), fastrand::f32(), fastrand::f32(), 1.),
            materials,
        );
    }
}

fn unlit_standard_material_handle(
    color: &RgbaColor,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) -> Handle<StandardMaterial> {
    let RgbaColor(r, g, b, a) = color;
    materials.add(StandardMaterial {
        unlit: true,
        ..Color::rgba(*r, *g, *b, *a).into()
    })
}
