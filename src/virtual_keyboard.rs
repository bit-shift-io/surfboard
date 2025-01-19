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

    //let keyboard_handler_weak = keyboard_handler.as_weak(); // this doesnt work
    keyboard_handler.on_mouse_moved(|x: f32, y: f32| {
        let point = LogicalPosition::new(x, y);
        let array = keyboard_handler.get_mouse_position_history();
        keyboard_handler.set_mouse_position_history(array);
        info!("{}, {}", x, y);
    });

    // assign value directly
    let command = "M 0 0 L 0 100 A 1 1 0 0 0 100 100 L 100 0 Z";
    keyboard_handler.set_path_command(command.into());


    // example of read in-out value
    // non async tasks
    // keyboard_handler.on_path_command({
    //     let command = "M 0 0 L 0 100 A 1 1 0 0 0 100 100 L 100 0 Z";
    //     move || {
    //         let ui = ui_weak.unwrap();
    //         ui.set_path_command(command);
    //     }
    // });
}
