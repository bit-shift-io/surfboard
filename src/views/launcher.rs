use iced::{
    widget::row, 
    Element, 
    Length, 
    Task
};
use crate::app::*;
use crate::components::*;


#[derive(Debug, Clone)]
pub struct LauncherView {
    applications: Vec<App>,
}


impl ViewTrait for LauncherView {
    fn new() -> Self {
        // add apps here
        let applications = vec![
            App::new("/usr/share/applications/code.desktop"),
            App::new("/usr/share/applications/org.kde.konsole.desktop"),
            App::new("/usr/share/applications/firefox.desktop"),
        ];

        info!("Applications: {:?}", applications);

        LauncherView {
            applications,
        }
    }

    fn view(&self) -> Element<main_app::Message> {
        let bottom_vec: Vec<Element<main_app::Message>> = 
            self.applications
            .iter()
            .enumerate()
            .map(|(filter_index, app)| app.view(filter_index))
            .collect();

        row(bottom_vec).width(Length::Fill).into()
    }
    
    fn class(&self) -> View {
        View::Launcher
    }
    
    fn update(&mut self, message: view::Message) -> Task<main_app::Message> {
        match message {
            view::Message::ViewMessage(index) => {
                // optionally we should have each app with an on_pressed? custom widget is needed then?
                self.applications[index].launch();
            }
            _ => {}
        }
        
        Task::none()
    }
}

