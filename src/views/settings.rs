use iced::{
    widget::{
        button, 
        pick_list, 
        row, 
        Button, 
        PickList, 
        Text}, 
        Length, 
        Renderer, 
        Theme
    };
use crate::app::*;


#[derive(Debug, Clone)]
pub struct SettingsView;


impl ViewTrait for SettingsView {
    fn new() -> Self where Self: Sized {
        SettingsView{}
    }

    fn view(&self) -> iced::Element<MainMessage> {

        let view_main = Button::new(Text::new("main")).on_press(MainMessage::ChangeView(View::Main));
        
        let view_launcher = Button::new(Text::new("launcher")).on_press(MainMessage::ChangeView(View::ApplicationLauncher));
        



        let pick_screen_edge: PickList<'_, ScreenEdge, &[ScreenEdge], ScreenEdge, MainMessage, Theme, Renderer> = pick_list(
            &ScreenEdge::ALL[..],
            None,
            MainMessage::ChangeScreenEdge,
        )
        .placeholder("Edge");

        row![
            button("Tab").on_press(MainMessage::Debug("tab".into())),
            button("q").on_press(MainMessage::Debug("q".into())),
            button("w").on_press(MainMessage::Debug("w".into())),
            button("e").on_press(MainMessage::Debug("e".into())),
            pick_screen_edge,
            view_main,
            view_launcher,
            //menu,
        ]
        .padding(20)
        .width(Length::Fill)
        .into()
    }

    fn name(&self) -> String {
        String::from("settings")
    }
    
    fn class(&self) -> View {
        View::Settings
    }
    
    fn has_gesture(&self) -> bool {
        false
    }
}