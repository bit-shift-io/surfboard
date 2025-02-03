use std::path::PathBuf;
use std::str::FromStr;

use iced::widget::{button, column, image, row, svg, text};
use iced::Pixels;
use iced::{Element, Length};
use applications::{AppInfoContext, AppInfo};

use crate::*;

static DEFAULT_ICON: &[u8] = include_bytes!("../../res/ghost-solid.svg");

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct App {
    name: String,
    icon: Option<PathBuf>,
}

impl App {
    pub fn launch(&self) {

    }

    pub fn title(&self) -> &str {
        &self.name
    }

    fn icon(&self) -> Element<MainMessage> {
        match &self.icon {
            Some(path) => {
                if path
                    .as_os_str()
                    .to_str()
                    .is_some_and(|pathname| pathname.ends_with("png"))
                {
                    image(image::Handle::from_path(path))
                        .width(Length::Fixed(80.))
                        .height(Length::Fixed(80.))
                        .into()
                } else {
                    svg(svg::Handle::from_path(path))
                        .width(Length::Fixed(80.))
                        .height(Length::Fixed(80.))
                        .into()
                }
            }
            None => svg(svg::Handle::from_memory(DEFAULT_ICON))
                .width(Length::Fixed(80.))
                .height(Length::Fixed(80.))
                .into(),
        }
    }

    pub fn description(&self) -> &str {
        ""
    }

    pub fn view(&self, index: usize, selected: bool) -> Element<MainMessage> {
        button(
            row![
                self.icon(),
                column![
                    text(self.title()).size(Pixels::from(20)),
                    text(self.description()).size(Pixels::from(10))
                ]
                .spacing(4)
            ]
            .spacing(10),
        )
        .on_press(MainMessage::Launch(index))
        .width(Length::Fill)
        .height(Length::Fixed(85.))
        .style(move |theme, status| {
            if selected {
                button::primary(theme, status)
            } else {
                button::secondary(theme, status)
            }
        })
        .into()
    }
}

static ICONS_SIZE: &[&str] = &["256x256", "128x128"];

static THEMES_LIST: &[&str] = &["breeze", "Adwaita"];

fn get_icon_path_from_xdgicon(iconname: &str) -> Option<PathBuf> {
    None
}

fn get_icon_path(iconname: &str) -> Option<PathBuf> {
    if iconname.contains('/') {
        PathBuf::from_str(iconname).ok()
    } else {
        get_icon_path_from_xdgicon(iconname)
    }
}

pub fn all_apps() -> Vec<App> {
    let mut ctx = AppInfoContext::new();
    ctx.refresh_apps().unwrap(); // must refresh apps before getting them

    let apps = ctx.get_all_apps();
    info!("Apps: {:#?}", apps);

    let frontmost_app = ctx.get_frontmost_application().unwrap();
    info!("Frontmost App: {:#?}", frontmost_app);

    let running_apps = ctx.get_running_apps();
    info!("Running Apps: {:#?}", running_apps);

    Vec::new()

    // let re = regex::Regex::new(r"([a-zA-Z]+);").unwrap();
    // gio::AppInfo::all()
    //     .iter()
    //     .filter(|app| app.should_show() && app.downcast_ref::<gio::DesktopAppInfo>().is_some())
    //     .map(|app| app.clone().downcast::<gio::DesktopAppInfo>().unwrap())
    //     .map(|app| App {
    //         appinfo: app.clone(),
    //         name: app.name().to_string(),
    //         descriptions: app.description(),
    //         categrades: match app.categories() {
    //             None => None,
    //             Some(categrades) => {
    //                 let tomatch = categrades.to_string();
    //                 let tips = re
    //                     .captures_iter(&tomatch)
    //                     .map(|unit| unit.get(1).unwrap().as_str().to_string())
    //                     .collect();
    //                 Some(tips)
    //             }
    //         },
    //         actions: {
    //             let actions = app.list_actions();
    //             if actions.is_empty() {
    //                 None
    //             } else {
    //                 Some(actions)
    //             }
    //         },
    //         icon: match &app.icon() {
    //             None => None,
    //             Some(icon) => {
    //                 let iconname = gio::prelude::IconExt::to_string(icon).unwrap();
    //                 get_icon_path(iconname.as_str())
    //             }
    //         },
    //     })
    //     .collect()
}
