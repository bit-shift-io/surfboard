use iced::{
    keyboard::{
        self, 
        key::Named
    }, 
    mouse, 
    time::{
        self, 
        Instant, 
        Duration
    }, 
    touch, 
    Event, 
    Point, 
    Subscription, 
    Task 
};
use iced_runtime::Action;
use super::*;

static LONG_PRESS_DURATION: Duration = Duration::from_millis(500);

#[derive(Debug, Clone)]
pub enum Message {
    Tick,
}

#[derive(Clone, Debug)]
pub struct InputHandler {
    lmouse_down: bool,
    rmouse_down: bool,
    finger_presses: Vec<(u64, Point, Instant)>, // id, pos, time
    timer_enabled: bool,
    timer_start: Option<Instant>,
}

impl InputHandler {
    pub fn new() -> Self {
        InputHandler {
            lmouse_down: false,
            rmouse_down: false,
            finger_presses: Vec::new(),
            timer_enabled: false,
            timer_start: None
        }
    }

    pub fn update<'a>(&mut self, message: Message) -> Task<main_app::Message> {
        match message {
            Message::Tick => {
                info!("input tick");
                if !self.timer_enabled {
                    return Task::none()
                }
                let duration = Instant::now().duration_since(self.timer_start.unwrap());
                if duration >= LONG_PRESS_DURATION {
                    self.timer_end();
                    return Task::done(view::Message::ActionGesture(ActionDirection::LongPress)).map(main_app::Message::ViewHandler);
                }
                Task::none()
            },
            //_ => Task::none()
        }
    }

    pub fn update2<'a>(&mut self, event: &Event, gesture_handler: &mut GestureHandler, window_handler: &mut WindowHandler) -> Task<main_app::Message> {
        match event {
            //Event::Window(event) => todo!(),

            // keyboard
            Event::Keyboard(keyboard::Event::KeyPressed {key, ..}) => match key {
                iced::keyboard::Key::Named(Named::Escape) => iced_runtime::task::effect(Action::Exit),
                iced::keyboard::Key::Named(Named::Backspace) => Task::none(),
                _ => Task::none()
            }

            // mouse
            Event::Mouse(event) => {
                match event {
                    mouse::Event::ButtonPressed(button) => {
                        match button {
                            mouse::Button::Left => {
                                self.lmouse_down = true;                        
                                self.timer_start();
                                return gesture_handler.start();
                            }
                            mouse::Button::Right => {
                                self.rmouse_down = true;
                                return window_handler.start_move();
                            }
                            _ => Task::none()
                        }
                    }
                    mouse::Event::ButtonReleased(button) => {
                        match button {
                            mouse::Button::Left => {
                                self.lmouse_down = false;
                                return gesture_handler.end();
                            }
                            mouse::Button::Right => {
                                self.rmouse_down = false;
                                return window_handler.end_move();
                            }
                            _ => Task::none()
                        }
                    }
                    mouse::Event::CursorMoved { position } => {
                        self.timer_end();

                        if self.lmouse_down {
                            return gesture_handler.append(*position);
                        }
                        if self.rmouse_down {
                            return window_handler.append_move(*position);
                        }
                        Task::none()
                    }
                    _ => Task::none()
                    
                }
            }

            // touch
            Event::Touch(event) => {
                match event {
                    touch::Event::FingerPressed { id, position} => {
                        self.finger_presses.push((id.0, *position, Instant::now()));
                        self.timer_start();

                        // todo: if gesture has started and multiple fingers pressed, cancel the gesture.
                        // todo: then if multiple fingers pressed we want to move the window instead
                           
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

                        return gesture_handler.start();
                    }
                    touch::Event::FingerMoved { id, position} => {
                        self.timer_end();
                        
                        if let Some((_, _, _)) = self.finger_presses.iter_mut().find(|(fid, _, _)| *fid == id.0) {
                            if id.0 == 1 {
                                info!("Finger 1 moved to: {position}");
                            }
                        }
                        return gesture_handler.append(*position);
                    }
                    touch::Event::FingerLifted { id, ..} | touch::Event::FingerLost { id, ..} => {
                        self.finger_presses.retain(|(fid, _, _)| *fid != id.0);
                        return gesture_handler.end();
                        // todo check for long press single finger
                        // todo check fo release of second finger - right click
                    }
                    //_ => {}
                }
            },
            _ => Task::none(),
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        match self.timer_enabled {
            true => time::every(Duration::from_millis(100)).map(|_| Message::Tick), // every function not found in iced::time?!
            false => Subscription::none()
        }
    }

    pub fn timer_start(&mut self) {
        if !self.timer_enabled {
            self.timer_start = Some(Instant::now());
            self.timer_enabled = true;
        }
    }

    pub fn timer_end(&mut self) {   
        self.timer_enabled = false;
        self.timer_start = None;
    }
}