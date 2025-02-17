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
        Layout, 
        Widget, 
    }, mouse::{self, Cursor}, time::{
        self, 
        Duration, 
        Instant,
    }, widget::{
        canvas::{
            Frame, 
            Geometry, 
            Path, 
            Program, 
            Stroke
        }, 
        Canvas
    }, 
    Color, 
    Element, 
    Length, 
    Point, 
    Rectangle, 
    Renderer, 
    Size, 
    Subscription, 
    Task, 
    Theme, 
    Transformation
};
use iced_graphics::{geometry::{
        LineCap, 
        LineJoin,
    }, mesh::Indexed};
use crate::{
    app::*,
    utils::*,
};

static FADE_DURATION: u128 = 800; // ms
static ACTION_GESTURE_DURATION: u128 = 250; // ms
static MIN_DISTANCE: f32 = 15.0; // pixels
static MAX_WIDTH: f32 = 25.0; // Max initial width
static MAX_OPACITY: f32 = 0.1; // Max initial opacity
static COLOR: Color = Color::from_rgba(0.6, 0.8, 1.0, 1.0);
static CONTROL_POINT_SCALE: f32 = 0.3;

#[derive(Debug, Clone)]
pub enum Message {
    Tick,
}

#[derive(Clone, Debug)]
pub struct GestureHandler {
    pub history: Vec<Gesture>,
    pub current_gesture: Option<Gesture>,
    pub timer_enabled: bool, // for animation to complete after gesture
}

#[derive(Debug, Clone, PartialEq)]
pub struct Gesture {
    pub start_instant: Option<Instant>,
    pub end_instant: Option<Instant>,
    pub buffer: Vec<GestureData>, // buffer to store gesture data
    // may want to store the type, left click, right click etc?
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GestureData {
    pub point: Point,
    pub instant: Instant,
    pub tangent: Point,
    pub normal: Point,
}


#[derive(Debug, Clone)]
pub enum ActionDirection {
    TopLeft,
    Top,
    TopRight,
    Right,
    BottomRight,
    Bottom,
    BottomLeft,
    Left,
    LongPress,
}

impl GestureHandler {
    pub fn new() -> Self {
        GestureHandler {
            history: Vec::new(),
            current_gesture: None,
            timer_enabled: false,
        }
    }

