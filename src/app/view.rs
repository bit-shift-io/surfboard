use iced::{
    Element,
    Task
};
use super::*;


pub trait ViewTrait {
    fn new() -> Self where Self: Sized;
    fn view(&self) -> Element<MainMessage>;
    fn update(&mut self, message: MainMessage) -> Task<MainMessage>;
    fn class(&self) -> View;
    fn has_gesture(&self) -> bool;
}


#[derive(Clone, Copy, Debug, PartialEq)]
pub enum View {
    Main,
    Configuration,
    ApplicationLauncher,
    // Add more views/layouts here
}

impl View {
    pub const ALL: [View; 3] = [
        View::Main,
        View::Configuration,
        View::ApplicationLauncher,
        // Add more views/layouts here
    ];
}

impl std::fmt::Display for View {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}",
            match self {
                View::Main => "Compact QWERTY",
                View::Configuration => "Configuration",
                View::ApplicationLauncher => "Launcher",
                // Add more views/layouts here
            }
        )
    }
}