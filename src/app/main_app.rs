use iced::{
    event, 
    widget::stack, 
    Color, 
    Element, 
    Event, 
    Subscription, 
    Task, 
};
use iced_layershell::{
    reexport::{
        Anchor, 
        KeyboardInteractivity, 
        Layer
    }, 
    to_layer_message,
};
use crate::*;


#[derive(Clone, Debug)]
pub struct MainApp {
    pub gesture_handler: GestureHandler,
    pub window_handler: WindowHandler,
    pub input_handler: InputHandler,
    pub view_handler: ViewHandler,
}


#[to_layer_message] // used for extra iced messages
#[derive(Debug, Clone)]
pub enum Message {
    Debug(String),
    IcedEvent(Event),
    WindowHandler(window::Message),
    ViewHandler(view::Message),
    GestureHandler(gesture::Message),
    None,
}


impl Default for MainApp {
    fn default() -> Self {
        Self {
            gesture_handler: GestureHandler::new(),
            window_handler: WindowHandler::new(),
            input_handler: InputHandler::new(),
            view_handler: ViewHandler::new(),
        }
    }
}


impl MainApp {
    pub fn default_layer_shell(_start_mode: StartMode) -> LayerShellSettings {
        let window_handler = WindowHandler::new();
        // default free window mode
        LayerShellSettings {
            anchor: Anchor::Bottom | Anchor::Left, //| Anchor::Right,
            layer: Layer::Top, // Layer::Overlay if need to go the max
            exclusive_zone: -1,
            size: Some(window_handler.size), //None,
            margin: window_handler.margin,
            keyboard_interactivity: KeyboardInteractivity::OnDemand,
            events_transparent: false,
            start_mode: StartMode::default(),
        }
    }

    pub fn new() -> (Self, Task<Message>) {
        let default = Self::default();
        (default, Task::none())
    }

    pub fn view(&self) -> Element<Message> {
        stack![
            self.view_handler.view(),
            self.gesture_handler.view(),
        ]
        .into()
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::IcedEvent(event) => self.input_handler.update(&event, &mut self.gesture_handler),
            Message::WindowHandler(msg) => self.window_handler.update(msg),
            Message::GestureHandler(msg) => self.gesture_handler.update(msg),
            Message::ViewHandler(msg) => self.view_handler.update(msg),
            Message::Debug(s) => {
                info!("{s}");
                Task::none()
            }
            _ => Task::none()
        }
    }


    pub fn style(&self, theme: &iced::Theme) -> iced_layershell::Appearance {
        iced_layershell::Appearance {
            //background_color: Color::TRANSPARENT,
            background_color: Color::from_rgba(0.21, 0.23, 0.25, 0.5),
            text_color: theme.palette().text,
        }
    }

    pub fn namespace(&self) -> String {
        String::from("surfboard")
    }

    pub fn subscription(&self) -> Subscription<Message> {
        let main_subscription = event::listen().map(Message::IcedEvent);
        let gesture_subscription = self.gesture_handler.subscription().map(Message::GestureHandler);
        Subscription::batch(vec![main_subscription, gesture_subscription])
    }
}