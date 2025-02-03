use std::collections::VecDeque;
use iced::{
    event, futures::future::ok, mouse, time::Instant, touch, widget::{
        stack, 
        Canvas,
    }, Color, Element, Event, Length, Point, Subscription, Task, Theme
};
use iced_layershell::{
    actions::LayershellCustomActions, application, reexport::{Anchor, KeyboardInteractivity, Layer}, to_layer_message, Application
};
use crate::{
    components::*, 
    views::*,
    *,
};


pub struct MainWindow {
    windowed: bool,
    screen_edge: ScreenEdge,
    margin: (i32, i32, i32, i32), // top, right, bottom, left
    theme: iced::Theme,
    dark_mode: bool,
    lmouse_down: bool,
    rmouse_down: bool,
    rmouse_start: Option<Point>,
    finger_presses: Vec<(u64, Point, Instant)>,
    current_view: View, // enum
    views: Vec<Box<dyn ViewTrait>>, // list of ViewTrait objects
    gesture_data: VecDeque<GestureData>,
}


#[to_layer_message]
#[derive(Debug, Clone)]
pub enum MainMessage {
    Debug(String),
    StringMessage(String),
    IcedEvent(Event),
    Launch(usize),
    ChangeScreenEdge(ScreenEdge),
    ChangeView(View),
    KeyEnter,
    KeyExit,
    KeyPress,
    KeyRelease,
}


#[derive(Clone, Copy, Debug, PartialEq)]
pub enum View {
    Main,
    Configuration,
    ApplicationLauncher,
    // Add more views/layouts here
}


impl MainWindow {
    fn current_view(&self) -> &Box<dyn ViewTrait> {
        self.views.iter().find(|view| view.class() == self.current_view).expect("No matching view found")
    }

    fn current_view_mut(&mut self) -> &mut Box<dyn ViewTrait> {
        self.views.iter_mut().find(|view| view.class() == self.current_view).expect("No matching view found")
    }

    fn push_gesture_data(&mut self, position: Point) {
        // debug print out the points
        info!("\nGesture Data:");
        self.gesture_data.iter().for_each(|item| info!("{:?}", item));

        if self.gesture_data.len() > 1 {
            // distance check with the back item
            let prev = self.gesture_data.back().unwrap();
            let distance = Point::distance(&prev.point, position);
            if distance < 20.0 {
                return;
            }
            
            // time check
            // remove the front items
            while let Some(item) = self.gesture_data.front() {
                let elapsed = Instant::now().duration_since(item.instant);
                if elapsed.as_millis() > 1000 { // 2 seconds
                    self.gesture_data.pop_front();
                } else {
                    break;
                }
            }

        }

        // round off the position
        let point = Point::new(position.x.round(), position.y.round());

        // add data to the back
        self.gesture_data.push_back(GestureData {
            point,
            instant: Instant::now(),
        });
    }


