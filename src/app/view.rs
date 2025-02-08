use iced::{
    Element,
    Task
};
use super::*;
use crate::views::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum View {
    CompactQWERTY,
    Configuration,
    Launcher,
    // Add more views/layouts here
}

impl View {
    pub const ALL: [View; 3] = [
        View::CompactQWERTY,
        View::Configuration,
        View::Launcher,
        // Add more views/layouts here
    ];

    pub fn init_views() -> Vec<Box<dyn ViewTrait>> {
        vec![
            Box::new(CompactQwertyView::new()),
            Box::new(ConfigurationView::new()),
            Box::new(LauncherView::new()),
            // Add more views/layouts here
        ]
    }
}

impl std::fmt::Display for View {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}",
            match self {
                View::CompactQWERTY => "Compact QWERTY",
                View::Configuration => "Configuration",
                View::Launcher => "Launcher",
                // Add more views/layouts here
            }
        )
    }
}

pub trait ViewTrait {
    fn new() -> Self where Self: Sized;
    fn view(&self) -> Element<main_app::Message>;
    fn class(&self) -> View;
    
    /// Returns true if this view has a gesture to handle, false otherwise.
    /// When a view has a gesture, a canvas is drawn on top of it to intercept
    /// touch and mouse events. The gesture is then evaluated in the corresponding
    /// view's `update` method.
    fn has_gesture(&self) -> bool { 
        // todo should this be a subscription? or just send a message when the view is changed?
        // todo remove this fn and make it a message?
        false
    }

    fn update(&mut self, _message: Message) -> Task<main_app::Message> {
        Task::none()
    }
}


// todo view handler which stores history of view, and manages the action gestures, view switching, and panes
pub struct ViewHandler {
    pub current_view: View, // enum
    pub views: Vec<Box<dyn ViewTrait>>, // list of ViewTrait objects
}

#[derive(Debug, Clone)]
pub enum Message {
    ChangeView(View),
    ActionGesture(ActionDirection),
    ViewMessage(usize),
}

impl ViewHandler {
    pub fn new() -> Self {
        ViewHandler {
            current_view: View::CompactQWERTY,
            views: View::init_views(),
        }
    }

    pub fn update(&mut self, message: Message) -> Task<main_app::Message> {
        match message {
            Message::ChangeView(view) => {
                info!("Change view to {view:?}");
                self.current_view = view;
                Task::none()
            }
            Message::ActionGesture(direction) => {
                match direction {
                    ActionDirection::TopLeft => Task::done(Message::ChangeView(View::CompactQWERTY)).map(main_app::Message::ViewHandler),
                    ActionDirection::Top => Task::done(Message::ChangeView(View::Configuration)).map(main_app::Message::ViewHandler),
                    ActionDirection::TopRight => Task::done(Message::ChangeView(View::CompactQWERTY)).map(main_app::Message::ViewHandler),
                    ActionDirection::Right => Task::done(Message::ChangeView(View::CompactQWERTY)).map(main_app::Message::ViewHandler),
                    ActionDirection::BottomRight => Task::done(Message::ChangeView(View::CompactQWERTY)).map(main_app::Message::ViewHandler),
                    ActionDirection::Bottom => Task::done(Message::ChangeView(View::Launcher)).map(main_app::Message::ViewHandler),
                    ActionDirection::BottomLeft => Task::done(Message::ChangeView(View::CompactQWERTY)).map(main_app::Message::ViewHandler),
                    ActionDirection::Left => Task::done(Message::ChangeView(View::CompactQWERTY)).map(main_app::Message::ViewHandler),
                }
            }
            //Message::ViewMessage() => self.current_view_mut().update(message),
            _ => Task::none()
        }
    }

    pub fn current_view(&self) -> &Box<dyn ViewTrait> {
        self.views.iter().find(|view| view.class() == self.current_view).expect("No matching view found")
    }

    pub fn current_view_mut(&mut self) -> &mut Box<dyn ViewTrait> {
        self.views.iter_mut().find(|view| view.class() == self.current_view).expect("No matching view found")
    }
}