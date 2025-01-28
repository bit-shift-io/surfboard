use core::time;

use iced::{
    mouse, time::Instant, widget::{canvas::{
        Frame, 
        Geometry, 
        Path, 
        Program, 
        Stroke}, text::Fragment}, Color, Point, Rectangle, Renderer, Theme
};
use iced_graphics::{geometry::frame::{self, Backend}, text::cosmic_text::ttf_parser::feat::FeatureName};


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GestureData {
    pub position: Point,
    pub time: Instant,
}

pub struct Gesture<'a> {
    gesture_data: &'a Vec<GestureData>,
}

impl<'a> Gesture<'a> {
    pub fn new(points: &'a Vec<GestureData>) -> Self {
        Gesture {
            gesture_data: points,
        }
    }


    fn draw_single_line_method(&self, mut frame: Frame) -> Frame {
        let path = Path::new(|builder| {
            builder.move_to(self.gesture_data.last().unwrap().position);
            let mut prev_point = self.gesture_data.last().unwrap().position;
            // quadratic_curve_to
            for data in self.gesture_data.iter().rev().skip(1).step_by(4) {
                let control_point = Point::new(
                    (prev_point.x + data.position.x) / 2.0,
                    (prev_point.y + data.position.y) / 2.0,
                );
                builder.quadratic_curve_to(control_point, data.position);
                prev_point = data.position;
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
    fn draw_segment_method(&self, mut frame: Frame) -> Frame {
        let max_width = 16f32; // Max initial width
        let max_opacity = 0.3; // Max initial opacity
        let fade_duration = 1000.0; // ms duration for fading
        let now = Instant::now();
        let mut prev_point = self.gesture_data.last().unwrap().position;

        for (i, data) in self.gesture_data.iter().enumerate().rev().skip(1).step_by(4) {
            // draw curve
            let control_point1 = Point::new(
                (prev_point.x + data.position.x) / 2.0,
                (prev_point.y + data.position.y) / 2.0,
            );
            let control_point2 = Point::new(
                (prev_point.x + data.position.x) / 2.0,
                (prev_point.y + data.position.y) / 2.0,
            );
            let path = Path::new(|builder| {
                builder.move_to(prev_point);
                builder.bezier_curve_to(control_point1, control_point2, data.position);
            });
            prev_point = data.position;

            // apply styling to curve
            let time_elapsed = now.duration_since(data.time).as_millis() as f32;
            if time_elapsed > fade_duration {
                return frame;
            }
            let progress = 1.0 - (time_elapsed / fade_duration);
            let width = max_width * progress; // width narrows
            let opacity = max_opacity * progress; // fade out

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
        if self.gesture_data.len() > 1 {
            return vec![self.draw_segment_method(frame).into_geometry()]
        }
        vec![frame.into_geometry()]
    }

    
}
