use iced::{
    mouse,
    widget::canvas::{
        Frame, 
        Geometry, 
        Path, 
        Program, 
        Stroke},
    Point, 
    Rectangle, 
    Renderer,
    time::Instant,
    Color,
    Theme
};


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
        let mut frame = Frame::new(renderer, bounds.size());

        if self.gesture_data.len() > 1 {
            // last n points
            //let last_gesture_data: Vec<_> = self.gesture_data.iter().rev().take(100).cloned().collect();

            // Create the path using a Builder closure
            // create the line in reverse order
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

                // bezier_curve_to
                // for (i, point) in last_points.iter().enumerate().skip(1).step_by(4) {
                //     let prev_point = last_points.get(i - 1).unwrap();
                //     let control_point = Point::new(
                //         (prev_point.x + point.x) / 2.0,
                //         (prev_point.y + point.y) / 2.0,
                //     );
                //     builder.bezier_curve_to(
                //         control_point,
                //         Point::new(
                //             (control_point.x + point.x) / 2.0,
                //             (control_point.y + point.y) / 2.0,
                //         ),
                //         *point,
                //     );
                // }

                // line to is chunky
                // for (i, point) in self.points.iter().enumerate().skip(1).step_by(3) { // skip every n points
                //     builder.line_to(*point);
                //     builder.c
                // }
            });

            frame.stroke(
            &path,
            Stroke {
                style: Color::from_rgba(0.6, 0.8, 1.0, 0.3).into(),
                width: 8.0,
                ..Default::default()
            },
        );
        }

        //frame.into_geometry()

        

        // frame.fill(
        //     &Path::circle(frame.center(), frame.width().min(frame.height()) / 4.0),
        //     Color::from_rgb(0.6, 0.8, 1.0),
        // );

        // frame.stroke(
        //     &Path::line(
        //         frame.center() + Vector::new(-250.0, 100.0),
        //         frame.center() + Vector::new(250.0, -100.0),
        //     ),
        //     Stroke {
        //         style: Color::from_rgba(0.6, 0.8, 1.0, 0.5).into(),
        //         width: 20.0,
        //         ..Default::default()
        //     },
        // );

        vec![frame.into_geometry()]
    }

}