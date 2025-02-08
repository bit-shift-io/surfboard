use std::str::FromStr;
use std::path::PathBuf;
use pretty_ini::{ini, ini_file};
use iced::widget::{image, svg};
use iced::{Element, Length};
use crate::components::*;

use crate::*;

static DEFAULT_ICON: &[u8] = include_bytes!("../../res/ghost-solid.svg");

#[allow(unused)]
#[derive(Debug, Clone, Default)]
pub struct App {
    name: String,
    icon: Option<PathBuf>,
    executable: Option<PathBuf>, // Path to the .app file for mac, or Exec for Linux, or .exe for Windows
    desktop: PathBuf,     // Path to the .desktop file for Linux, .app for Mac
}


impl App {
    pub fn new(app_path: &str) -> Self {
        let app_path = PathBuf::from(app_path);
        let app = parse_desktop_file(app_path);
        app
    }

    pub fn launch(&self) {
        if let Some(executable) = self.executable.as_ref() {
            info!("Launching: {}", executable.display());
            let _ = std::process::Command::new(executable)
                        .spawn();
        }
    }

    fn icon(&self) -> Element<Message> {
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

    pub fn view(&self, index: usize) -> Element<main_app::Message> {
        Key::new(self.icon())
            //.on_press(main_app::Message::Debug("btn")) // main_app::Message::ViewHandler(view::Message::ViewMessage(index))
            .into()
    }
}


static ICONS_SIZE: &[&str] = &["256x256", "128x128"];
static THEMES_LIST: &[&str] = &["breeze", "Adwaita", "Qogir"];

fn get_icon_path_from_xdgicon(iconname: &str) -> Option<PathBuf> {
    let scalable_icon_path = xdg::BaseDirectories::with_prefix("icons/hicolor/scalable/apps").unwrap();
    if let Some(iconpath) = scalable_icon_path.find_data_file(format!("{iconname}.svg")) {
        return Some(iconpath);
    }
    for prefix in ICONS_SIZE {
        let iconpath = xdg::BaseDirectories::with_prefix(format!("icons/hicolor/{prefix}/apps")).unwrap();
        if let Some(iconpath) = iconpath.find_data_file(format!("{iconname}.png")) {
            return Some(iconpath);
        }
    }

    let pixmappath = xdg::BaseDirectories::with_prefix("pixmaps").unwrap();
    if let Some(iconpath) = pixmappath.find_data_file(format!("{iconname}.svg")) {
        return Some(iconpath);
    }
    if let Some(iconpath) = pixmappath.find_data_file(format!("{iconname}.png")) {
        return Some(iconpath);
    }

    for themes in THEMES_LIST {
        let iconpath = xdg::BaseDirectories::with_prefix(format!("icons/{themes}/apps/48")).unwrap();
        if let Some(iconpath) = iconpath.find_data_file(format!("{iconname}.svg")) {
            return Some(iconpath);
        }
        let iconpath = xdg::BaseDirectories::with_prefix(format!("icons/{themes}/apps/64")).unwrap();
        if let Some(iconpath) = iconpath.find_data_file(format!("{iconname}.svg")) {
            return Some(iconpath);
        }
        let iconpath = xdg::BaseDirectories::with_prefix(format!("icons/{themes}/scalable/apps")).unwrap();
        if let Some(iconpath) = iconpath.find_data_file(format!("{iconname}.svg")) {
            return Some(iconpath);
        }
    }

    None
}

fn get_icon_path(iconname: &str) -> Option<PathBuf> {
    if iconname.contains('/') {
        PathBuf::from_str(iconname).ok()
    } else {
        get_icon_path_from_xdgicon(iconname)
    }
}


pub fn load_ini(file_path: &str) -> ini::Ini {
    let mut file = ini_file::IniFile::default();
    file.set_path(file_path);

    let mut ini = ini::Ini::default();
    ini.load(&mut file).unwrap();
    ini
}


pub fn parse_desktop_file(desktop_file_path: PathBuf) -> App {
    let mut app = App::default();
    app.desktop = desktop_file_path.clone();
    let ini = load_ini(app.desktop.to_str().unwrap());

    match ini.get("Desktop Entry", "Name") {
        Ok(item) => app.name = item.value,
        Err(_) => {}
    }

    match ini.get("Desktop Entry", "Icon") {
        Ok(item) => app.icon = get_icon_path(&item.value),
        Err(_) => {}
    }

    match ini.get("Desktop Entry", "Exec") {
        Ok(item) => {
            // filter out the " %F" and " %U" etc from launch commands
            let exec = item.value.split_once(" ").map(|(first, _last)| first).unwrap_or(&item.value);
            app.executable = Some(PathBuf::from(exec));
        }
        Err(_) => {}
    }

    return app
}


// pub fn get_all_apps() -> Vec<App> {
//     // read XDG_DATA_DIRS env var
//     let xdg_data_dirs = std::env::var("XDG_DATA_DIRS").unwrap_or("/usr/share".to_string());
//     let xdg_data_dirs: Vec<&str> = xdg_data_dirs.split(':').collect();
//     // make a string sett from xdg_data_dirs
//     let mut search_dirs: HashSet<&str> = xdg_data_dirs.iter().cloned().collect();
//     search_dirs.insert("/usr/share/applications");
//     // get home dir of current user
//     let home_dir = std::env::var("HOME").unwrap();
//     let home_path = PathBuf::from(home_dir);
//     let local_share_apps = home_path.join(".local/share/applications");
//     search_dirs.insert(local_share_apps.to_str().unwrap());
//     search_dirs.insert("/usr/share/xsessions");
//     search_dirs.insert("/etc/xdg/autostart");
//     search_dirs.insert("/var/lib/snapd/desktop/applications");
//     // for each dir, search for .desktop files
//     let mut apps: Vec<App> = Vec::new();
//     for dir in search_dirs {
//         let dir = PathBuf::from(dir);
//         if !dir.exists() {
//             continue;
//         }
//         for entry in WalkDir::new(dir.clone()) {
//             if entry.is_err() {
//                 continue;
//             }
//             let entry = entry.unwrap();
//             let path = entry.path();
//             if path.extension().is_none() {
//                 continue;
//             }

//             if path.is_dir() {
//                 continue;
//             }

//             if path.extension().unwrap() == "desktop" {
//                 let app = parse_desktop_file(path.to_path_buf());
//                 apps.push(app);
//             }
//         }
//     }
//     apps
// }