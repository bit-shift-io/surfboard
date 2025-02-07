use iced::{
    event, 
    keyboard::{
        self, 
        key::Named
    }, 
    mouse, 
    time::Instant, 
    touch, 
    widget::stack, 
    Color, 
    Element, 
    Event, 
    Point, 
    Subscription, 
    Task, 
};
use iced_layershell::{
    reexport::{
        Anchor, 
        KeyboardInteractivity, 
        Layer
    }, 
    to_layer_message,
};
use iced_runtime::Action;
use std::rc::{
    Rc, 
    Weak
};
use std::cell::RefCell;
use std::fmt::Debug;
use crate::{
    components::*, 
    *,
};



pub struct MainApp {
    self_ref: Option<Weak<RefCell<MainApp>>>, // weak ref
    windowed: bool,
    size: (u32, u32),
    dock: Dock,
    margin: (i32, i32, i32, i32), // top, right, bottom, left
    lmouse_down: bool,
    rmouse_down: bool,
    rmouse_start: Option<Point>,
    finger_presses: Vec<(u64, Point, Instant)>,
    current_view: View, // enum
    pub views: Vec<Box<dyn ViewTrait>>, // list of ViewTrait objects
    gesture_handler: GestureHandler,
}


#[to_layer_message]
#[derive(Debug, Clone)]
pub enum MainMessage {
    Debug(String),
    String(String),
    IcedEvent(Event),
    Index(usize),
    Dock(Dock),
    ChangeView(View),
    KeyEnter,
    KeyExit,
    KeyPress,
    KeyRelease,
}


impl MainApp {
    fn initialize_self_ref(&mut self, self_rc: &Rc<RefCell<Self>>) {
        let weak_self = Rc::downgrade(self_rc);
        self.self_ref = Some(weak_self);
    }

    fn current_view(&self) -> &Box<dyn ViewTrait> {
        self.views.iter().find(|view| view.class() == self.current_view).expect("No matching view found")
    }

    fn current_view_mut(&mut self) -> &mut Box<dyn ViewTrait> {
        self.views.iter_mut().find(|view| view.class() == self.current_view).expect("No matching view found")
    }



    fn move_window(&mut self, position: Point) -> Task<MainMessage> {
        // get windows initial position - the margin
        if self.rmouse_start.is_none() {
            self.rmouse_start = Some(position);
            info!("start: {:?}", self.rmouse_start.unwrap());
            return Task::none();
        }

        // calulate the difference
        let diff = self.rmouse_start.unwrap() - position;
        info!("diff: {:?} {:?}", -diff.x as i32, diff.y as i32);

        // calculate for the margin change
        let y = diff.y as i32 + self.margin.2;
        let x = -diff.x as i32 + self.margin.3;

        //info!("mar: {:?} {:?}", x as i32, y as i32);

        // store the mouse pos
        self.rmouse_start = Some(position);
        
        // apply margin to move window
        self.margin.2 = y;
        self.margin.3 = x;
        info!("mar: {:?} {:?}", x as i32, y as i32);
        return Task::done(MainMessage::MarginChange((0, 0, y, x)))

        //Task::none()
    }


    fn handle_input_event(&mut self, event: &Event) -> Task<MainMessage> {
        match event {
            //Event::Window(event) => todo!(),

            // keyboard
            Event::Keyboard(keyboard::Event::KeyPressed {
                key,
                ..
            }) => match key {
                iced::keyboard::Key::Named(Named::Escape) => {
                    return iced_runtime::task::effect(Action::Exit)
                }
                iced::keyboard::Key::Named(Named::Backspace) => {
                    // pop stack history
                }
                _ => {}
            }

            // mouse
            Event::Mouse(event) => {
                match event {
                    mouse::Event::ButtonPressed(button) => {
                        match button {
                            mouse::Button::Left => {
                                self.gesture_handler.start();
                                //self.gesture_handler.clear();
                                self.lmouse_down = true;
                            }
                            mouse::Button::Right => {
                                self.rmouse_start = None;
                                self.rmouse_down = true;
                            }
                            _ => {info!("Unhandled mouse button")}
                        }
                    }
                    mouse::Event::ButtonReleased(button) => {
                        match button {
                            mouse::Button::Left => {
                                self.gesture_handler.end();
                                self.lmouse_down = false;
                            }
                            mouse::Button::Right => {
                                self.rmouse_down = false;
                            }
                            _ => {info!("Unhandled mouse release")}
                        }
                    }
                    mouse::Event::CursorMoved { position } => {
                        if self.lmouse_down {
                            self.gesture_handler.append(*position);
                        }
                        if self.rmouse_down {
                            if self.rmouse_down {
                                return self.move_window(*position);
                            }
                        }
                    }
                    _ => {info!("Unhandled event")}
                    
                }
            }

            // touch
            Event::Touch(event) => {
                match event {
                    touch::Event::FingerPressed { id, position} => {
                        self.finger_presses.push((id.0, *position, Instant::now()));
                    }
                    touch::Event::FingerMoved { id, position} => {
                        if let Some((_, _, _)) = self.finger_presses.iter_mut().find(|(fid, _, _)| *fid == id.0) {
                            if id.0 == 1 {
                                info!("Finger 1 moved to: {position}");
                            }
                        }
                    }
                    touch::Event::FingerLifted { id, ..} | touch::Event::FingerLost { id, ..} => {
                        self.finger_presses.retain(|(fid, _, _)| *fid != id.0);
                        //todo
                    }
                    _ => {}
                }

                // Check for multiple finger presses
                if self.finger_presses.len() >= 2 {
                    // Get the timestamps of the two most recent finger presses
                    let (t1, t2) = {
                        let mut timestamps = self.finger_presses.iter().map(|(_, _, t)| t).collect::<Vec<_>>();
                        timestamps.sort();
                        (timestamps[0], timestamps[1])
                    };

                    // Check if the delay between the two finger presses is within a certain threshold
                    if t2.duration_since(*t1).as_millis() < 200 { // 200ms threshold
                        // Handle the multiple finger press event
                        info!("Multiple finger press detected!");
                    }
                }
            },
            _ => {}, // info!("event: {event:?}");
        }
        Task::none()
    }

