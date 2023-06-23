use bevy::prelude::*;
use geo_bevy::*;
use geo_types::*;
use maprox_api::color::RgbaColor;

use crate::api::BevySpawnStructs;

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

pub fn render_point(point: Point, color: &RgbaColor, bevy_spawn_structs: &mut BevySpawnStructs) {
    bevy_spawn_structs.commands.spawn(MaterialMeshBundle {
        // TODO: assign value based on viewport size
        mesh: bevy_spawn_structs
            .meshes
            .add(shape::Circle::new(0.5).into()),
        material: unlit_standard_material_handle(color, &mut bevy_spawn_structs.materials),
        transform: Transform::from_translation(Vec3::new(point.x() as f32, point.y() as f32, 0.)),
        ..default()
    });
}

pub fn render_multi_point(
    multi_point: MultiPoint,
    color: &RgbaColor,
    bevy_spawn_structs: &mut BevySpawnStructs,
) {
    for point in multi_point {
        render_point(point, color, bevy_spawn_structs)
    }
}

pub fn render_line(line: Line, color: &RgbaColor, bevy_spawn_structs: &mut BevySpawnStructs) {
    if let Ok(Some(line_mesh)) = line_to_mesh(&line) {
        spawn_mesh(line_mesh, color, bevy_spawn_structs)
    }
}

pub fn render_line_string(
    line_string: LineString,
    color: &RgbaColor,
    bevy_spawn_structs: &mut BevySpawnStructs,
) {
    if let Ok(Some(line_string_mesh)) = line_string_to_mesh(&line_string) {
        spawn_mesh(line_string_mesh, color, bevy_spawn_structs)
    }
}

pub fn render_multi_linestring(
    multi_linestring: MultiLineString,
    color: &RgbaColor,
    bevy_spawn_structs: &mut BevySpawnStructs,
) {
    if let Ok(line_string_meshes) = multi_line_string_to_mesh(&multi_linestring) {
        for line_string_mesh in line_string_meshes {
            spawn_mesh(line_string_mesh, color, bevy_spawn_structs);
        }
    }
}

pub fn render_polygon(
    polygon: Polygon,
    color: &RgbaColor,
    bevy_spawn_structs: &mut BevySpawnStructs,
) {
    if let Ok(Some(polygon_mesh)) = polygon_to_mesh(&polygon) {
        spawn_polygon_mesh(polygon_mesh, color, bevy_spawn_structs)
    }
}

pub fn render_multi_polygon(
    multi_polygon: MultiPolygon,
    color: &RgbaColor,
    bevy_spawn_structs: &mut BevySpawnStructs,
) {
    if let Ok(polygon_meshes) = multi_polygon_to_mesh(&multi_polygon) {
        for polygon_mesh in polygon_meshes {
            spawn_polygon_mesh(polygon_mesh, color, bevy_spawn_structs);
        }
    }
}

pub fn render_rect(
    rect: geo_types::Rect,
    color: &RgbaColor,
    bevy_spawn_structs: &mut BevySpawnStructs,
) {
    if let Ok(Some(polygon_mesh)) = rect_to_mesh(&rect) {
        spawn_polygon_mesh(polygon_mesh, color, bevy_spawn_structs)
    }
}

pub fn render_geometry(
    geometry: Geometry,
    color: &RgbaColor,
    bevy_spawn_structs: &mut BevySpawnStructs,
) {
    let color_material_handle =
        unlit_standard_material_handle(color, &mut bevy_spawn_structs.materials);
    // TEMP: unwrap
    match geometry_to_mesh(&geometry).unwrap().unwrap() {
        GeometryMesh::LineString(mesh) => {
            bevy_spawn_structs.commands.spawn(MaterialMeshBundle {
                mesh: bevy_spawn_structs.meshes.add(mesh),
                material: color_material_handle,
                ..default()
            });
        }
        GeometryMesh::Polygon(polygon_mesh) => {
            bevy_spawn_structs.commands.spawn(MaterialMeshBundle {
                mesh: bevy_spawn_structs.meshes.add(polygon_mesh.mesh),
                material: color_material_handle,
                ..default()
            });
        }
        GeometryMesh::Point(points) => {
            for point in points {
                render_point(point, color, bevy_spawn_structs)
            }
        }
    }
}

pub fn render_geometry_collection(
    geometry_collection: GeometryCollection,
    color: &RgbaColor,
    bevy_spawn_structs: &mut BevySpawnStructs,
) {
    for geometry in geometry_collection {
        render_geometry(geometry, color, bevy_spawn_structs);
    }
}

fn spawn_mesh(mesh: Mesh, color: &RgbaColor, bevy_spawn_structs: &mut BevySpawnStructs) {
    bevy_spawn_structs.commands.spawn(MaterialMeshBundle {
        mesh: bevy_spawn_structs.meshes.add(mesh),
        material: unlit_standard_material_handle(color, &mut bevy_spawn_structs.materials),
        ..default()
    });
}

fn spawn_polygon_mesh(
    polygon_mesh: PolygonMesh,
    color: &RgbaColor,
    bevy_spawn_structs: &mut BevySpawnStructs,
) {
    bevy_spawn_structs.commands.spawn(MaterialMeshBundle {
        mesh: bevy_spawn_structs.meshes.add(polygon_mesh.mesh),
        material: unlit_standard_material_handle(color, &mut bevy_spawn_structs.materials),
        ..default()
    });
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
