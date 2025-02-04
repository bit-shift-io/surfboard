use iced::{
    widget::{
        column, 
        row,
    }, Element, Length, Task
};
use iced_runtime::Action;
use crate::app::*;
use crate::components::*;


#[derive(Debug, Clone)]
pub struct ApplicationLauncherView {
    applications: Vec<App>,
}


impl ViewTrait for ApplicationLauncherView {
    fn new() -> Self {
        // add apps here
        let applications = vec![
            App::new("/usr/share/applications/code.desktop"),
            App::new("/usr/share/applications/org.kde.konsole.desktop"),
            App::new("/usr/share/applications/firefox.desktop"),
        ];

        info!("Applications: {:?}", applications);

        ApplicationLauncherView {
            applications,
        }
    }

    fn view(&self) -> Element<MainMessage> {

        let bottom_vec: Vec<Element<MainMessage>> = 
            self.applications
            .iter()
            .enumerate()
            .map(|(filter_index, app)| app.view(filter_index, false))
            .collect();

        row(bottom_vec).width(Length::Fill).into()
        //column![bottom_vec].into()

    }
    
    fn name(&self) -> String {
        String::from("launcher")
    }
    
    fn class(&self) -> View {
        View::ApplicationLauncher
    }
    
    fn has_gesture(&self) -> bool {
        true
    }
    
    fn update(&mut self, message: MainMessage) -> Task<MainMessage> {
        match message {
            MainMessage::Index(index) => {
                // optionally we should have each app with an on_pressed? custom widget is needed then?
                info!("click task!");
                self.applications[index].launch();
                //iced_runtime::task::effect(Action::Exit)
                //return Task::none()
                //return Task::done(MainMessage::Launch(index))
            }
            _ => {}
        }
        return Task::none()
    }
}