    // handle layer shell settings
    pub fn default_layer_shell(_start_mode: StartMode) -> LayerShellSettings {
        let default = MainApp::default();
        // default free window mode
        LayerShellSettings {
            anchor: Anchor::Bottom | Anchor::Left, //| Anchor::Right,
            layer: Layer::Top, // Layer::Overlay if need to go the max
            exclusive_zone: -1,
            size: Some(default.size), //None,
            margin: default.margin,
            keyboard_interactivity: KeyboardInteractivity::OnDemand,
            events_transparent: false,
            start_mode: StartMode::default(),
        }
    }

}


impl Default for MainApp {
    /// Creates a default instance of [`MainWindow`].
    fn default() -> Self {
        Self {
            self_ref: None,
            windowed: true,
            size: (600, 250),
            dock: Dock::Top,
            margin: (0, 0, 0, 0),
            lmouse_down: false,
            rmouse_down: false,
            rmouse_start: None,
            finger_presses: Vec::new(),
            current_view: View::CompactQWERTY,
            views: View::init_views(),
            gesture_handler: GestureHandler::new(),
        }
    }
}


impl MainApp {
    pub fn new() -> (Self, Task<MainMessage>) {
        let default = Self::default();
        // create a weakreference to the main window
        let main = Rc::new(RefCell::new(default));
        main.borrow_mut().initialize_self_ref(&main);
        (Rc::try_unwrap(main).map_err(|_| panic!("Failed to unwrap Rc")).unwrap().into_inner(), Task::none())
    }


    pub fn view(&self) -> Element<MainMessage> {
        //info!("view draw");
        let has_gesture = self.current_view().has_gesture();
        match has_gesture {
            true => {
                return stack![
                    self.current_view().view(),
                    self.gesture_handler.view(),
                ]
                .into()
            }
            false => {
                return self.current_view().view()
            }
        }
    }


    pub fn update(&mut self, message: MainMessage) -> Task<MainMessage> {
        match message {
            MainMessage::Debug(s) => {
                info!("{s}");
            }
            MainMessage::ChangeView(view) => {
                info!("Change view to {view:?}");
                self.current_view = view;
            }
            MainMessage::String(s) => {
                info!("{s}");
            }
            MainMessage::IcedEvent(event) => {
                return self.handle_input_event(&event);
            }
            MainMessage::Dock(dock) => {
                self.dock = dock;
                match dock {
                    Dock::Left => {
                        return Task::done(MainMessage::AnchorSizeChange(
                        Anchor::Left | Anchor::Top | Anchor::Bottom,
                        (400, 0),
                        ))
                    }
                    Dock::Right => {
                        return Task::done(MainMessage::AnchorSizeChange(
                        Anchor::Right | Anchor::Top | Anchor::Bottom,
                        (400, 0),
                        ))
                    }
                    Dock::Bottom => {
                        return Task::done(MainMessage::AnchorSizeChange(
                        Anchor::Bottom | Anchor::Left | Anchor::Right,
                        (0, 400),
                        ))
                    }
                    Dock::Top => {
                        return Task::done(MainMessage::AnchorSizeChange(
                        Anchor::Top | Anchor::Left | Anchor::Right,
                        (0, 400),
                        ))
                    }
                }
            }
            _ => {
                return self.current_view_mut().update(message);
            }
        }
        Task::none()
    }


    pub fn style(&self, theme: &iced::Theme) -> iced_layershell::Appearance {
        iced_layershell::Appearance {
            //background_color: Color::TRANSPARENT,
            background_color: Color::from_rgba(0.21, 0.23, 0.25, 0.5),
            text_color: theme.palette().text,
        }
    }

    pub fn namespace(&self) -> String {
        String::from("surfboard")
    }

    pub fn subscription(&self) -> Subscription<MainMessage> {
        event::listen().map(MainMessage::IcedEvent)
        //event::listen_with(self.handle_input_event) // can try splitting this out?
    }

}