    pub fn update(&mut self, message: Message) -> Task<main_app::Message> {
        match message {
            Message::Tick => {
                //self.view(); // redraw view?
                info!("gesture tick");
                //self.update_history();
                //self.timer_enabled = false;
                Task::none()
            }
            _ => Task::none()
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        match self.timer_enabled {
            true => time::every(Duration::from_millis(100)).map(|_| Message::Tick), // every function not found in iced::time?!
            false => Subscription::none()
        }
    }

    pub fn get_all_gestures(&self) -> Vec<Gesture> {
        let mut result: Vec<Gesture> = self.history.iter().cloned().collect();
        if let Some(gesture) = self.current_gesture.as_ref() {
            result.push(gesture.clone());
        }
        result
    }

    pub fn update_history(&mut self) {
        // loop through history and remove expired gestures if end_instant is older than FADE_DURATION
        let now = Instant::now();
        self.history.retain(|gesture| {
            if let Some(end_instant) = gesture.end_instant {
                now.duration_since(end_instant).as_millis() < FADE_DURATION
            } else {
                true
            }
        });
    }

    pub fn view(&self) -> Element<main_app::Message> {
        iced::Element::new(MeshRibbon::new(self))
            .map(|message| main_app::Message::GestureHandler(message))
            .into()
        //MeshRibbon::new(&self).into()
        //Element::new()
        // iced::Element::new(MeshRibbon)
        // Canvas::new(GestureView::new(self))
        //     .width(Length::Fill)
        //     .height(Length::Fill)
        //     .into()
    }

    pub fn start(&mut self) -> Task<main_app::Message> {
        self.current_gesture = Some(Gesture {
            start_instant: Some(Instant::now()),
            end_instant: None,
            buffer: Vec::new(),
        });
        Task::none()
    }

    pub fn end(&mut self) -> Task<main_app::Message> {
        if let Some(mut gesture) = self.current_gesture.take() {
            if gesture.buffer.is_empty() {
                return Task::none()
            }
            gesture.end_instant = Some(Instant::now());
            self.history.push(gesture.clone()); // clone to history

            match gesture.end_instant.unwrap().duration_since(gesture.start_instant.unwrap()).as_millis() {
                duration if duration < ACTION_GESTURE_DURATION => {
                    return self.handle_action_gesture(gesture)
                }
                _ => {
                    return self.handle_view_gesture(gesture)
                }
            }
        }
        Task::none()
    }

    pub fn append(&mut self, position: Point) -> Task<main_app::Message> {
        //self.update_history(); // todo: need appropriate spot for this? some kind of timer based update

        if let Some(gesture) = self.current_gesture.as_mut() {
            let length = gesture.buffer.len();

            if length > 1 {
                // distance check with the back/end item
                let prev = gesture.buffer.last().unwrap();
                let distance = Point::distance(&prev.point, position);
                if distance < MIN_DISTANCE {
                    return Task::none()
                }
            }

            
            if length > 2 {
                // calc better tangent & normal of previous point
                let n2 = gesture.buffer[length - 2];
                gesture.buffer[length - 1].tangent = Point::new(
                    (position.x - n2.point.x) * CONTROL_POINT_SCALE,
                    (position.y - n2.point.y) * CONTROL_POINT_SCALE,
                );

                let normal = Point::new(
                    -(position.y - n2.point.y),
                    position.x - n2.point.x,
                );
                gesture.buffer[length - 1].normal = functions::normalize_point(normal); // normalized
            }


            let tangent = if length > 1 {
                // calc tangent & normal of the new point
                let n1 = gesture.buffer[length - 1];
                Point::new(
                    (position.x - n1.point.x) * CONTROL_POINT_SCALE,
                    (position.y - n1.point.y) * CONTROL_POINT_SCALE,
                ) // not normalised as we want to include the spacing between points
            } else {
                Point::new(0.0, 0.0)
            };

            
            let normal = if length > 1 {
                // calc the normal vector with the previous point
                let n1 = gesture.buffer[length - 1];
                let normal = Point::new(
                    -(position.y - n1.point.y),
                    position.x - n1.point.x,
                );
                functions::normalize_point(normal) // normalized
            } else {
                Point::new(0.0, 0.0)
            };

            // update this point
            gesture.buffer.push(GestureData {
                point: position,
                instant: Instant::now(),
                tangent,
                normal,
            });
        }
        Task::none()
    }

    fn handle_action_gesture(&mut self, gesture: Gesture) -> Task<main_app::Message> {
        let start = gesture.buffer.first().unwrap().point;
        let end = gesture.buffer.last().unwrap().point;
        let angle = functions::calculate_angle_degrees(start, end);
        let normalized_angle = (angle + 90.0).rem_euclid(360.0); // adjust and normalize to 0-360 range

        // weighted direction with 50-degree ranges for 45-degree angles
        let direction = match normalized_angle {
            x if x < 20.0 || x >= 340.0 => ActionDirection::Top,
            x if x < 70.0 => ActionDirection::TopRight,
            x if x < 110.0 => ActionDirection::Right,
            x if x < 160.0 => ActionDirection::BottomRight,
            x if x < 200.0 => ActionDirection::Bottom,
            x if x < 250.0 => ActionDirection::BottomLeft,
            x if x < 290.0 => ActionDirection::Left,
            _ => ActionDirection::TopLeft,
        };
        Task::done(view::Message::ActionGesture(direction)).map(main_app::Message::ViewHandler)
    }

    fn handle_view_gesture(&mut self, gesture: Gesture) -> Task<main_app::Message> {
        // todo dictionary etc... pass to view or actionbar view
        info!("view gesture");
        Task::none()
    }
}


pub struct GestureView<'a> {
    handler: &'a GestureHandler,
}

impl<'a> GestureView<'a> {
    pub fn new(handler: &'a GestureHandler) -> Self {
        GestureView {
            handler,
        }
    }

