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

    pub fn view(&self) -> Canvas<GestureView<'_>, main_app::Message> {
        Canvas::new(GestureView::new(self))
            .width(Length::Fill)
            .height(Length::Fill)
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

            // calc tangent vector of previous point
            if length > 2 {
                let n2 = gesture.buffer[length - 2];
                let tangent = Point::new(
                    (position.x - n2.point.x) * CONTROL_POINT_SCALE,
                    (position.y - n2.point.y) * CONTROL_POINT_SCALE,
                );
                gesture.buffer[length - 1].tangent = tangent;
            }

            // update this point
            gesture.buffer.push(GestureData {
                point: position,
                instant: Instant::now(),
                tangent: Point::new(0.0, 0.0),
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

            frame = self.segment_style(frame, &path, time_elapsed);
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

            frame = self.segment_style(frame, &path, time_elapsed);
        }
        frame
    }


    pub fn segment_style(&self, mut frame: Frame, path: &Path, time_elapsed: u128) -> Frame {
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
        //info!("gesture draw");
        let mut frame = Frame::new(renderer, bounds.size());

        // draw all gestures
        for gesture in self.handler.get_all_gestures().iter() {
            if gesture.buffer.len() > 2 {
                frame = self.draw_segment_method(gesture, frame);
            }
        }

        vec![frame.into_geometry()]
    }
}