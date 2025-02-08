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

    fn update(&mut self, _message: Message) -> Task<Message> {
        Task::none()
    }
}




// pub struct View<T> {
//     pub parent: Option<Weak<RefCell<MainWindow>>>,
//     inner: T,
// }



// impl<T: ViewTrait> ViewTrait for View<T> {
//     fn new() -> Self {
//         Self {
//             parent: None,
//             inner: T::new(),
//         }
//     }

//     fn view(&self) -> Element<MainMessage> {
//         self.inner.view()
//     }

//     fn update(&mut self, message: MainMessage) -> Task<MainMessage> {
//         self.inner.update(message)
//     }

//     fn name(&self) -> &str {
//         self.inner.name()
//     }

//     fn has_gesture(&self) -> bool {
//         self.inner.has_gesture()
//     }

//     fn set_parent(&mut self, parent: Option<Weak<RefCell<MainWindow>>>) {
//         self.parent = parent;
//     }

//     fn get_parent(&self) -> Option<Rc<RefCell<MainWindow>>> {
//         self.parent.as_ref().and_then(|parent| parent.upgrade())
//     }
// }

// impl<T: ViewTrait> View<T> {
//     pub fn new(inner: T) -> Self {
//         Self {
//             parent: None,
//             inner,
//         }
//     }
// }


