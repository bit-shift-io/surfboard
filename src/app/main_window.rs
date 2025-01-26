use iced::{event, Color, Element, Event, Task as Command, Theme};
use iced_layershell::{reexport::Anchor, to_layer_message, Application};

use crate::views::*;
use super::*;


pub struct MainWindow {
    windowed: bool,
    screen_edge: ScreenEdge,
    theme: iced::Theme,
    dark_mode: bool,
    current_view: View, // enum
    views: Vec<Box<dyn ViewTrait>>, // list of ViewTrait objects
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
            current_view: View::Main,
            views,
        }
    }
}


impl Application for MainWindow {
    type Message = MainMessage;
    type Flags = ();
    type Theme = Theme;
    type Executor = iced::executor::Default;

    /// Create a new instance of [`MainWindow`].
    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        let mut default_window = Self::default();
        default_window.windowed = false;
        (default_window, Command::none())
    }
    

    fn view(&self) -> Element<Self::Message> {
        self.current_view().view()
    }


    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
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
                //info!("{event:?}");
            }
            MainMessage::ChangeScreenEdge(screen_edge) => {
                self.screen_edge = screen_edge;
                match screen_edge {
                    ScreenEdge::Left => {
                        return Command::done(MainMessage::AnchorSizeChange(
                        Anchor::Left | Anchor::Top | Anchor::Bottom,
                        (400, 0),
                        ))
                    }
                    ScreenEdge::Right => {
                        return Command::done(MainMessage::AnchorSizeChange(
                        Anchor::Right | Anchor::Top | Anchor::Bottom,
                        (400, 0),
                        ))
                    }
                    ScreenEdge::Bottom => {
                        return Command::done(MainMessage::AnchorSizeChange(
                        Anchor::Bottom | Anchor::Left | Anchor::Right,
                        (0, 400),
                        ))
                    }
                    ScreenEdge::Top => {
                        return Command::done(MainMessage::AnchorSizeChange(
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
        Command::none()
    }

    fn style(&self, theme: &Self::Theme) -> iced_layershell::Appearance {
        iced_layershell::Appearance {
            //background_color: Color::TRANSPARENT,
            background_color: Color::from_rgb(0.21, 0.23, 0.25),
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