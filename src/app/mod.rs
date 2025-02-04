pub mod view;
pub mod main_window;
pub mod dock;

// re-export
pub use super::app::dock::*;
pub use super::app::view::*;
pub use super::app::main_window::*;