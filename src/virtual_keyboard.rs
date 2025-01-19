use std::rc::Rc;
use tokio::sync::mpsc;
use super::*;
use private_unstable_api::re_exports::Point;
use slint::*;
use svg_path::Path;

pub struct VirtualKeyboard<'a> {
    pub handler: VirtualKeyboardHandler<'a>, // the slint virtual keyboard handler
    pub main_window: &'a MainWindow,
    pub gesture_history: Vec<Point>,
    pub svg_path: Path,
}

impl<'a> VirtualKeyboard<'a> {
    pub fn new(ui: &'a MainWindow) -> Self  {
        let handler = ui.global::<VirtualKeyboardHandler>();
        let mut svg_path = Path::new();
        let gesture_history = Vec::new();
        
        // callback
        handler.on_key_pressed({
            let ui_weak = ui.as_weak();
            move |key| {
                ui_weak.unwrap()
                    .window()
                    .dispatch_event(slint::platform::WindowEvent::KeyPressed { text: key.clone() });
                ui_weak.unwrap()
                    .window()
                    .dispatch_event(slint::platform::WindowEvent::KeyReleased { text: key });
            }
        });
    
        /* example of interacting with a global in a closure and assigning arrays via modelrc
        // Create an empty VecModel and put it in an Rc, convert to ModelRc and assign
        let the_model : Rc<VecModel<LogicalPosition>> = Rc::new(VecModel::default());
        let the_model_rc = ModelRc::from(the_model.clone());
        keyboard_handler.set_mouse_position_history(the_model_rc);

        // callback
        keyboard_handler.on_mouse_moved({
            let ui_weak = ui.as_weak();
            move |x: f32, y: f32| {
                let ui= ui_weak.unwrap();
                let keyboard_handler = ui.global::<VirtualKeyboardHandler>();
                let point = LogicalPosition::new(x, y);
                the_model.push(point);
                let the_model_rc = ModelRc::from(the_model.clone());
                keyboard_handler.set_mouse_position_history(the_model_rc);
                info!("{}, {}", x, y);
            }
        });
        */


        handler.on_mouse_moved({
            move |x: f32, y: f32| {
                //svg_path.add_point([x as usize, y as usize]);
                info!("{}, {}", x, y);
                //let point = [x as usize, y as usize];
                //svg_path.add_point(point);
            }
        });

        // callback
        // keyboard.handler.on_mouse_moved({
        //     let ui_weak = ui.as_weak();
        //     move |x: f32, y: f32| {
        //         let ui= ui_weak.unwrap();
        //         let keyboard_handler = ui.global::<VirtualKeyboardHandler>();
        //         keyboard.svg_path.add_point([x as usize, y as usize]);
        //         let command = keyboard.svg_path.create_command().into();
        //         keyboard_handler.set_path_command(command);
        //         info!("{}, {}", x, y);
        //     }
        // });
    
        // // assign value directly
        // let command = "M 0 0 L 0 100 A 1 1 0 0 0 100 100 L 100 0 Z";
        // keyboard_handler.set_path_command(command.into());
    

        
        // path.add_point([0, 0]);
        // path.add_point([0, 100]);
        // path.add_point([50, 50]);
        // info!("{}", path.create_command());

        // keyboard_handler.set_path_command(path.create_command().into());
        VirtualKeyboard {
            handler,
            main_window: ui,
            gesture_history,
            svg_path,
        }
    }
}

