use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use flatgeobuf::{FallibleStreamingIterator, FgbReader};
use std::{fs::File, io::BufReader};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}

// TEMP:
#[allow(dead_code)]
fn read_flat_geobuf_example() {
    let mut file_buffer = BufReader::new(File::open("countries.fgb").unwrap());
    let mut flatgeobuf_reader = FgbReader::open(&mut file_buffer)
        .unwrap()
        .select_all()
        .unwrap();
    while let Some(simple_feature) = flatgeobuf_reader.next().unwrap() {
        println!("{:#?}", simple_feature.fbs_feature())
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    // Circle
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(50.).into()).into(),
        material: materials.add(ColorMaterial::from(Color::PURPLE)),
        transform: Transform::from_translation(Vec3::new(-150., 0., 0.)),
        ..default()
    });

    // Quad
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes
            .add(shape::Quad::new(Vec2::new(50., 100.)).into())
            .into(),
        material: materials.add(ColorMaterial::from(Color::LIME_GREEN)),
        transform: Transform::from_translation(Vec3::new(50., 0., 0.)),
        ..default()
    });

    // Hexagon
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::RegularPolygon::new(50., 6).into()).into(),
        material: materials.add(ColorMaterial::from(Color::TURQUOISE)),
        transform: Transform::from_translation(Vec3::new(150., 0., 0.)),
        ..default()
    });
}
