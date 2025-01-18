use super::*;
use slint::*;

pub fn init(ui: &MainWindow) {
    let ui_weak = ui.as_weak();
    let keyboard_handler = ui.global::<VirtualKeyboardHandler>();

    // callback
    keyboard_handler.on_key_pressed({
        move |key| {
            ui_weak.unwrap()
                .window()
                .dispatch_event(slint::platform::WindowEvent::KeyPressed { text: key.clone() });
            ui_weak.unwrap()
                .window()
                .dispatch_event(slint::platform::WindowEvent::KeyReleased { text: key });
        }
    });

    // callback
    keyboard_handler.on_add_mouse_position(|x: f32, y: f32| {
        println!("{}, {}", x, y);
    });


    // example of read in-out value
    // non async tasks
    // ui.on_request_increase_value({
    //     let ui_handle = ui.as_weak();
    //     move || {
    //         let ui = ui_handle.unwrap();
    //         ui.set_speed(ui.get_speed() + 1);
    //     }
    // });
}
