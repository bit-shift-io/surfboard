use iced::{event, widget::{canvas, stack}, Color, Element, Event, Task, Theme};
use iced_layershell::{actions::LayershellCustomActions, application, reexport::Anchor, settings::{LayerShellSettings, Settings}, to_layer_message, Application};
use iced::{
    mouse,
    widget::{
        canvas::{Frame, Geometry, Path, Program, Stroke},
        column, Canvas,
    },
    Alignment, Length, Point, Rectangle, Renderer, Vector,
};
use iced::time::Instant; // wrapper for std time
use crate::{components::*, views::*};
use super::*;


pub struct MainWindow {
    windowed: bool,
    screen_edge: ScreenEdge,
    theme: iced::Theme,
    dark_mode: bool,
    lmouse_down: bool,
    rmouse_down: bool,
    current_view: View, // enum
    views: Vec<Box<dyn ViewTrait>>, // list of ViewTrait objects
    gesture_data: Vec<GestureData>,
}


#[to_layer_message]
#[derive(Debug, Clone)]
pub enum MainMessage {
    Debug(String),
    StringMessage(String),
    IcedEvent(Event),
    ChangeScreenEdge(ScreenEdge),
    ChangeView(View),
    KeyEnter,
    KeyExit,
    KeyPress,
    KeyRelease,
}


#[derive(Clone, Copy, Debug, PartialEq)]
pub enum View {
    Main,
    Settings,
    // Add more views/layouts here
}


impl MainWindow {
    pub fn title(&self) -> String {
        format!("surfboard")
    }

    fn current_view(&self) -> &Box<dyn ViewTrait> {
        self.views.iter().find(|view| view.class() == self.current_view).expect("No matching view found")
    }
}

impl Default for MainWindow {
    /// Creates a default instance of [`MainWindow`].
    fn default() -> Self {

        // Add more views/layouts here
        let views: Vec<Box<dyn ViewTrait>> = vec![
            Box::new(MainView::new()),
            Box::new(SettingsView::new()),
        ];

        // Return a default instance of MainWindow
        Self {
            windowed: true,
            screen_edge: ScreenEdge::Top,
            theme: iced::Theme::Light,
            dark_mode: true,
            lmouse_down: false,
            rmouse_down: false,
            current_view: View::Main,
            views,
            gesture_data: Vec::new(),
        }
    }
}


impl Application for MainWindow {
    type Message = MainMessage;
    type Flags = ();
    type Theme = Theme;
    type Executor = iced::executor::Default;

    /// Create a new instance of [`MainWindow`].
    fn new(_flags: ()) -> (Self, Task<Self::Message>) {
        let mut default_window = Self::default();
        default_window.windowed = false;
        (default_window, Task::none())
    }
    

    fn view(&self) -> Element<Self::Message> {
        let has_gesture = self.current_view().has_gesture();
        match has_gesture {
            true => {
                return stack![
                    self.current_view().view(),
                    Canvas::new(Gesture::new(&self.gesture_data)).width(Length::Fill).height(Length::Fill)
                ]
                .into()
            }
            false => {
                return self.current_view().view()
            }
        }
    }


