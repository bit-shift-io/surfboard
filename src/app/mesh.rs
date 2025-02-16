use iced::{
    advanced::{
        graphics::{
            color,
            mesh::{self, Renderer as _, SolidVertex2D},
            Mesh,
        },
        layout::{Limits, Node},
        renderer::Style,
        widget::Tree,
        Layout, Widget,
    }, mouse::Cursor, Color, Length, Rectangle, Renderer, Size, Theme, Transformation
};

use super::main_app::Message;

pub struct Test;

impl Widget<Message, Theme, Renderer> for Test {
    fn size(&self) -> Size<Length> {
        Size::new(Length::Fill, Length::Fill)
    }

    fn layout(&self, _tree: &mut Tree, _renderer: &Renderer, limits: &Limits) -> Node {
        Node::new(limits.max())
    }

    // fn draw(
    //     &self,
    //     _tree: &Tree,
    //     renderer: &mut Renderer,
    //     _theme: &Theme,
    //     _style: &Style,
    //     _layout: Layout<'_>,
    //     _cursor: Cursor,
    //     _viewport: &Rectangle,
    // ) {
    //     renderer.draw_mesh(Mesh::Solid {
    //         buffers: mesh::Indexed {
    //             vertices: Vec::new(),
    //             indices: Vec::new(),
    //         },
    //         transformation: Transformation::IDENTITY,
    //         clip_bounds: Rectangle::INFINITE,
    //     });
    // }


    fn draw(
        &self,
        _tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        _style: &Style,
        _layout: Layout<'_>,
        _cursor: Cursor,
        _viewport: &Rectangle,
    ) {
        let mesh1 = Mesh::Solid {
            buffers: mesh::Indexed {
                vertices: vec![
                    SolidVertex2D {
                        position: [0.0, 0.0],
                        color: color::pack(Color::WHITE),

                    },
                    SolidVertex2D {
                        position: [0.0, 100.0],
                        color: color::pack(Color::WHITE),
                    },
                    SolidVertex2D {
                        position: [100.0, 100.0],
                        color: color::pack(Color::WHITE),
                    },
                ],
                indices: vec![0, 1, 2],
            },
            transformation: Transformation::IDENTITY,
            clip_bounds: Rectangle {
                x: 0.0,
                y: 0.0,
                width: 100.0,
                height: 0.9999999,
            },
        };

        let mesh2 = Mesh::Solid {
            buffers: mesh::Indexed {
                vertices: vec![
                    SolidVertex2D {
                        position: [0.0, 100.0],
                        color: color::pack(Color::WHITE),
                    },
                    SolidVertex2D {
                        position: [0.0, 200.0],
                        color: color::pack(Color::WHITE),
                    },
                    SolidVertex2D {
                        position: [100.0, 200.0],
                        color: color::pack(theme.extended_palette().secondary.base.text),
                    },
                ],
                indices: vec![0, 1, 2],
            },
            transformation: Transformation::IDENTITY,
            clip_bounds: Rectangle {
                x: 0.0,
                y: 100.0,
                width: 100.0,
                height: 100.0,
            },
        };

        // draw calls
        
        renderer.draw_mesh(mesh1);
        renderer.draw_mesh(mesh2);
        
    }
}