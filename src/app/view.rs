use iced::{
    Element,
    Task
};
use std::fmt;
use super::*;
use crate::views::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum View {
    CompactQwerty,
    Configuration,
    Launcher,
}

impl std::fmt::Display for View {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            View::CompactQwerty => write!(f, "Compact QWERTY"),
            View::Configuration => write!(f, "Configuration"),
            View::Launcher => write!(f, "Launcher"),
        }
    }
}

impl View {
    pub const ALL: [View; 3] = [
        View::CompactQwerty,
        View::Configuration,
        View::Launcher,
    ];
}

pub trait ViewTrait {
    fn new() -> Self where Self: Sized;
    fn view(&self) -> Element<main_app::Message>;
    
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

impl fmt::Debug for dyn ViewTrait + 'static {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ViewTrait").finish()
    }
}


// todo view handler which stores history of view, and manages the action gestures, view switching, and panes
#[derive(Clone, Debug)]
pub struct ViewHandler {
    pub current_view: View, // enum
    pub compact_qwerty_view: CompactQwertyView,
    pub configuration_view: ConfigurationView,
    pub launcher_view: LauncherView,
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
            current_view: View::CompactQwerty,
            compact_qwerty_view: CompactQwertyView::new(),
            configuration_view: ConfigurationView::new(),
            launcher_view: LauncherView::new(),
        }
    }

    pub fn update(&mut self, message: Message) -> Task<main_app::Message> {
        match message {
            Message::ChangeView(view) => {
                self.current_view = view;
                Task::none()
            }
            Message::ActionGesture(direction) => {
                let view_class = match direction {
                    ActionDirection::TopLeft => View::CompactQwerty,
                    ActionDirection::Top => View::Configuration,
                    ActionDirection::TopRight => View::CompactQwerty,
                    ActionDirection::Right => View::CompactQwerty,
                    ActionDirection::BottomRight => View::CompactQwerty,
                    ActionDirection::Bottom => View::Launcher,
                    ActionDirection::BottomLeft => View::CompactQwerty,
                    ActionDirection::Left => View::CompactQwerty,
                };
                Task::done(Message::ChangeView(view_class)).map(main_app::Message::ViewHandler)
            }
            Message::ViewMessage(_) => self.current_view_mut().update(message),
            _ => Task::none()
        }
    }

    pub fn view(&self) -> Element<main_app::Message> {
        self.current_view().view()
    }

    pub fn current_view(&self) -> &dyn ViewTrait {
        match self.current_view {
            View::CompactQwerty => &self.compact_qwerty_view,
            View::Configuration => &self.configuration_view,
            View::Launcher => &self.launcher_view,
        }
    }

    pub fn current_view_mut(&mut self) -> &mut dyn ViewTrait {
        match self.current_view {
            View::CompactQwerty => &mut self.compact_qwerty_view,
            View::Configuration => &mut self.configuration_view,
            View::Launcher => &mut self.launcher_view,
        }
    }
}