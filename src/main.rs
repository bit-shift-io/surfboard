#[macro_use]
extern crate log;

use iced_layershell::reexport::Anchor;
use iced_layershell::settings::{LayerShellSettings, Settings, StartMode};
use iced_layershell::Application;

mod app;
mod utils;
mod views;
mod components;
use utils::*;
use app::*;


pub fn main() -> Result<(), iced_layershell::Error> {
    functions::init_env_var();
    functions::init_logger();
    info!("== Start Surfboard ==");

    let start_mode = handle_args();

    // window kind of mode?
    MainWindow::run(Settings {
        layer_settings: LayerShellSettings {
            size: Some((600, 250)),
            //exclusive_zone: 250,
            anchor: Anchor::Bottom | Anchor::Right,
            start_mode,
            margin: (10, 10, 10, 10),
            ..Default::default()
        },
        ..Default::default()
    })

    // docked mode
    // MainWindow::run(Settings {
    //     layer_settings: LayerShellSettings {
    //         size: Some((100, 200)),
    //         exclusive_zone: 200,
    //         anchor: Anchor::Bottom | Anchor::Left | Anchor::Right,
    //         start_mode,
    //         ..Default::default()
    //     },
    //     ..Default::default()
    // })
}


fn handle_args() -> StartMode {
    let args: Vec<String> = std::env::args().collect();
    let mut binded_output_name = None;
    if args.len() >= 2 {
        binded_output_name = Some(args[1].to_string())
    }
    let start_mode = match binded_output_name {
        Some(output) => StartMode::TargetScreen(output),
        None => StartMode::Active,
    };
    start_mode
}