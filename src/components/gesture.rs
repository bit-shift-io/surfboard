use core::time;
use std::collections::VecDeque;

use iced::{
    mouse, time::Instant, widget::{canvas::{
        Frame, 
        Geometry, 
        Path, 
        Program, 
        Stroke}, text::Fragment}, Color, Gradient, Point, Rectangle, Renderer, Theme
};
use iced_graphics::{geometry::{frame::{self, Backend}, LineCap, LineJoin}, text::cosmic_text::ttf_parser::feat::FeatureName};


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GestureData {
    pub point: Point,
    pub instant: Instant,
}

pub struct Gesture<'a> {
    gesture_data: &'a VecDeque<GestureData>,
}

impl<'a> Gesture<'a> {
    pub fn new(gesture_data: &'a VecDeque<GestureData>) -> Self {
        Gesture {
            gesture_data,
        }
    }


    fn draw_single_line_method(&self, mut frame: Frame) -> Frame {
        let path = Path::new(|builder| {
            builder.move_to(self.gesture_data.back().unwrap().point);
            let mut prev_point = self.gesture_data.back().unwrap().point;
            // quadratic_curve_to
            for data in self.gesture_data.iter().rev().skip(1) {
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
    fn draw_line_segment_method(&self, mut frame: Frame) -> Frame {
        let gesture_data = self.gesture_data.clone();
        let max_width = 16u128; // Max initial width
        let max_opacity = 0.3; // Max initial opacity
        let fade_duration = 1000u128; // ms duration for fading
        let now = Instant::now();
        let mut prev_point = gesture_data.back().unwrap().point;

        for (i, data) in gesture_data.iter().enumerate().rev().skip(1) {
            // draw curve
            let next_point = data.point;

            let path = Path::new(|builder| {
                builder.move_to(prev_point);
                builder.line_to(next_point);
            });

            prev_point = next_point;

            // apply styling to curve
            let time_elapsed = now.duration_since(data.instant).as_millis();
            if time_elapsed > fade_duration {
                return frame;
            }

            //let progress = 1.0 - (time_elapsed / fade_duration);
            // Calculate fade progress using integer math for time_elapsed and fade_duration
            let progress = (fade_duration - time_elapsed) as f32 / fade_duration as f32;

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
    fn draw_segment_method(&self, mut frame: Frame) -> Frame {
        let max_width = 16u128; // Max initial width
        let max_opacity = 0.3; // Max initial opacity
        let fade_duration = 1000u128; // ms duration for fading
        let now = Instant::now();
        let mut prev_point = self.gesture_data.back().unwrap().point;

        for (i, data) in self.gesture_data.iter().enumerate().rev().skip(1) {
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
            // let time_elapsed = now.duration_since(data.instant).as_millis();
            // if time_elapsed > fade_duration {
            //     return frame;
            // }

            // Calculate fade progress using integer math for time_elapsed and fade_duration
            // Calculate width and opacity based on progress
            // let progress = (fade_duration - time_elapsed) as f32 / fade_duration as f32;
            // let width = (max_width as f32 * progress).max(1.0); // Ensure width doesn't go below 1.0
            // let opacity = (max_opacity * progress).max(0.0);   // Ensure opacity doesn't go below 0.0

            // debug
            let opacity = 0.5;
            let width = 16.0;

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

impl<'a, Message> Program<Message> for Gesture<'a> {
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
        let frame = Frame::new(renderer, bounds.size());
        if self.gesture_data.len() > 4 {
            return vec![self.draw_segment_method(frame).into_geometry()]
        }
        vec![frame.into_geometry()]
    }

    
}
