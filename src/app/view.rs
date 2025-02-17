use iced::{
    Element,
    Task
};
use std::fmt;
use super::*;
use crate::{utils::*, views::*};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum View {
    CompactQwerty,
    Settings,
    Launcher,
    QuickPick,
    Pick,
    // Add more views/layouts here
}

impl std::fmt::Display for View {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            View::CompactQwerty => write!(f, "Compact QWERTY"),
            View::Settings => write!(f, "Settings"),
            View::Launcher => write!(f, "Launcher"),
            View::QuickPick => write!(f, "Quick Pick"),
            View::Pick => write!(f, "Pick"),
            // Add more views/layouts here
        }
    }
}

impl View {
    pub const ALL: [View; 5] = [
        View::CompactQwerty,
        View::Settings,
        View::Launcher,
        View::QuickPick,
        View::Pick,
        // Add more views/layouts here
    ];
}


pub trait ViewTrait {
    fn new() -> Self where Self: Sized;
    fn init(&mut self, view_handler: &mut ViewHandler) {}
    fn view(&self, view_handler: &ViewHandler) -> Element<main_app::Message>;
    fn update(&mut self, _message: Message) -> Task<main_app::Message> {Task::none()}
    fn class(&self) -> View;
    fn name(&self) -> String {self.class().to_string()}
    fn icon(&self) -> &'static [u8] {globals::ICON_DEFAULT}
    
    /// Returns true if this view has a gesture to handle, false otherwise.
    /// When a view has a gesture, a canvas is drawn on top of it to intercept
    /// touch and mouse events. The gesture is then evaluated in the corresponding
    /// view's `update` method.
    fn has_gesture(&self) -> bool { 
        // todo should this be a subscription? or just send a message when the view is changed?
        // todo remove this fn and make it a message?
        false
    }
}

impl fmt::Debug for dyn ViewTrait + 'static {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ViewTrait").finish()
    }
}


// todo view handler which stores history of view, and manages the action gestures, view switching, and panes
#[derive(Debug)]
pub struct ViewHandler {
    pub current_view: View,
    pub views: [Box<dyn ViewTrait>; 5], // Add more views/layouts here
}

#[derive(Debug, Clone)]
pub enum Message {
    ChangeView(View),
    ActionGesture(ActionDirection),
    ViewMessage(usize),
}

impl ViewHandler {
    pub fn new() -> Self {
        let views: [Box<dyn ViewTrait>; 5] = [
            Box::new(CompactQwertyView::new()),
            Box::new(SettingsView::new()),
            Box::new(LauncherView::new()),
            Box::new(MiniPickView::new()),
            Box::new(PickView::new()),
            // Add more views/layouts here
        ];

        ViewHandler {
            current_view: View::CompactQwerty,
            views,
        }
    }

    pub fn init(&mut self) {
        info!("Initializing views");
        // Create a temporary reference to self
        let view_handler = self as *mut ViewHandler;

        for view in self.views.iter_mut() {
            // SAFETY: We know that view_handler is valid for the duration of this loop
            unsafe {
                view.init(&mut *view_handler);
            }
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
                    ActionDirection::Top => View::Settings,
                    ActionDirection::TopRight => View::CompactQwerty,
                    ActionDirection::Right => View::CompactQwerty,
                    ActionDirection::BottomRight => View::CompactQwerty,
                    ActionDirection::Bottom => View::Launcher,
                    ActionDirection::BottomLeft => View::CompactQwerty,
                    ActionDirection::Left => View::CompactQwerty,
                    ActionDirection::LongPress => View::Pick,
                };
                Task::done(Message::ChangeView(view_class)).map(main_app::Message::ViewHandler)
            }
            Message::ViewMessage(_) => self.current_view_mut().update(message),
            //_ => Task::none()
        }
    }

    pub fn view(&self) -> Element<main_app::Message> {
        self.current_view().view(self)
    }

    pub fn current_view(&self) -> &Box<dyn ViewTrait> {
        self.views.iter().find(|view| view.class() == self.current_view).expect("No matching view found")
    }

    pub fn current_view_mut(&mut self) -> &mut Box<dyn ViewTrait> {
        self.views.iter_mut().find(|view| view.class() == self.current_view).expect("No matching view found")
    }

    // pub fn all_class(&self) -> Vec<View> {
    //     self.views.iter().map(|view| view.class()).collect()
    // }

    // pub fn collect_as_string(&self) -> Vec<&str> {
    //     self.views.iter().map(|view| view.name()).collect()
    // }
}