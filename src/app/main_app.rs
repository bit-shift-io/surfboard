use iced::{
    event, theme::{self, Style}, widget::stack, Color, Element, Event, Subscription, Task
};

#[cfg(target_os="windows")]
use iced_layershell::{
    reexport::{
        Anchor, 
        KeyboardInteractivity, 
        Layer
    },
    to_layer_message,
};

use crate::*;


to_layershell_message! { // extra layershell messages
#[derive(Debug, Clone)]
pub enum Message {
    Debug(String),
    IcedEvent(Event),
    WindowHandler(window::Message),
    ViewHandler(view::Message),
    GestureHandler(gesture::Message),
    InputHandler(input::Message),
    ComponentHandler(component::Message),
    None,
}
}


/// The main app holds all the helpers and links everything together.  
#[derive(Debug)]
pub struct MainApp {
    pub component_handler: ComponentHandler,
    pub gesture_handler: GestureHandler,
    pub window_handler: WindowHandler,
    pub input_handler: InputHandler,
    pub view_handler: ViewHandler,
}


impl Default for MainApp {
    fn default() -> Self {
        Self {
            component_handler: ComponentHandler::new(),
            gesture_handler: GestureHandler::new(),
            window_handler: WindowHandler::new(),
            input_handler: InputHandler::new(),
            view_handler: ViewHandler::new(),
        }
    }
}

impl MainApp {
  
    pub fn window_size() -> iced::Size {
        let window_handler = WindowHandler::new();
        iced::Size::new(window_handler.size.0 as f32, window_handler.size.1 as f32)
    }

    pub fn new() -> (Self, Task<Message>) {
        let default = Self::default();
        (default, Task::none())
    }

    pub fn init(&mut self) {
        self.view_handler.init();
    }

    pub fn view(&self) -> Element<Message> {
        // TODO: should this be changed to use map??
        stack![self.view_handler.view(), self.gesture_handler.view()].into()
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        // TODO: should this be changed to use map??
        match message {
            Message::IcedEvent(event) => self.input_handler.update2(&event, &mut self.gesture_handler, &mut self.window_handler),
            Message::InputHandler(msg) => self.input_handler.update(msg),
            Message::WindowHandler(msg) => self.window_handler.update(msg),
            Message::GestureHandler(msg) => self.gesture_handler.update(msg),
            Message::ViewHandler(msg) => self.view_handler.update(msg),
            Message::ComponentHandler(msg) => self.component_handler.update(msg),
            Message::Debug(s) => {
                info!("{s}");
                Task::none()
            }
            _ => Task::none(),
        }
    }

    pub fn style(&self, theme: &iced::Theme) -> Style {
        Style {
            background_color: Color::from_rgba(0.21, 0.23, 0.25, 0.95),
            text_color: theme.palette().text,
        }
    }


    pub fn namespace(&self) -> String {
        String::from("surfboard")
    }

    pub fn subscription(&self) -> Subscription<Message> {
        let main_subscription = event::listen().map(Message::IcedEvent);
        let input_subscription = self.input_handler.subscription().map(Message::InputHandler);
        Subscription::batch(vec![
            main_subscription,
            input_subscription,
        ])
    }
}


#[cfg(target_os="windows")]
impl MainApp {
    pub fn default_layer_shell(_start_mode: StartMode) -> LayerShellSettings {
        let window_handler = WindowHandler::new();
        // default free window mode
        LayerShellSettings {
            anchor: Anchor::Top | Anchor::Left, //| Anchor::Right,
            layer: Layer::Top,                  // Layer::Overlay if need to go the max
            exclusive_zone: -1,
            size: Some(window_handler.size), //None,
            margin: window_handler.margin,
            keyboard_interactivity: KeyboardInteractivity::OnDemand,
            events_transparent: false,
            start_mode: StartMode::default(),
        }
    }

    pub fn style_layershell(&self, theme: &iced::Theme) -> iced_layershell::Appearance {
        iced_layershell::Appearance {
            background_color: Color::from_rgba(0.21, 0.23, 0.25, 0.95),
            text_color: theme.palette().text,
        }
    }
}