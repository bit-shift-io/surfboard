#![allow(dead_code)]

use iced::{
    advanced::graphics::{
            color,
            mesh::{self, Renderer as _, SolidVertex2D},
            Mesh,
        }, 
    Color, 
    Point, 
    Rectangle, 
    Renderer, 
    Transformation
};


/// Helper for drawing a vertex as a point
pub fn draw_vertex(renderer: &mut Renderer, vertex: SolidVertex2D) {
    draw_point(renderer, vertex.position.into(), vertex.color.components().into())
}


/// Draw a single point for debug
/// Edit frame in place by having the &mut on the type instead of the variable
pub fn draw_point(renderer: &mut Renderer, point: Point, color: Color) {
    let half_size = 5.0 * 0.5;
    let color = color::pack(color);
    let mesh = Mesh::Solid {
        buffers: mesh::Indexed {
            vertices: vec![
                SolidVertex2D { // top left
                    position: [point.x - half_size, point.y - half_size],
                    color,
                },
                SolidVertex2D { // bottom left
                    position: [point.x - half_size, point.y + half_size],
                    color,
                },
                SolidVertex2D { // bottom right
                    position: [point.x + half_size, point.y + half_size],
                    color,
                },
                SolidVertex2D { // top right
                    position: [point.x + half_size, point.y - half_size],
                    color,
                },
            ],
            indices: vec![
                0, 1, 2, // First triangle: Top-left, Bottom-left, Bottom-right
                0, 2, 3, // Second triangle: Top-left, Bottom-right, Top-right
            ],
        },
        transformation: Transformation::IDENTITY,
        clip_bounds: Rectangle {
            x: point.x - half_size,
            y: point.y - half_size,
            width: half_size * 2.0,
            height: half_size * 2.0,
        },
    };

    renderer.draw_mesh(mesh);
}