    fn draw_single_line_method(&self, gesture: &Gesture, mut frame: Frame) -> Frame {
        let path = Path::new(|builder| {
            builder.move_to(gesture.buffer.last().unwrap().point);
            let mut prev_point = gesture.buffer.last().unwrap().point;
            // quadratic_curve_to
            for data in gesture.buffer.iter().rev().skip(1) {
                let control_point = Point::new(
                    (prev_point.x + data.point.x) / 2.0,
                    (prev_point.y + data.point.y) / 2.0,
                );
                builder.quadratic_curve_to(control_point, data.point);
                prev_point = data.point;
            }
        });

        frame.stroke(
        &path,
        Stroke {
            style: COLOR.scale_alpha(0.5).into(),
            width: 8.0,
            ..Default::default()
        });
        frame
    }

    /// Draw the gesture using the segment method.
    /// Create the path using a Builder closure
    /// create the line in reverse order
    fn draw_line_segment_method(&self, gesture: &Gesture, mut frame: Frame) -> Frame {
        let now = Instant::now();
        let mut prev_point = gesture.buffer.last().unwrap().point;

        for (i, data) in gesture.buffer.iter().enumerate().rev() {
            // return if gesture is older than FADE_DURATION
            let time_elapsed = now.duration_since(data.instant).as_millis();
            if time_elapsed > FADE_DURATION {
                return frame;
            }

            // draw curve
            let next_point = data.point;

            let path = Path::new(|builder| {
                builder.move_to(prev_point);
                builder.line_to(next_point);
            });

            prev_point = next_point;

            self.segment_style(&mut frame, &path, time_elapsed);
        }
        frame
    }


    /// Draw the gesture using the segment method.
    /// Create the path using a Builder closure
    /// create the line in reverse order
    fn draw_segment_method(&self, gesture: &Gesture, mut frame: Frame) -> Frame {
        // points are all stored in gesture.buffer, which is a Vector of GestureData {Point, Instant}
        let now = Instant::now();
        let mut prev_point = gesture.buffer.last().unwrap().point;
        let mut prev_tangent = gesture.buffer.last().unwrap().tangent;

        for (_i, data) in gesture.buffer.iter().enumerate().rev() {
            // return if gesture is older than FADE_DURATION
            let time_elapsed = now.duration_since(data.instant).as_millis();
            if time_elapsed > FADE_DURATION {
                return frame;
            }

            // prev_point & prev_control
            // current_point & current_control

            // draw curve
            let current_point = data.point;
            let current_tangent = data.tangent;
            //let next_point = if i > 0 { gesture.buffer[i - 1].point } else { current_point };

            // Calculate control points using the previous tangent and the current tangent
            let prev_control = Point::new(
                prev_point.x - prev_tangent.x, // * CONTROL_POINT_PERCENT,
                prev_point.y - prev_tangent.y, // * CONTROL_POINT_PERCENT,
            );

            // calculate new tangent for the end of the segment
            let current_control = Point::new(
                current_point.x + current_tangent.x, // * CONTROL_POINT_PERCENT,
                current_point.y + current_tangent.y, // * CONTROL_POINT_PERCENT,
            );

            //info!("{}, {}, {} | {}, {}, {}", prev_point, prev_tangent, prev_control, current_point, current_tangent, current_control);

            let path = Path::new(|builder| {
                builder.move_to(prev_point); // first point
                builder.bezier_curve_to(prev_control, current_control, current_point);
            });

            prev_point = current_point;
            prev_tangent = current_tangent;

            self.segment_style(&mut frame, &path, time_elapsed);
        }
        frame
    }


