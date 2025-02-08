use iced::{
    keyboard::{
        self, 
        key::Named
    }, 
    mouse, 
    time::Instant, 
    touch, 
    Event, 
    Point, 
    Task, 
};
use iced_runtime::Action;

use super::*;

pub struct InputHandler {
    pub lmouse_down: bool,
    pub rmouse_down: bool,
    pub rmouse_start: Option<Point>,
    pub finger_presses: Vec<(u64, Point, Instant)>,
}

impl InputHandler {
    pub fn new() -> Self {
        InputHandler {
            lmouse_down: false,
            rmouse_down: false,
            rmouse_start: None,
            finger_presses: Vec::new(),
        }
    }

    pub fn update(&mut self, event: &Event) -> Task<main_app::Message> {
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
                                Task::done(main_app::Message::GestureHandler(gesture::Message::Start))
                            }
                            // Task::perform(
                            //     async { self.lmouse_down = true }, 
                            //     |_| main_app::Message::GestureHandler(gesture::Message::Start
                            // )),
                            // mouse::Button::Right => {
                            //     self.rmouse_start = None;
                            //     self.rmouse_down = true;
                            //     Task::none()
                            // }
                            _ => Task::none()
                        }
                    }
                    mouse::Event::ButtonReleased(button) => {
                        match button {
                            mouse::Button::Left => {
                                self.lmouse_down = false;
                                Task::done(main_app::Message::GestureHandler(gesture::Message::End))
                            }
                            // mouse::Button::Right => {
                            //     self.rmouse_down = false;
                            //     Task::none()
                            // }
                            _ => Task::none()
                        }
                    }
                    mouse::Event::CursorMoved { position } => {
                        if self.lmouse_down {
                            return Task::done(main_app::Message::GestureHandler(gesture::Message::Move(*position)))
                        }
                        // if self.rmouse_down {
                        //     return Task::done(main_app::Message::GestureHandler(gesture::Message::Move(*position)))
                        //     return self.move_window(*position);
                        // }
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
                Task::none()
            },
            _ => Task::none(),
        }
    }


}