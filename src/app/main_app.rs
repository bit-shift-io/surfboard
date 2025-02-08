use iced::{
    event, 
    widget::stack, 
    Color, 
    Element, 
    Event, 
    Point, 
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
use std::fmt::Debug;
use crate::*;



pub struct MainApp {
    self_ref: Option<Weak<RefCell<MainApp>>>, // weak ref
    current_view: View, // enum
    pub views: Vec<Box<dyn ViewTrait>>, // list of ViewTrait objects
    gesture_handler: GestureHandler,
    window_handler: WindowHandler,
    input_handler: InputHandler,
}


#[to_layer_message] // used for extra iced messages
#[derive(Debug, Clone)]
pub enum Message {
    Debug(String),
    IcedEvent(Event),
    ViewMessage(usize),
    WindowHandler(window::Message),
    ChangeView(View),
    ActionGesture(ActionDirection),
    GestureHandler(gesture::Message),
    None,
}


impl MainApp {
    fn initialize_self_ref(&mut self, self_rc: &Rc<RefCell<Self>>) {
        let weak_self = Rc::downgrade(self_rc);
        self.self_ref = Some(weak_self);
    }

    fn current_view(&self) -> &Box<dyn ViewTrait> {
        self.views.iter().find(|view| view.class() == self.current_view).expect("No matching view found")
    }

    fn current_view_mut(&mut self) -> &mut Box<dyn ViewTrait> {
        self.views.iter_mut().find(|view| view.class() == self.current_view).expect("No matching view found")
    }


    // todo move this into window helper, and make it work like the gesture
    // start, end, append, update
    fn move_window(&mut self, position: Point) -> Task<main_app::Message> {
        // get windows initial position - the margin
        if self.input_handler.rmouse_start.is_none() {
            self.input_handler.rmouse_start = Some(position);
            info!("start: {:?}", self.input_handler.rmouse_start.unwrap());
            return Task::none();
        }

        // calulate the difference
        let diff = self.input_handler.rmouse_start.unwrap() - position;
        info!("diff: {:?} {:?}", -diff.x as i32, diff.y as i32);

        // calculate for the margin change
        let y = diff.y as i32 + self.window_handler.margin.2;
        let x = -diff.x as i32 + self.window_handler.margin.3;

        //info!("mar: {:?} {:?}", x as i32, y as i32);

        // store the mouse pos
        self.input_handler.rmouse_start = Some(position);
        
        // apply margin to move window
        self.window_handler.margin.2 = y;
        self.window_handler.margin.3 = x;
        info!("mar: {:?} {:?}", x as i32, y as i32);
        return Task::done(Message::MarginChange((0, 0, y, x)))

        //Task::none()
    }


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

}


impl Default for MainApp {
    fn default() -> Self {
        Self {
            self_ref: None,
            current_view: View::CompactQWERTY,
            views: View::init_views(),
            gesture_handler: GestureHandler::new(),
            window_handler: WindowHandler::new(),
            input_handler: InputHandler::new(),
        }
    }
}


impl MainApp {
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
            self.current_view().view(),
            self.gesture_handler.view(),
        ]
        .into()
    }


    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::IcedEvent(event) => self.input_handler.update(&event),
            Message::WindowHandler(msg) => self.window_handler.update(msg),
            Message::ViewMessage(_) => self.current_view_mut().update(message),
            Message::GestureHandler(msg) => self.gesture_handler.update(msg),
            Message::ActionGesture(direction) => {
                info!("Gesture: {direction:?}");
                Task::none()
            }
            Message::Debug(s) => {
                info!("{s}");
                Task::none()
            }
            Message::ChangeView(view) => {
                info!("Change view to {view:?}");
                self.current_view = view;
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