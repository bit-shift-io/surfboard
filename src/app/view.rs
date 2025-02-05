use iced::{
    Element,
    Task
};
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use super::*;



// pub struct View<T> {
//     pub parent: Option<Weak<RefCell<MainWindow>>>,
//     inner: T,
// }

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
    fn view(&self) -> Element<MainMessage>;
    fn update(&mut self, message: MainMessage) -> Task<MainMessage>;
    //fn name(&self) -> &str;
    fn class(&self) -> View;
    fn has_gesture(&self) -> bool;

    fn set_parent(&mut self, parent: Option<Weak<RefCell<MainWindow>>>) {
        // Default implementation does nothing
    }

    fn get_parent(&self) -> Option<Rc<RefCell<MainWindow>>> {
        // Default implementation returns None
        None
    }

    fn get_main_window(&self) -> Rc<RefCell<MainWindow>> {
        self.get_parent().expect("Parent not set")
    }

    // fn get_all_views(&self) -> &Vec<Box<dyn ViewTrait>> {
    //     &self.get_main_window().borrow().views
    // }
}

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