use iced::{
    mouse, 
    widget::{
        canvas::{
            Frame, 
            Geometry, 
            Path, 
            Program, 
            Stroke
        }, Canvas
    }, 
    Color, 
    Length, 
    Point, 
    Rectangle, 
    Renderer, 
    Task, 
    Theme,
    Subscription,
    time::{
        self, 
        Duration, 
        Instant,
    },
};
use iced_graphics::geometry::{
        LineCap, 
        LineJoin,
    };

use crate::{
    app::*,
    utils::*,
};

static FADE_DURATION: u128 = 1500; // ms
static ACTION_GESTURE_DURATION: u128 = 250; // ms
static MIN_DISTANCE: f32 = 20.0; // pixels


#[derive(Debug, Clone)]
pub enum Message {
    Debug(String),
    Tick,
}

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
}

impl GestureHandler {
    pub fn new() -> Self {
        GestureHandler {
            history: Vec::new(),
            current_gesture: None,
            timer_enabled: false,
        }
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        info!("gesture update");
        Task::none()
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

    pub fn view(&self) -> Canvas<GestureView<'_>, main_app::Message> {
        Canvas::new(GestureView::new(self))
            .width(Length::Fill)
            .height(Length::Fill)
    }

    pub fn start(&mut self) {
        self.current_gesture = Some(Gesture {
            start_instant: Some(Instant::now()),
            end_instant: None,
            buffer: Vec::new(),
        });
    }

    pub fn end(&mut self) {
        if let Some(mut gesture) = self.current_gesture.take() {
            if gesture.buffer.is_empty() {
                return;
            }
            gesture.end_instant = Some(Instant::now());
            self.history.push(gesture.clone()); // clone to history

            match gesture.end_instant.unwrap().duration_since(gesture.start_instant.unwrap()).as_millis() {
                duration if duration < ACTION_GESTURE_DURATION => {
                    self.handle_action_gesture(gesture);
                }
                _ => {
                    self.handle_view_gesture(gesture);
                }
            }
        }
    }

    pub fn append(&mut self, position: Point) {
        self.update_history(); // todo: need appropriate spot for this? some kind of timer based update

        if let Some(gesture) = self.current_gesture.as_mut() {
            if gesture.buffer.len() > 1 {
                // distance check with the back/end item
                let prev = gesture.buffer.last().unwrap();
                let distance = Point::distance(&prev.point, position);
                if distance < MIN_DISTANCE {
                    return;
                }
            }

            gesture.buffer.push(GestureData {
                point: position,
                instant: Instant::now(),
            });
        }
    }

    fn handle_action_gesture(&mut self, gesture: Gesture) {
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

        let _ = Task::perform(async { main_app::Message::ActionGesture(direction) }, |result | result);
    }

    fn handle_view_gesture(&mut self, gesture: Gesture) {
        info!("view gesture");
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
            style: Color::from_rgba(0.6, 0.8, 1.0, 0.3).into(),
            width: 8.0,
            ..Default::default()
        });
        frame
    }

    /// Draw the gesture using the segment method.
    /// Create the path using a Builder closure
    /// create the line in reverse order
    fn draw_line_segment_method(&self, gesture: &Gesture, mut frame: Frame) -> Frame {
        //let gesture_data = self.gesture_data.clone();
        let max_width = 16u128; // Max initial width
        let max_opacity = 0.3; // Max initial opacity
        let now = Instant::now();
        let mut prev_point = gesture.buffer.last().unwrap().point;

        for (i, data) in gesture.buffer.iter().enumerate().rev().skip(1) {
            // draw curve
            let next_point = data.point;

            let path = Path::new(|builder| {
                builder.move_to(prev_point);
                builder.line_to(next_point);
            });

            prev_point = next_point;

            // apply styling to curve
            let time_elapsed = now.duration_since(data.instant).as_millis();
            if time_elapsed > FADE_DURATION {
                return frame;
            }

            //let progress = 1.0 - (time_elapsed / fade_duration);
            // Calculate fade progress using integer math for time_elapsed and fade_duration
            let progress = (FADE_DURATION - time_elapsed) as f32 / FADE_DURATION as f32;

            //let width = max_width * progress; // width narrows
            //let opacity = max_opacity * progress; // fade out
            // Calculate width and opacity based on progress
            let width = (max_width as f32 * progress).max(1.0); // Ensure width doesn't go below 1.0
            let opacity = (max_opacity * progress).max(0.0);   // Ensure opacity doesn't go below 0.0

            // debug
            //let opacity = 0.5;
            //let width = 16.0;

            frame.stroke(
                &path,
                Stroke {
                    style: Color::from_rgba(0.3, 0.1, 0.8, opacity).into(),
                    width,
                    ..Default::default()
                },
            );
        }
        frame
    }

    /// Draw the gesture using the segment method.
    /// Create the path using a Builder closure
    /// create the line in reverse order
    fn draw_segment_method(&self, gesture: &Gesture, mut frame: Frame) -> Frame {
        let max_width = 20u128; // Max initial width
        let max_opacity = 0.3; // Max initial opacity
        let now = Instant::now();
        let mut prev_point = gesture.buffer.last().unwrap().point;

        for (_i, data) in gesture.buffer.iter().enumerate().rev() {
            // draw curve
            let next_point = data.point;

            // Generate control points using Catmull-Rom (for smoother curves)
            let control_point1 = Point::new(
                prev_point.x + (next_point.x - prev_point.x) * 0.5, 
                prev_point.y + (next_point.y - prev_point.y) * 0.5
            );

            let control_point2 = Point::new(
                next_point.x + (next_point.x - prev_point.x) * 0.5, 
                next_point.y + (next_point.y - prev_point.y) * 0.5
            );

            let path = Path::new(|builder| {
                builder.move_to(prev_point);
                builder.bezier_curve_to(control_point1, control_point2, next_point);
            });

            prev_point = next_point;

            // apply styling to curve
            let time_elapsed = now.duration_since(data.instant).as_millis();
            if time_elapsed > FADE_DURATION {
                return frame;
            }

            // Calculate fade progress using integer math for time_elapsed and fade_duration
            // Calculate width and opacity based on progress
            let progress = (FADE_DURATION - time_elapsed) as f32 / FADE_DURATION as f32;
            let width = (max_width as f32 * progress).max(1.0); // Ensure width doesn't go below 1.0
            let opacity = (max_opacity * progress).max(0.0);   // Ensure opacity doesn't go below 0.0

            // debug
            //let opacity = 0.5;
            //let width = 16.0;

            frame.stroke(
                &path,
                Stroke {
                    style: Color::from_rgba(0.3, 0.1, 0.8, opacity).into(),
                    width,
                    line_cap: LineCap::Butt,
                    line_join: LineJoin::Bevel,
                    ..Default::default()
                },
            );
        }
        frame
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
        //info!("{}", self.gesture_data.len());
        let mut frame = Frame::new(renderer, bounds.size());

        // draw all gestures
        for gesture in self.handler.get_all_gestures().iter() {
            if gesture.buffer.len() > 1 {
                frame = self.draw_segment_method(gesture, frame);
            }
        }

        vec![frame.into_geometry()]
    }
}