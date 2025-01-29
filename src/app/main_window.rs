use std::collections::VecDeque;

use iced::{
    event, mouse, time::Instant, widget::{
        stack, 
        Canvas,
    }, Color, Element, Event, Length, Point, Task, Theme
};
use iced_layershell::{
    reexport::Anchor, 
    to_layer_message, 
    Application
};
use crate::{
    components::*, 
    views::*,
    *,
};


pub struct MainWindow {
    windowed: bool,
    screen_edge: ScreenEdge,
    theme: iced::Theme,
    dark_mode: bool,
    lmouse_down: bool,
    rmouse_down: bool,
    current_view: View, // enum
    views: Vec<Box<dyn ViewTrait>>, // list of ViewTrait objects
    gesture_data: VecDeque<GestureData>,
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
    fn current_view(&self) -> &Box<dyn ViewTrait> {
        self.views.iter().find(|view| view.class() == self.current_view).expect("No matching view found")
    }

    fn push_gesture_data(&mut self, position: Point) {
        // debug print out the points
        info!("\nGesture Data:");
        self.gesture_data.iter().for_each(|item| info!("{:?}", item));

        if self.gesture_data.len() > 1 {
            // distance check with the back item
            let prev = self.gesture_data.back().unwrap();
            let distance = Point::distance(&prev.point, position);
            if distance < 20.0 {
                return;
            }
            
            // time check
            // remove the front items
            while let Some(item) = self.gesture_data.front() {
                let elapsed = Instant::now().duration_since(item.instant);
                if elapsed.as_millis() > 1000 { // 2 seconds
                    self.gesture_data.pop_front();
                } else {
                    break;
                }
            }

        }

        // round off the position
        let point = Point::new(position.x.round(), position.y.round());

        // add data to the back
        self.gesture_data.push_back(GestureData {
            point,
            instant: Instant::now(),
        });
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
            gesture_data: VecDeque::new(),
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
                                    self.push_gesture_data(position);
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
}
