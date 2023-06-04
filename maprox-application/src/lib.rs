mod connection;

use bevy::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use connection::ConnectionPlugin;
use flatgeobuf::{FallibleStreamingIterator, FgbReader};
use geo_bevy::{build_bevy_meshes, PreparedMesh};
use geozero::ToGeo;
use std::{f32::consts::PI, fs::File, io::BufReader};

pub fn run() {
    let mut app = App::new();

    app.insert_resource(ClearColor(Color::WHITE))
        .add_plugins(DefaultPlugins)
        .add_plugin(PanOrbitCameraPlugin)
        .add_plugin(ConnectionPlugin)
        .add_startup_system(camera)
        .add_startup_system(scene_3d_example)
        // .add_startup_system(read_flat_geobuf_example)
        .add_system(clamp_zoom)
        .run();
}

fn scene_3d_example(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(5.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
    // cube
    commands.spawn((PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    },));
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}

// fn read_flat_geobuf_example(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
// ) {
//     let mut file_buffer =
// BufReader::new(File::open("countries.fgb").unwrap());     let mut
// flatgeobuf_reader = FgbReader::open(&mut file_buffer)         .unwrap()
//         .select_all()
//         .unwrap();

//     while let Some(simple_feature) = flatgeobuf_reader.next().unwrap() {
//         if let Ok(geometry) = simple_feature.to_geo() {
//             let prepared_meshes = build_bevy_meshes(
//                 &geometry,
//                 Color::rgb(fastrand::f32(), fastrand::f32(),
// fastrand::f32()),             )
//             .unwrap();

//             for prepared_mesh in prepared_meshes {
//                 match prepared_mesh {
//                     PreparedMesh::LineString { mesh, color } => {
//                         commands.spawn(MaterialMeshBundle {
//                             mesh: meshes.add(mesh),
//                             material: materials.add(color.into()),
//                             ..default()
//                         });
//                     }
//                     PreparedMesh::Polygon { mesh, color } => {
//                         commands.spawn(MaterialMeshBundle {
//                             mesh: meshes.add(mesh),
//                             material: materials.add(color.into()),
//                             ..default()
//                         });
//                     }
//                     PreparedMesh::Point(_) => {
//                         panic!("Drawing points from .fgb hasn't yet been
// implemented.")                     }
//                 }
//             }
//         }
//     }
// }

fn camera(mut commands: Commands) {
    const ORBIT_CLAMP: f32 = (PI / 2.) * 0.95;
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 50.0)),
            ..default()
        },
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
        orbit_camera.radius = Some(f32::min(
            orbit_camera.radius.expect("Initialized value"),
            200.0,
        ))
    }
}
