use iced::Task;
use iced_layershell::{reexport::Anchor, to_layer_message};

use super::*;



pub struct WindowHandler {
    pub dock: Dock,
    pub windowed: bool,
    pub size: (u32, u32),
    pub margin: (i32, i32, i32, i32), // top, right, bottom, left
}


#[derive(Debug, Clone)]
pub enum Message {
    Dock(Dock),
}

impl WindowHandler {
    pub fn new() -> Self {
        WindowHandler {
            dock: Dock::Top,
            windowed: true,
            size: (600, 250),
            margin: (0, 0, 0, 0),
        }
    }

    pub fn update(&mut self, message: window::Message) -> Task<main_app::Message> {
        match message {
            Message::Dock(dock) => {
                self.dock = dock;
                match dock {
                    Dock::Left => {
                        return Task::done(main_app::Message::AnchorSizeChange(
                        Anchor::Left | Anchor::Top | Anchor::Bottom,
                        (400, 0),
                        ))
                    }
                    Dock::Right => {
                        return Task::done(main_app::Message::AnchorSizeChange(
                        Anchor::Right | Anchor::Top | Anchor::Bottom,
                        (400, 0),
                        ))
                    }
                    Dock::Bottom => {
                        return Task::done(main_app::Message::AnchorSizeChange(
                        Anchor::Bottom | Anchor::Left | Anchor::Right,
                        (0, 400),
                        ))
                    }
                    Dock::Top => {
                        return Task::done(main_app::Message::AnchorSizeChange(
                        Anchor::Top | Anchor::Left | Anchor::Right,
                        (0, 400),
                        ))
                    }
                }
            }
            _ => {Task::none()}
        }
    }


    // // todo move this into window helper, and make it work like the gesture
    // // start, end, append, update
    // fn move_window(&mut self, position: Point) -> Task<main_app::Message> {
    //     // get windows initial position - the margin
    //     if self.input_handler.rmouse_start.is_none() {
    //         self.input_handler.rmouse_start = Some(position);
    //         info!("start: {:?}", self.input_handler.rmouse_start.unwrap());
    //         return Task::none();
    //     }

    //     // calulate the difference
    //     let diff = self.input_handler.rmouse_start.unwrap() - position;
    //     info!("diff: {:?} {:?}", -diff.x as i32, diff.y as i32);

    //     // calculate for the margin change
    //     let y = diff.y as i32 + self.window_handler.margin.2;
    //     let x = -diff.x as i32 + self.window_handler.margin.3;

    //     //info!("mar: {:?} {:?}", x as i32, y as i32);

    //     // store the mouse pos
    //     self.input_handler.rmouse_start = Some(position);
        
    //     // apply margin to move window
    //     self.window_handler.margin.2 = y;
    //     self.window_handler.margin.3 = x;
    //     info!("mar: {:?} {:?}", x as i32, y as i32);
    //     return Task::done(Message::MarginChange((0, 0, y, x)))

    //     //Task::none()
    // }


}