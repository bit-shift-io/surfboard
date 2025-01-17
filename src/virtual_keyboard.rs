use super::*;
use slint::*;

pub fn init(app: &MainWindow) {
    let weak = app.as_weak();
    app.global::<VirtualKeyboardHandler>().on_key_pressed({
        move |key| {
            weak.unwrap()
                .window()
                .dispatch_event(slint::platform::WindowEvent::KeyPressed { text: key.clone() });
            weak.unwrap()
                .window()
                .dispatch_event(slint::platform::WindowEvent::KeyReleased { text: key });
        }
    });
}
