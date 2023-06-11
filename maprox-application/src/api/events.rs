use bevy::prelude::*;
use geo_bevy::{build_bevy_meshes, PreparedMesh};
use geo_types::Geometry;

pub fn refresh_colors(
    query: &mut Query<&mut Handle<StandardMaterial>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let color = Color::rgb(fastrand::f32(), fastrand::f32(), fastrand::f32());
    for mut material in query.iter_mut() {
        *material = materials.add(color.into());
    }
}

pub fn render_geometry(
    geometry: Geometry,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let prepared_meshes = build_bevy_meshes(
        &geometry,
        Color::rgb(fastrand::f32(), fastrand::f32(), fastrand::f32()),
    )
    .unwrap();

    for prepared_mesh in prepared_meshes {
        match prepared_mesh {
            PreparedMesh::LineString { mesh, color } => {
                commands.spawn(MaterialMeshBundle {
                    mesh: meshes.add(mesh),
                    material: materials.add(color.into()),
                    ..default()
                });
            }
            PreparedMesh::Polygon { mesh, color } => {
                commands.spawn(MaterialMeshBundle {
                    mesh: meshes.add(mesh),
                    material: materials.add(color.into()),
                    ..default()
                });
            }
            PreparedMesh::Point(_) => {
                panic!("Drawing points from .fgb hasn't yet been implemented.")
            }
        }
    }
}