    pub fn segment_style(&self, frame: &mut Frame, path: &Path, time_elapsed: u128) {
          // Calculate fade progress using integer math for time_elapsed and fade_duration
        // Calculate width and opacity based on progress
        let progress = (FADE_DURATION - time_elapsed) as f32 / FADE_DURATION as f32;
        let width = (MAX_WIDTH * progress).max(1.0); // Ensure width doesn't go below 1.0
        let opacity = (MAX_OPACITY * progress).max(0.0);   // Ensure opacity doesn't go below 0.0

        frame.stroke(
            &path,
            Stroke {
                style: COLOR.scale_alpha(opacity).into(),
                width,
                line_cap: LineCap::Butt,
                line_join: LineJoin::Miter,
                ..Default::default()
            },
        );
    }

    /// Draw a single point for debug
    /// Edit frame in place by having the &mut on the type instead of the variable
    fn draw_point(&self, frame: &mut Frame, point: Point) {
        // We create a `Path` representing a simple circle
        let circle = Path::circle(point, 2.0);
        // And fill it with some color
        frame.fill(&circle, Color::WHITE);
    }

    /// Draw the gesture using the mesh method.
    fn draw_mesh_method(&self, gesture: &Gesture, mut frame: &mut Frame) {
        // points are all stored in gesture.buffer, which is a Vector of GestureData {Point, Instant}
        let now = Instant::now();
        let mut prev_point = gesture.buffer.last().unwrap().point;
        let mut prev_normal = gesture.buffer.last().unwrap().normal;
        let mut prev_tangent = gesture.buffer.last().unwrap().tangent;

        for (_i, data) in gesture.buffer.iter().enumerate().rev() {
            // return if gesture is older than FADE_DURATION
            let time_elapsed = now.duration_since(data.instant).as_millis();
            if time_elapsed > FADE_DURATION {
                return;
            }

            // draw curve
            let current_point = data.point;
            let current_tangent = data.tangent;
            let current_normal = data.normal;

            // draw mesh from previous to the current point
   
            // Calculate control points using the previous tangent and the current tangent
            let prev_control = Point::new(
                prev_point.x - prev_tangent.x, // * CONTROL_POINT_PERCENT,
                prev_point.y - prev_tangent.y, // * CONTROL_POINT_PERCENT,
            );

            // calculate new tangent for the end of the segment
            let current_control = Point::new(
                current_point.x + current_tangent.x, // * CONTROL_POINT_PERCENT,
                current_point.y + current_tangent.y, // * CONTROL_POINT_PERCENT,
            );

            // debug
            //info!("{}, {} | {}, {}", prev_point, prev_normal, current_point, current_normal);
            self.draw_point(&mut frame, current_point);


            let path = Path::new(|builder| {
                builder.move_to(prev_point); // first point
                builder.bezier_curve_to(prev_control, current_control, current_point);
            });

            prev_point = current_point;
            prev_tangent = current_tangent;
            prev_normal = current_normal;

            self.segment_style(&mut frame, &path, time_elapsed);
        }
    }


}

impl<'a, Message> Program<Message> for GestureView<'a> {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        //info!("gesture draw");
        let mut frame = Frame::new(renderer, bounds.size());


        // draw all gestures
        for gesture in self.handler.get_all_gestures().iter() {
            if gesture.buffer.len() > 1 {
                self.draw_mesh_method(gesture, &mut frame);
            }
        }

        vec![frame.into_geometry()]
    }
}




// https://github.com/generic-daw/generic-daw/blob/main/generic_daw_gui/src/widget/audio_clip.rs


pub struct MeshRibbon<'a> {
    handler: &'a GestureHandler,
}

impl<'a> MeshRibbon<'a> {
    pub fn new(handler: &'a GestureHandler) -> Self {
        MeshRibbon {
            handler,
        }
    }

