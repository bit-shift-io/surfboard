#[macro_use]
extern crate log;

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

    // in the future, the user may want to start the dock as the default, or the keyboard via cmdline
    let start_mode = handle_args();

    MainApp::run(Settings {
        layer_settings: MainApp::layer_shell_default(start_mode),
        ..Default::default()
    })
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