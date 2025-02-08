pub mod view;
pub mod main_app;
pub mod dock;
pub mod window;

// re-export
pub use super::app::dock::*;
pub use super::app::view::*;
pub use super::app::main_app::*;
pub use super::app::window::*;