    /// Draw a single point for debug
    /// Edit frame in place by having the &mut on the type instead of the variable
    fn draw_point(&self, renderer: &mut Renderer, point: Point) {
        let half_size = 5.0 * 0.5;
        let color = color::pack(Color::from_rgba(1.0, 0.0, 0.0, 0.5));
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

    pub fn draw_mesh(&self, gesture: &Gesture, renderer: &mut Renderer, viewport: &Rectangle) {
        // points are all stored in gesture.buffer, which is a Vector of GestureData {Point, Instant}
        let now = Instant::now();

        // collect all points that are younger than fade duration
        // also do the reverse here
        let points: Vec<_> = gesture.buffer
            .iter()
            .filter(|data| {
                let time_elapsed = now.duration_since(data.instant).as_millis();
                time_elapsed <= FADE_DURATION
            })
            .rev()
            .collect();

        // generate verteces for the width of the ribbon
        let vertices = points
            .iter()
            .enumerate()
            .rev()
            .flat_map(|(i, data)| {
                let time_elapsed = now.duration_since(data.instant).as_millis();
                let progress = (FADE_DURATION - time_elapsed) as f32 / FADE_DURATION as f32;
                let width = (MAX_WIDTH * progress).max(1.0); // Ensure width doesn't go below 1.0
                let opacity = (MAX_OPACITY * progress).max(0.0);   // Ensure opacity doesn't go below 0.0
                let color = color::pack(COLOR.scale_alpha(opacity));
                [
                    SolidVertex2D {
                        position: [1.0, 1.0],
                        color,
                    },
                    SolidVertex2D {
                        position: [1.0, 1.0],
                        color,
                    },
                ]
            })
            .collect::<Vec<_>>();

            
        // if vertices.len() < 3 {
        //     // the triangles
        //     let indices = (0..vertices.len() as u32 - 2)
        //     .flat_map(|i| [i, i + 1, i + 2])
        //     .collect();

        //     // the mesh
        //     let mesh = Mesh::Solid {
        //     buffers: Indexed { vertices, indices },
        //     transformation: Transformation::IDENTITY,
        //     clip_bounds: *viewport,
        //     };

        //     // draw the mesh
        //     renderer.draw_mesh(mesh);
        // }
        

        // debug draw
        for data in points {
            self.draw_point(renderer, data.point);
        }

    }
}


impl<'a> Widget<Message, Theme, Renderer> for MeshRibbon<'a> {

    fn size(&self) -> Size<Length> {
        Size::new(Length::Fill, Length::Fill)
    }

    fn layout(&self, _tree: &mut Tree, _renderer: &Renderer, limits: &Limits) -> Node {
        Node::new(limits.max())
    }

    // https://github.com/generic-daw/generic-daw/blob/main/generic_daw_gui/src/widget/audio_clip.rs

    fn draw(
        &self,
        _tree: &Tree,
        renderer: &mut Renderer,
        _theme: &Theme,
        _style: &Style,
        _layout: Layout<'_>,
        _cursor: Cursor,
        viewport: &Rectangle,
    ) {
        // draw all gestures
        for gesture in self.handler.get_all_gestures().iter() {
            if gesture.buffer.len() > 1 {
                self.draw_mesh(gesture, renderer, viewport);
            }
        }

        // let mesh2 = Mesh::Solid {
        //     buffers: mesh::Indexed {
        //         vertices: vec![
        //             SolidVertex2D {
        //                 position: [0.0, 100.0],
        //                 color: color::pack(Color::WHITE),
        //             },
        //             SolidVertex2D {
        //                 position: [0.0, 200.0],
        //                 color: color::pack(Color::WHITE),
        //             },
        //             SolidVertex2D {
        //                 position: [100.0, 200.0],
        //                 color: color::pack(theme.extended_palette().secondary.base.text),
        //             },


        //             SolidVertex2D {
        //                 position: [100.0, 100.0],
        //                 color: color::pack(theme.extended_palette().secondary.base.text),
        //             },
        //         ],
        //         indices: vec![
        //             0, 1, 2, // First triangle: Top-left, Bottom-left, Bottom-right
        //             0, 2, 3, // Second triangle: Top-left, Bottom-right, Top-right
        //         ],
        //     },
        //     transformation: Transformation::IDENTITY,
        //     clip_bounds: Rectangle {
        //         x: 0.0,
        //         y: 100.0,
        //         width: 100.0,
        //         height: 100.0,
        //     },
        // };

        // renderer.draw_mesh(mesh2);
        
    }
}
