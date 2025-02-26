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
    }, 
    mouse::Cursor, 
    time::{
        Duration, 
        Instant,
    }, 
    Color, 
    Element, 
    Length, 
    Point, 
    Rectangle, 
    Renderer, 
    Size, 
    Task, 
    Theme, 
    Transformation
};
use crate::{
    app::*,
    utils::*,
};

static FADE_DURATION: u128 = 800; // ms
static ACTION_GESTURE_DURATION: u128 = 250; // ms
static MIN_DISTANCE: f32 = 15.0; // pixels
static MAX_WIDTH: f32 = 25.0; // Max initial width
static MAX_OPACITY: f32 = 0.5; // Max initial opacity
static COLOR: Color = Color::from_rgba(0.6, 0.8, 1.0, 1.0);

#[derive(Debug, Clone)]
pub enum Message {
    UpdateHistory,
}

/// Handles the gesture ribbon.  
#[derive(Clone, Debug)]
pub struct GestureHandler {
    pub history: Vec<Gesture>,
    pub current_gesture: Option<Gesture>,
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
        }
    }

    pub fn update(&mut self, message: Message) -> Task<main_app::Message> {
        match message {
            Message::UpdateHistory => {
                self.clear_history();
                Task::none()
            }
        }
    }

    pub fn get_all_gestures(&self) -> Vec<Gesture> {
        let mut result: Vec<Gesture> = self.history.iter().cloned().collect();
        if let Some(gesture) = self.current_gesture.as_ref() {
            result.push(gesture.clone());
        }
        result
    }

    pub fn clear_history(&mut self) {
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
        let return_task = Task::perform(async {
            async_std::task::sleep(Duration::from_millis(FADE_DURATION as u64 + 10)).await;
            main_app::Message::GestureHandler(Message::UpdateHistory)
        }, |result| result);

        if let Some(mut gesture) = self.current_gesture.take() {
            if gesture.buffer.is_empty() {
                return return_task
            }
            gesture.end_instant = Some(Instant::now());
            self.history.push(gesture.clone()); // clone to history

            match gesture.end_instant.unwrap().duration_since(gesture.start_instant.unwrap()).as_millis() {
                duration if duration < ACTION_GESTURE_DURATION => {
                    return Task::batch(vec![
                        self.handle_action_gesture(gesture),
                        return_task,
                    ]);
                }
                _ => {
                    return Task::batch(vec![
                        self.handle_view_gesture(gesture),
                        return_task,
                    ]);
                }
            }
        }
        return_task
    }

    pub fn update_move(&mut self, position: Point) -> Task<main_app::Message> {
        if self.current_gesture.is_none() {
            let _ = self.start();
        }

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
                    position.x - n2.point.x,
                    position.y - n2.point.y,
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
                    position.x - n1.point.x,
                    position.y - n1.point.y,
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

    fn handle_view_gesture(&mut self, _gesture: Gesture) -> Task<main_app::Message> {
        // todo dictionary etc... pass to view or actionbar view
        info!("view gesture");
        Task::none()
    }
}



pub struct MeshRibbon<'a> {
    handler: &'a GestureHandler,
}

impl<'a> MeshRibbon<'a> {
    pub fn new(handler: &'a GestureHandler) -> Self {
        MeshRibbon {
            handler,
        }
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

        if points.len() < 2 {
            return;
        }

        // generate verteces for the width of the ribbon
        let vertices = points
            .iter()
            .enumerate()
            .rev()
            .flat_map(|(_i, data)| {
                let time_elapsed = now.duration_since(data.instant).as_millis();
                let progress = (FADE_DURATION - time_elapsed) as f32 / FADE_DURATION as f32;
                let width = (MAX_WIDTH * progress).max(1.0); // Ensure width doesn't go below 1.0
                let opacity = (MAX_OPACITY * progress).max(0.0);   // Ensure opacity doesn't go below 0.0
                let color = color::pack(COLOR.scale_alpha(opacity));
                let half_normal = functions::multiply_point(data.normal, width * 0.5);
                let left = functions::add_point(data.point, half_normal);
                let right = functions::add_point(data.point, functions::invert_point(half_normal));
                [
                    SolidVertex2D {
                        position: left.into(),
                        color,
                    },
                    SolidVertex2D {
                        position: right.into(),
                        color,
                    },
                ]
            })
            .collect::<Vec<_>>();

            
        // the triangles
        let indices = (0..vertices.len() as u32 - 2)
            .flat_map(|i| [i, i + 1, i + 2])
            .collect();

        // the mesh
        let mesh = Mesh::Solid {
            buffers: mesh::Indexed { vertices, indices },
            transformation: Transformation::IDENTITY,
            clip_bounds: *viewport,
        };

        // draw the mesh
        renderer.draw_mesh(mesh);

        // // debug draw
        // for point in vertices {
        //     self.draw_vertex(renderer, point);
        // }

    }
}


impl<'a> Widget<Message, Theme, Renderer> for MeshRibbon<'a> {

    fn size(&self) -> Size<Length> {
        Size::new(Length::Fill, Length::Fill)
    }

    fn layout(&self, _tree: &mut Tree, _renderer: &Renderer, limits: &Limits) -> Node {
        Node::new(limits.max())
    }

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
    }
}
