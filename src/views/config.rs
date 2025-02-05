use iced::{
    widget::{
        button, 
        pick_list, 
        row, 
        Button, 
        PickList, 
        Text}, Element, Length, Renderer, Task, Theme
    };
use crate::app::*;


#[derive(Debug, Clone)]
pub struct ConfigurationView;


impl ViewTrait for ConfigurationView {
    fn new() -> Self where Self: Sized {
        ConfigurationView{}
    }

    fn view(&self) -> Element<MainMessage> {
        let view_main = Button::new(Text::new("main")).on_press(MainMessage::ChangeView(View::CompactQWERTY));
        let view_launcher = Button::new(Text::new("launcher")).on_press(MainMessage::ChangeView(View::Launcher));
        
        // let views = self.get_main_window().borrow().views;
        // let pick_view: PickList<'_, View, &[View], View, MainMessage, Theme, Renderer> = pick_list(
        //     &View::ALL[..], //this requires an enum??
        //     None,
        //     MainMessage::ChangeView,
        // )
        // .placeholder("View");

        let pick_dock: PickList<'_, Dock, &[Dock], Dock, MainMessage, Theme, Renderer> = pick_list(
            &Dock::ALL[..],
            None,
            MainMessage::Dock,
        )
        .placeholder("Edge");

        row![
            pick_dock,
            //pick_view,
            view_main,
            view_launcher,
            //menu,
        ]
            .padding(20)
            .width(Length::Fill)
            .into()
    }

    fn class(&self) -> View {
        View::Configuration
    }
    
    fn has_gesture(&self) -> bool {
        false
    }
    
    fn update(&mut self, message: MainMessage) -> Task<MainMessage> {
        Task::none()
    }

    // fn name(&self) -> &str {
    //     "Settings"
    // }
}