    fn update(&mut self, message: Self::Message) -> Task<Self::Message> {
        match message {
            MainMessage::Debug(s) => {
                info!("{s}");
            }
            MainMessage::ChangeView(view) => {
                self.current_view = view;
            }
            MainMessage::StringMessage(s) => {
                info!("{s}");
            }
            MainMessage::IcedEvent(event) => {
                match event {
                    Event::Mouse(event) => {
                        match event {
                            mouse::Event::ButtonPressed(button) => {
                                match button {
                                    mouse::Button::Left => {
                                        self.gesture_data.clear();
                                        self.lmouse_down = true;
                                    }
                                    mouse::Button::Right => {
                                        self.rmouse_down = true;
                                    }
                                    _ => {info!("Unhandled mouse button")}
                                }
                            }
                            mouse::Event::ButtonReleased(button) => {
                                match button {
                                    mouse::Button::Left => {
                                        self.lmouse_down = false;
                                    }
                                    mouse::Button::Right => {
                                        self.rmouse_down = false;
                                    }
                                    _ => {info!("Unhandled mouse release")}
                                }
                            }
                            mouse::Event::CursorMoved { position } => {
                                if self.lmouse_down {
                                    let data = GestureData{position, time: std::time::Instant::now()};
                                    self.gesture_data.push(data);
                                }
                            }
                            _ => {info!("Unhandled event")}
                            
                        }
                    }
                    //Event::Keyboard(event) => todo!(),
                    //Event::Window(event) => todo!(),
                    //Event::Touch(event) => todo!(),
                    _ => {}, // info!("event: {event:?}");
                }
            }
            MainMessage::ChangeScreenEdge(screen_edge) => {
                self.screen_edge = screen_edge;
                match screen_edge {
                    ScreenEdge::Left => {
                        return Task::done(MainMessage::AnchorSizeChange(
                        Anchor::Left | Anchor::Top | Anchor::Bottom,
                        (400, 0),
                        ))
                    }
                    ScreenEdge::Right => {
                        return Task::done(MainMessage::AnchorSizeChange(
                        Anchor::Right | Anchor::Top | Anchor::Bottom,
                        (400, 0),
                        ))
                    }
                    ScreenEdge::Bottom => {
                        return Task::done(MainMessage::AnchorSizeChange(
                        Anchor::Bottom | Anchor::Left | Anchor::Right,
                        (0, 400),
                        ))
                    }
                    ScreenEdge::Top => {
                        return Task::done(MainMessage::AnchorSizeChange(
                        Anchor::Top | Anchor::Left | Anchor::Right,
                        (0, 400),
                        ))
                    }
                }
            }
            _ => {
                // Handle layout-specific messages
                //let view = self.current_view_mut();
                //let boxed_message = view.convert_message(message);
                //view.update(&boxed_message);
            }
        }
        Task::none()
    }


    fn style(&self, theme: &Self::Theme) -> iced_layershell::Appearance {
        iced_layershell::Appearance {
            //background_color: Color::TRANSPARENT,
            background_color: Color::from_rgba(0.21, 0.23, 0.25, 0.5),
            text_color: theme.palette().text,
        }
    }

    fn namespace(&self) -> String {
        String::from("surfboard")
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        event::listen().map(MainMessage::IcedEvent)
    }

    // fn run(settings: Settings<Self::Flags>) -> iced_layershell::Result
    // where Self: 'static,
    //       Self::Message: 'static + TryInto<LayershellCustomActions, Error = Self::Message> 
    // {
    //     let settings = Settings {
    //         layer_settings: LayerShellSettings {
    //             size: Some((600, 250)),
    //             //exclusive_zone: 250,
    //             anchor: Anchor::Bottom | Anchor::Right,
    //             //start_mode,
    //             margin: (10, 10, 10, 10),
    //             ..Default::default()
    //         },
    //         ..Default::default()
    //     };

    //     #[allow(clippy::needless_update)]
    //     let renderer_settings = iced_graphics::Settings {
    //         default_font: settings.default_font,
    //         default_text_size: settings.default_text_size,
    //         antialiasing: if settings.antialiasing {
    //             Some(iced_graphics::Antialiasing::MSAAx4)
    //         } else {
    //             None
    //         },
    //         ..iced_graphics::Settings::default()
    //     };

    //     application::run::<Instance<Self>, Self::Executor, iced_renderer::Compositor>(
    //         settings,
    //         renderer_settings,
    //     )
    // }
}


#[derive(Debug, Clone, Copy, PartialEq)]
struct GestureData {
    position: Point,
    time: Instant,
}

struct Gesture<'a> {
    gesture_data: &'a Vec<GestureData>,
}

impl<'a> Gesture<'a> {
    pub fn new(points: &'a Vec<GestureData>) -> Self {
        Gesture {
            gesture_data: points,
        }
    }
}

// impl Default for Gesture {
//     fn default() -> Self {
//         let points = vec![Point::new(0.0, 0.0), Point::new(100.0, 100.0), Point::new(400.0, 100.0)];
//         Gesture {
//             points,
//         }
//     }
// }

impl<'a, Message> Program<Message> for Gesture<'a> {
    type State = ();

    // fn update(
    //     &self,
    //     state: &mut Self::State,
    //     event: Event,
    //     bounds: Rectangle,
    //     cursor: mouse::Cursor,
    // ) -> Option<canvas::Action<Curve>> {
    //     let cursor_position = cursor.position_in(bounds)?;
    //     info!("{cursor_position}");


    //     match event {
    //         Event::Mouse(mouse::Event::ButtonPressed(
    //             mouse::Button::Left,
    //         )) => {
    //             }
    //         Event::Mouse(mouse::Event::CursorMoved { .. })
    //         {
    //             if !self.points.is_empty() {
    //                 self.points.push(point);
    //             }
    //         }
    //         _ => None,
    //     }
    // }

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