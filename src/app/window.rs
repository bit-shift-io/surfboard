use iced::{Point, Task};
//use iced_layershell::reexport::Anchor;
use super::*;

/// Handles all things window related.  
#[derive(Clone, Debug)]
pub struct WindowHandler {
    pub dock: Dock,
    pub windowed: bool,
    pub size: (u32, u32),
    pub margin: (i32, i32, i32, i32), // top, right, bottom, left
    pub moving: bool,
    move_start: Option<Point>,
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
            moving: false,
            move_start: None,
        }
    }

    pub fn update(&mut self, message: window::Message) -> Task<main_app::Message> {
        // match message {
        //     Message::Dock(dock) => {
        //         if self.windowed {
        //             // todo need to chain commands as we need to restore the margins?
        //             return Task::done(main_app::Message::AnchorSizeChange(
        //                 Anchor::Left | Anchor::Top,
        //                 self.size,
        //             ))
        //         }
        //         self.dock = dock;
        //         let size_dock = 400;
        //         match dock {
        //             Dock::Left => {
        //                 return Task::done(main_app::Message::AnchorSizeChange(
        //                     Anchor::Left | Anchor::Top | Anchor::Bottom,
        //                     (size_dock, 0),
        //                 ))
        //             }
        //             Dock::Right => {
        //                 return Task::done(main_app::Message::AnchorSizeChange(
        //                     Anchor::Right | Anchor::Top | Anchor::Bottom,
        //                     (size_dock, 0),
        //                 ))
        //             }
        //             Dock::Bottom => {
        //                 return Task::done(main_app::Message::AnchorSizeChange(
        //                     Anchor::Bottom | Anchor::Left | Anchor::Right,
        //                     (0, size_dock),
        //                 ))
        //             }
        //             Dock::Top => {
        //                 return Task::done(main_app::Message::AnchorSizeChange(
        //                     Anchor::Top | Anchor::Left | Anchor::Right,
        //                     (0, size_dock),
        //                 ))
        //             }
        //         }
        //     }
        //     //_ => Task::none()
        // }
        Task::none()
    }


    pub fn start_move(&mut self) -> Task<main_app::Message> {
        self.moving = true;
        self.move_start = None;
        Task::none()
    }

    pub fn end_move(&mut self) -> Task<main_app::Message> {
        self.moving = false;
        Task::none()
    }

    pub fn append_move(&mut self, position: Point) -> Task<main_app::Message> {
        // get windows initial position - the margin
        if self.move_start.is_none() {
            self.move_start = Some(position);
            return Task::none();
        }

        // calulate the difference
        let diff = self.move_start.unwrap() - position;
        info!("diff: {:?} {:?}", -diff.x as i32, -diff.y as i32);

        // calculate for the margin change
        let y = -diff.y as i32 + self.margin.2;
        let x = -diff.x as i32 + self.margin.3;

        //info!("mar: {:?} {:?}", x as i32, y as i32);

        // store the mouse pos
        self.move_start = Some(position);
        
        // apply margin to move window
        self.margin.2 = y;
        self.margin.3 = x;
        info!("mar: {:?} {:?}", x as i32, y as i32);
        //return Task::done(main_app::Message::MarginChange((y, 0, 0, x)))
        Task::none()
    }
}



#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Dock {
    Top,
    #[default]
    Bottom,
    Left,
    Right,
}

impl Dock {
    pub const ALL: [Dock; 4] = [
        Dock::Top,
        Dock::Bottom,
        Dock::Left,
        Dock::Right,
    ];
}

impl std::fmt::Display for Dock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}",
            match self {
                Dock::Top => "Top",
                Dock::Bottom => "Bottom",
                Dock::Left => "Left",
                Dock::Right => "Right",
            }
        )
    }
}