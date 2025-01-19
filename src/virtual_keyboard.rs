use super::*;
use private_unstable_api::re_exports::Point;
use slint::*;
use svg_path::Path;

pub struct VirtualKeyboard<'a> {
    pub keyboard_handler: VirtualKeyboardHandler<'a>,
    pub main_window: &'a MainWindow,
    pub mouse_position_history: Vec<Point>, // or logical position?
    pub svg_path: Path,
}

impl<'a> VirtualKeyboard<'a> {
    pub fn new(ui: &'a MainWindow) -> Self  {
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
            //let point = LogicalPosition::new(x, y);
            //let array = keyboard_handler.get_mouse_position_history();
            //keyboard_handler.set_mouse_position_history(array);
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

        // gesture path
        let mut path = Path::new();
        path.add_point([0, 0]);
        path.add_point([0, 100]);
        path.add_point([50, 50]);
        info!("{}", path.create_command());

        keyboard_handler.set_path_command(path.create_command().into());

        VirtualKeyboard {
            keyboard_handler,
            main_window: ui,
            mouse_position_history: Vec::new(),
            svg_path: path,
        }
        
    }
}

