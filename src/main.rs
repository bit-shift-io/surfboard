slint::include_modules!();

mod virtual_keyboard;
mod wayland;

pub fn main() {
    let ui = MainWindow::new().unwrap();

    wayland::init(&ui);       

    virtual_keyboard::init(&ui);

    ui.run().unwrap();
}