    fn move_window(&mut self, position: Point) -> Task<<main_window::MainWindow as iced_layershell::Application>::Message> {
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


    fn handle_input_event(&mut self, event: &Event) -> Task<<MainWindow as iced_layershell::Application>::Message> {

        match event {
            Event::Mouse(event) => {
                match event {
                    mouse::Event::ButtonPressed(button) => {
                        match button {
                            mouse::Button::Left => {
                                self.gesture_data.clear();
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
                            self.push_gesture_data(*position);
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
            //Event::Keyboard(event) => todo!(),
            //Event::Window(event) => todo!(),
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
                    touch::Event::FingerLifted { id, position} | touch::Event::FingerLost { id, position} => {
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
    pub fn layer_shell_settings(start_mode: StartMode) -> LayerShellSettings {
        let default = MainWindow::default();
        // default free window mode
        LayerShellSettings {
            anchor: Anchor::Bottom | Anchor::Left, //| Anchor::Right,
            layer: Layer::Top, // Layer::Overlay if need to go the max
            exclusive_zone: -1,
            size: Some((600, 250)), //None,
            margin: default.margin,
            keyboard_interactivity: KeyboardInteractivity::OnDemand,
            events_transparent: false,
            start_mode: StartMode::default(),
        }
    }
}


impl Default for MainWindow {
    /// Creates a default instance of [`MainWindow`].
    fn default() -> Self {

        // Add more views/layouts here
        let views: Vec<Box<dyn ViewTrait>> = vec![
            Box::new(MainView::new()),
            Box::new(ConfigurationView::new()),
            Box::new(ApplicationLauncherView::new()),
        ];

        // Return a default instance of MainWindow
        Self {
            windowed: true,
            screen_edge: ScreenEdge::Top,
            margin: (0, 0, 0, 0),
            theme: iced::Theme::Light,
            dark_mode: true,
            lmouse_down: false,
            rmouse_down: false,
            rmouse_start: None,
            finger_presses: Vec::new(),
            current_view: View::Main,
            views,
            gesture_data: VecDeque::new(),
        }
    }
}


impl Application for MainWindow {
    type Message = MainMessage;
    type Flags = ();
    type Theme = Theme;
    type Executor = iced::executor::Default;

    /// Create a new instance of [`MainWindow`].
    fn new(_flags: ()) -> (Self, Task<Self::Message>) {
        let default_window = Self::default();
        //default_window.windowed = true;
        (default_window, Task::none())
    }

    fn view(&self) -> Element<Self::Message> {
        let has_gesture = self.current_view().has_gesture();
        match has_gesture {
            true => {
                return stack![
                    self.current_view().view(),
                    Canvas::new(Gesture::new(&self.gesture_data)).width(Length::Fill).height(Length::Fill)
                ]
                .into()
            }
            false => {
                return self.current_view().view()
            }
        }
    }


    fn update(&mut self, message: Self::Message) -> Task<Self::Message> {
        match message {
            MainMessage::Debug(s) => {
                info!("{s}");
            }
            MainMessage::ChangeView(view) => {
                self.current_view = view;
            }
            MainMessage::StringMessage(s) => {
                info!("{s}");
            }
            MainMessage::IcedEvent(event) => {
                //info!("iced: {event:?}");
                return self.handle_input_event(&event);
            }
            MainMessage::Launch(_) => {
                return self.current_view_mut().update(message);
            }
            MainMessage::ChangeScreenEdge(screen_edge) => {
                self.screen_edge = screen_edge;
                match screen_edge {
                    ScreenEdge::Left => {
                        return Task::done(MainMessage::AnchorSizeChange(
                        Anchor::Left | Anchor::Top | Anchor::Bottom,
                        (400, 0),
                        ))
                    }
                    ScreenEdge::Right => {
                        return Task::done(MainMessage::AnchorSizeChange(
                        Anchor::Right | Anchor::Top | Anchor::Bottom,
                        (400, 0),
                        ))
                    }
                    ScreenEdge::Bottom => {
                        return Task::done(MainMessage::AnchorSizeChange(
                        Anchor::Bottom | Anchor::Left | Anchor::Right,
                        (0, 400),
                        ))
                    }
                    ScreenEdge::Top => {
                        return Task::done(MainMessage::AnchorSizeChange(
                        Anchor::Top | Anchor::Left | Anchor::Right,
                        (0, 400),
                        ))
                    }
                }
            }
            _ => {
                // Handle layout-specific messages
                //let view = self.current_view_mut();
                //let boxed_message = view.convert_message(message);
                //view.update(&boxed_message);
            }
        }
        Task::none()
    }


    fn style(&self, theme: &Self::Theme) -> iced_layershell::Appearance {
        iced_layershell::Appearance {
            //background_color: Color::TRANSPARENT,
            background_color: Color::from_rgba(0.21, 0.23, 0.25, 0.5),
            text_color: theme.palette().text,
        }
    }

    fn namespace(&self) -> String {
        String::from("surfboard")
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        event::listen().map(MainMessage::IcedEvent)
            //.map(MainMessage::MarginChange(()))
    }
    
    fn theme(&self) -> Self::Theme {
        Self::Theme::default()
    }
    
    fn scale_factor(&self) -> f64 {
        1.0
    }


}