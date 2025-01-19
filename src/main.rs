#[macro_use]
extern crate log;

slint::include_modules!();
mod virtual_keyboard;
mod wayland;
mod utils;
use utils::*;

pub fn main() {
    functions::init_logger();
    info!("== Start Surfboard ==");
    let ui = MainWindow::new().unwrap();
    wayland::init(&ui);       
    virtual_keyboard::init(&ui);
    ui.run().unwrap();
}