use iced::{event, Color, Element, Event, Task as Command, Theme};
use iced_layershell::{reexport::Anchor, Application, to_layer_message};

use crate::views::*;
use super::*;


pub struct MainWindow {
    screen_edge: ScreenEdge,
    theme: iced::Theme,
    dark_mode: bool,
    current_view: Views, // enum
    views: Vec<Box<dyn ViewTrait>>, // list of ViewTrait objects
}


#[derive(Clone, Copy, Debug)]
pub enum Views {
    Main,
    Settings,
    // Add more views/layouts here (e.g., Qwerty, Dvorak)
}


#[to_layer_message]
#[derive(Debug, Clone)]
pub enum MainMessage {
    Debug(String),
    StringMessage(String),
    IcedEvent(Event),
    ChangeScreenEdge(ScreenEdge),
    ChangeView(Views),
}


impl MainWindow {
    fn current_view_mut(&mut self) -> &mut Box<dyn ViewTrait> {
        match self.current_view {
            Views::Main => &mut self.views[0],
            Views::Settings => &mut self.views[1],
            // Add more matches for other layouts
        }
    }

    fn current_view(&self) -> &Box<dyn ViewTrait> {
        match self.current_view {
            Views::Main => &self.views[0],
            Views::Settings => &self.views[1],
            // Add more matches for other layouts
        }
    }

}


impl Application for MainWindow {
    type Message = MainMessage;
    type Flags = ();
    type Theme = Theme;
    type Executor = iced::executor::Default;

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        // put new views/layouts here
        let views: Vec<Box<dyn ViewTrait>> = vec![
            Box::new(MainView::new()),
            Box::new(SettingsView::new()),
        ];

        (
            Self {
                screen_edge: ScreenEdge::Top,
                theme: iced::Theme::Light,
                dark_mode: true,
                current_view: Views::Main,
                views,
            },
            Command::none(),
        )
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
                info!("{event:?}");
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
            background_color: Color::TRANSPARENT,
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