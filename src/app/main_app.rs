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
use std::rc::{
    Rc, 
    Weak
};
use std::cell::RefCell;
use crate::*;


pub struct MainApp {
    self_ref: Option<Weak<RefCell<MainApp>>>,
    gesture_handler: GestureHandler,
    window_handler: WindowHandler,
    input_handler: InputHandler,
    view_handler: ViewHandler,
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
            self_ref: None,
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
        // create a weakreference to the main window
        let main = Rc::new(RefCell::new(default));
        main.borrow_mut().initialize_self_ref(&main);
        (Rc::try_unwrap(main).map_err(|_| panic!("Failed to unwrap Rc")).unwrap().into_inner(), Task::none())
    }

    pub fn view(&self) -> Element<Message> {
        //info!("view draw");
        // todo move the has gesture into the guesture helper
        //let has_gesture = self.current_view().has_gesture();
        stack![
            self.view_handler.current_view().view(),
            self.gesture_handler.view(),
        ]
        .into()
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::IcedEvent(event) => self.input_handler.update(&event),
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

    fn initialize_self_ref(&mut self, self_rc: &Rc<RefCell<Self>>) {
        let weak_self = Rc::downgrade(self_rc);
        self.self_ref = Some(weak_self);
    }
}