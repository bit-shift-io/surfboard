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
static MOVE_THRESHOLD: f32 = 10.0;

#[derive(Debug, Clone)]
pub enum Message {
    LongPressTick,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PressType {
    None,
    Tap,
    LongPress,
    Gesture,
}

/// Handles the user inputs.  
#[derive(Clone, Debug)]
pub struct InputHandler {
    left_mouse: PressType,
    rmouse_down: bool,
    finger_presses: Vec<(u64, Point, Instant)>, // id, pos, time
    long_press_timer_enabled: bool,
    start_cursor_position: Option<Point>, // start position of press
    last_cursor_position: Option<Point>, // last cursor position, so we know position of click
    cursor_position: Option<Point>, // cursor position, so we know position of click, bug in iced not giving the point on click!
}

/// Notes on how this works:
/// - The InputHandler is responsible for handling all user inputs.
/// - When the user taps the screen, the InputHandler will start a timer, and store a start position.
/// - If the user lifts their finger before the timer ends, without moving, the InputHandler will consider it a tap.
/// - If the user moves their finger before the timer ends, the InputHandler will consider it a gesture.
/// - If the user presses their finger for a long time, without moving, the InputHandler will consider it a long press.
impl InputHandler {
    pub fn new() -> Self {
        InputHandler {
            left_mouse: PressType::None,
            rmouse_down: false,
            finger_presses: Vec::new(),
            long_press_timer_enabled: false,
            start_cursor_position: None,
            last_cursor_position: None,
            cursor_position: None,
        }
    }

    pub fn update<'a>(&mut self, message: Message) -> Task<main_app::Message> {
        match message {
            Message::LongPressTick => {
                // will only tick at long press duration
                if !self.long_press_timer_enabled {
                    return Task::none()
                }
                self.long_press_timer_enabled = false;
                self.left_mouse = PressType::LongPress;
                return Task::done(view::Message::ActionGesture(ActionDirection::LongPress)).map(main_app::Message::ViewHandler);
            },
        }
    }

    pub fn update_event<'a>(&mut self, event: &Event, gesture_handler: &mut GestureHandler, window_handler: &mut WindowHandler) -> Task<main_app::Message> {
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
                    // button pressed
                    mouse::Event::ButtonPressed(button) => {
                        match button {
                            mouse::Button::Left => {
                                self.left_mouse = PressType::Tap;                
                                self.long_press_timer_enabled = true;
                                self.start_cursor_position = self.cursor_position;
                                return Task::none()
                            }
                            mouse::Button::Right => {
                                self.rmouse_down = true;
                                return window_handler.start_move();
                            }
                            _ => Task::none()
                        }
                    }

                    // cursor moved
                    mouse::Event::CursorMoved { position } => {
                        // store the new cursor position
                        self.cursor_position = Some(*position);

                        // left mouse movement
                        match self.left_mouse {
                            PressType::Tap => {
                                // check if the cursor has moved above the threshold
                                // do nothing if the cursor has not moved above the threshold
                                let is_above_move_threshold = self.is_above_move_threshold();
                                if !is_above_move_threshold {
                                    return Task::none()
                                }

                                // reset timer
                                self.long_press_timer_enabled = false;
                                self.left_mouse = PressType::Gesture;
                                info!("Gesture started!");
                                // add start position to gesture handler
                                let _ = gesture_handler.start();
                                if self.last_cursor_position.is_some() {
                                    info!("last position: {position}");
                                    let _ = gesture_handler.update_move(self.last_cursor_position.unwrap());
                                }
                                // and current position
                                return gesture_handler.update_move(*position);
                            }
                            PressType::Gesture => {
                                info!("Gesture moved to: {position}");
                                return gesture_handler.update_move(*position);
                            }
                            _ => {}
                            
                        }

                        // right mouse movement
                        if self.rmouse_down {
                            return window_handler.update_move(*position);
                        }

                        Task::none()
                    }

                    // button released
                    mouse::Event::ButtonReleased(button) => {
                        match button {
                            mouse::Button::Left => {
                                let mut result = Task::none();
                                match self.left_mouse {
                                    PressType::Gesture => { result = gesture_handler.end(); }
                                    _ => {}
                                }
                                // reset the state
                                self.left_mouse = PressType::None;
                                self.long_press_timer_enabled = false;
                                self.last_cursor_position = None;
                                return result;
                            }
                            mouse::Button::Right => {
                                self.rmouse_down = false;
                                return window_handler.end_move();
                            }
                            _ => Task::none()
                        }
                    }

                    _ => Task::none()
                    
                }
            }

            // touch
            Event::Touch(event) => {
                match event {
                    touch::Event::FingerPressed { id, position} => {
                        self.finger_presses.push((id.0, *position, Instant::now()));
                        self.long_press_timer_enabled = true;

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
                        self.long_press_timer_enabled = false;
                        
                        if let Some((_, _, _)) = self.finger_presses.iter_mut().find(|(fid, _, _)| *fid == id.0) {
                            if id.0 == 1 {
                                info!("Finger 1 moved to: {position}");
                            }
                        }
                        return gesture_handler.update_move(*position);
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
        match self.long_press_timer_enabled {
            true => time::every(LONG_PRESS_DURATION).map(|_| Message::LongPressTick),
            false => Subscription::none()
        }
    }

    /// Returns true if the distance between the last cursor position and the current cursor position is larger than
    /// MOVE_THRESHOLD. Updates the last cursor position to the current position. If the last cursor position is None,
    /// it sets it to the current cursor position and returns false.
    fn is_above_move_threshold(&mut self) -> bool {
        if self.last_cursor_position.is_none() {
            self.last_cursor_position = self.cursor_position;
            return false
        }
        let last_cursor_position = self.last_cursor_position.unwrap();
        let distance = self.start_cursor_position.unwrap().distance(last_cursor_position);
        let result = distance > MOVE_THRESHOLD;
        // only update until this changes to true
        if result {
            return result
        }
        self.last_cursor_position = self.cursor_position;
        return result
    }
}