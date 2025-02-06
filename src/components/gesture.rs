use std::collections::VecDeque;
use iced::{
    mouse, 
    time::Instant,
    widget::{
        Canvas,
        canvas::{
            Frame, 
            Geometry, 
            Path, 
            Program, 
            Stroke
        },
    }, 
    Color, 
    Point, 
    Rectangle, 
    Renderer, 
    Theme,
    Length,
};
use iced_graphics::geometry::{
        LineCap, 
        LineJoin,
    };

use crate::app::*;

static FADE_DURATION: u128 = 1500; // ms


pub struct GestureHandler {
    pub history: VecDeque<Gesture>,
    pub current_gesture: Option<Gesture>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Gesture {
    pub start_instant: Option<Instant>,
    pub end_instant: Option<Instant>,
    pub buffer: VecDeque<GestureData>, // buffer to store gesture data
    // may want to store the type, left click, right click etc?
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GestureData {
    pub point: Point,
    pub instant: Instant,
}

impl GestureHandler {
    pub fn new() -> Self {
        GestureHandler {
            history: VecDeque::new(),
            current_gesture: None,
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

    pub fn view(&self) -> Canvas<GestureView<'_>, MainMessage> {
        Canvas::new(GestureView::new(self))
        .width(Length::Fill)
        .height(Length::Fill)
    }

    pub fn begin(&mut self) {
        

        self.current_gesture = Some(Gesture {
            start_instant: Some(Instant::now()),
            end_instant: None,
            buffer: VecDeque::new(),
        });
    }

    pub fn end(&mut self) {
        if let Some(mut gesture) = self.current_gesture.take() {
            gesture.end_instant = Some(Instant::now());

            // todo: process gesture


            // move to history
            self.history.push_back(gesture);
        }
    }

    pub fn append(&mut self, position: Point) {
        self.update_history(); // todo: need appropriate spot for this? some kind of timer based update

        if let Some(gesture) = self.current_gesture.as_mut() {
            // debug print out the points
            // info!("\nGesture Data:");
            // self.gesture_data.iter().for_each(|item| info!("{:?}", item));

            if gesture.buffer.len() > 1 {
                // distance check with the back/end item
                let prev = gesture.buffer.back().unwrap();
                let distance = Point::distance(&prev.point, position);
                if distance < 20.0 { // spacing between points, make static
                    return;
                }
                
                // time check
                // remove the front items
                while let Some(item) = gesture.buffer.front() {
                    let elapsed = Instant::now().duration_since(item.instant);
                    if elapsed.as_millis() > 1000 { // 2 seconds
                        gesture.buffer.pop_front();
                    } else {
                        break;
                    }
                }

            }

            // round off the position not needed
            //let point = Point::new(position.x.round(), position.y.round());

            // add data to the back
            gesture.buffer.push_back(GestureData {
                point: position,
                instant: Instant::now(),
            });
        }
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
            builder.move_to(gesture.buffer.back().unwrap().point);
            let mut prev_point = gesture.buffer.back().unwrap().point;
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
        let mut prev_point = gesture.buffer.back().unwrap().point;

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
            let opacity = 0.5;
            let width = 16.0;

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
        let max_width = 16u128; // Max initial width
        let max_opacity = 0.3; // Max initial opacity
        let now = Instant::now();
        let mut prev_point = gesture.buffer.back().unwrap().point;

        for (i, data) in gesture.buffer.iter().enumerate().rev().skip(1) {
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