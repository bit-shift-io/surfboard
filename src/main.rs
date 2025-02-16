#[macro_use]
extern crate log;
use iced::Task;
use iced_layershell::{
    build_pattern::application,
    settings::{LayerShellSettings, StartMode},
};

mod app;
mod components;
mod utils;
mod views;
use app::*;
use utils::*;

pub fn main() -> iced_layershell::Result {
    functions::init_env_var();
    functions::init_logger();
    info!("== Start Surfboard ==");

    // in the future, the user may want to start the dock as the default, or the keyboard via cmdline
    let start_mode = handle_args();

    // now we can put whatever we want into app!
    let mut app = MainApp::new();
    app.0.init();

    application(MainApp::namespace, MainApp::update, MainApp::view)
        .layer_settings(MainApp::default_layer_shell(start_mode))
        .style(MainApp::style)
        .default_text_size(iced::Pixels(18.0))
        .subscription(MainApp::subscription)
        .run_with(move || app)
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
