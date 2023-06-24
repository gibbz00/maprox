use super::{unlit_standard_material_handle, BevySpawnStructs};
use bevy::prelude::*;
use geo_bevy::*;
use geo_types::*;
use maprox_api::color::RgbaColor;

pub fn render_geometry(
    geometry: Geometry,
    color: &RgbaColor,
    bevy_spawn_structs: &mut BevySpawnStructs,
) {
    match geometry {
        Geometry::Point(point) => render_point(point, color, bevy_spawn_structs),
        Geometry::MultiPoint(multi_point) => {
            render_multi_point(multi_point, color, bevy_spawn_structs)
        }
        Geometry::Line(line) => render_line(line, color, bevy_spawn_structs),
        Geometry::LineString(line_string) => {
            render_line_string(line_string, color, bevy_spawn_structs)
        }
        Geometry::MultiLineString(multi_linestring) => {
            render_multi_linestring(multi_linestring, color, bevy_spawn_structs)
        }
        Geometry::Polygon(polygon) => render_polygon(polygon, color, bevy_spawn_structs),
        Geometry::MultiPolygon(multi_polygon) => {
            render_multi_polygon(multi_polygon, color, bevy_spawn_structs)
        }
        Geometry::Rect(rect) => render_rect(rect, color, bevy_spawn_structs),
        Geometry::Triangle(triangle) => render_triangle(triangle, color, bevy_spawn_structs),
        Geometry::GeometryCollection(geometry_collection) => {
            render_geometry_collection(geometry_collection, color, bevy_spawn_structs)
        }
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
    if let Some(line_mesh) = line_to_mesh(&line) {
        spawn_mesh(line_mesh, color, bevy_spawn_structs)
    }
}

pub fn render_line_string(
    line_string: LineString,
    color: &RgbaColor,
    bevy_spawn_structs: &mut BevySpawnStructs,
) {
    if let Some(line_string_mesh) = line_string_to_mesh(&line_string) {
        spawn_mesh(line_string_mesh, color, bevy_spawn_structs)
    }
}

pub fn render_multi_linestring(
    multi_linestring: MultiLineString,
    color: &RgbaColor,
    bevy_spawn_structs: &mut BevySpawnStructs,
) {
    for line_string_mesh in multi_line_string_to_mesh(&multi_linestring) {
        spawn_mesh(line_string_mesh, color, bevy_spawn_structs);
    }
}

pub fn render_polygon(
    polygon: Polygon,
    color: &RgbaColor,
    bevy_spawn_structs: &mut BevySpawnStructs,
) {
    if let Some(polygon_mesh) = polygon_to_mesh(&polygon) {
        spawn_polygon_mesh(polygon_mesh, color, bevy_spawn_structs)
    }
}

pub fn render_multi_polygon(
    multi_polygon: MultiPolygon,
    color: &RgbaColor,
    bevy_spawn_structs: &mut BevySpawnStructs,
) {
    for polygon_mesh in multi_polygon_to_mesh(&multi_polygon) {
        spawn_polygon_mesh(polygon_mesh, color, bevy_spawn_structs);
    }
}

pub fn render_rect(
    rect: geo_types::Rect,
    color: &RgbaColor,
    bevy_spawn_structs: &mut BevySpawnStructs,
) {
    if let Some(polygon_mesh) = rect_to_mesh(&rect) {
        spawn_polygon_mesh(polygon_mesh, color, bevy_spawn_structs)
    }
}

pub fn render_triangle(
    triangle: Triangle,
    color: &RgbaColor,
    bevy_spawn_structs: &mut BevySpawnStructs,
) {
    if let Some(polygon_mesh) = triangle_to_mesh(&triangle) {
        spawn_polygon_mesh(polygon_mesh, color, bevy_spawn_structs)
    }
}

pub fn render_geometry_collection(
    geometry_collection: GeometryCollection,
    color: &RgbaColor,
    bevy_spawn_structs: &mut BevySpawnStructs,
) {
    let color_material_handle =
        unlit_standard_material_handle(color, &mut bevy_spawn_structs.materials);

    for geometry_mesh in geometry_collection_to_mesh(&geometry_collection) {
        match geometry_mesh {
            GeometryMesh::LineString(mesh) => {
                bevy_spawn_structs.commands.spawn(MaterialMeshBundle {
                    mesh: bevy_spawn_structs.meshes.add(mesh),
                    material: color_material_handle.clone(),
                    ..default()
                });
            }
            GeometryMesh::Polygon(polygon_mesh) => {
                bevy_spawn_structs.commands.spawn(MaterialMeshBundle {
                    mesh: bevy_spawn_structs.meshes.add(polygon_mesh.mesh),
                    material: color_material_handle.clone(),
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
