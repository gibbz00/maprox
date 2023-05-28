use bevy::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use flatgeobuf::{FallibleStreamingIterator, FgbReader};
use geo_bevy::{build_bevy_meshes, PreparedMesh};
use geozero::ToGeo;
use std::{f32::consts::PI, fs::File, io::BufReader};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PanOrbitCameraPlugin)
        .add_startup_system(read_flat_geobuf_example)
        .add_startup_system(camera)
        .add_system(clamp_zoom)
        .run();
}

fn read_flat_geobuf_example(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut file_buffer = BufReader::new(File::open("countries.fgb").unwrap());
    let mut flatgeobuf_reader = FgbReader::open(&mut file_buffer)
        .unwrap()
        .select_all()
        .unwrap();

    while let Some(simple_feature) = flatgeobuf_reader.next().unwrap() {
        if let Ok(geometry) = simple_feature.to_geo() {
            let prepared_meshes = build_bevy_meshes(
                &geometry,
                Color::rgb(fastrand::f32(), fastrand::f32(), fastrand::f32()),
            )
            .unwrap();

            for prepared_mesh in prepared_meshes {
                match prepared_mesh {
                    PreparedMesh::LineString { mesh, color } => {
                        commands.spawn(PbrBundle {
                            mesh: meshes.add(mesh),
                            material: materials.add(color.into()),
                            ..default()
                        });
                    }
                    PreparedMesh::Polygon { mesh, color } => {
                        commands.spawn(PbrBundle {
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
    }
}

fn camera(mut commands: Commands) {
    const ORBIT_CLAMP: f32 = (PI / 2.) * 0.95;
    commands.spawn((
        Camera3dBundle::default(),
        PanOrbitCamera {
            alpha_lower_limit: Some(-ORBIT_CLAMP),
            alpha_upper_limit: Some(ORBIT_CLAMP),
            beta_lower_limit: Some(-ORBIT_CLAMP),
            beta_upper_limit: Some(ORBIT_CLAMP),
            ..Default::default()
        },
    ));
}

fn clamp_zoom(mut orbit_cameras: Query<&mut PanOrbitCamera>) {
    for mut orbit_camera in orbit_cameras.iter_mut() {
        orbit_camera.radius = f32::min(orbit_camera.radius, 200.0)
    }
}
