use bevy::prelude::*;
use geo_bevy::{build_bevy_meshes, PreparedMesh};
use geo_types::Geometry;

fn gen_random_color_material(
    materials: &mut ResMut<Assets<StandardMaterial>>,
) -> Handle<StandardMaterial> {
    materials.add(StandardMaterial {
        unlit: true,
        ..Color::rgb(fastrand::f32(), fastrand::f32(), fastrand::f32()).into()
    })
}

pub fn refresh_colors(
    query: &mut Query<&mut Handle<StandardMaterial>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    for mut material in query.iter_mut() {
        *material = gen_random_color_material(materials);
    }
}

pub fn render_geometry(
    geometry: Geometry,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    // TEMP: color's should really be passed through the mesh builder.
    let prepared_meshes = build_bevy_meshes(&geometry, Color::WHITE).unwrap();

    for prepared_mesh in prepared_meshes {
        match prepared_mesh {
            PreparedMesh::LineString { mesh, .. } => {
                commands.spawn(MaterialMeshBundle {
                    mesh: meshes.add(mesh),
                    material: gen_random_color_material(materials),
                    ..default()
                });
            }
            PreparedMesh::Polygon { mesh, .. } => {
                commands.spawn(MaterialMeshBundle {
                    mesh: meshes.add(mesh),
                    material: gen_random_color_material(materials),
                    ..default()
                });
            }
            PreparedMesh::Point(_) => {
                panic!("Drawing points from .fgb hasn't yet been implemented.")
            }
        }
    }
}
