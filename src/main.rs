#![deny(unused)]

mod components;
use clap::Parser;
use components::cli;
use components::cli::*;
use components::{module_loader, startup_anim};

mod utils;
use utils::config;

use colored::Colorize;
use log::{error, info};
use std::process;

#[tokio::main]
async fn main() {
    logger::init().unwrap();
    println!("{}", cli::colored_banner());
    info!("Starting up...");
    // TODO: startup animation actually waits for modules to load and gets callback to stop animating
    // TODO also could use ratatui for this
    startup_anim::animate();
    module_loader::init();

    let cli = Cli::parse();

    let servx_thread = servx::init();

    let tray_thread = components::tray::init();

    let voice_recognizer_thread = tokio::spawn(async {
        let config = if let Some(path) = cli.config {
            config::init().custom(&path).init()
        } else {
            config::init().filename("nekosys_config").init()
        };

        let voice_model = if !cli.model.is_none() {
            cli.model.unwrap()
        } else {
            config.read_key(|k| k.voice_model.clone())
        };

        // TODO: move this to the crate itself
        if voice_model.is_empty() {
            error!("Please specify the voice model path via -m or in config!")
        } else {
            voice_recognizer::init(voice_model).await.unwrap();
        }
    });

    tokio::select! {
        _ = tokio::signal::ctrl_c() => {
            // TODO maybe callback to modules to exit?
            info!("{}", "Goodbye /ᐠ • ᴖ •マ Ⳋ".red());
        }
        _ = servx_thread => {}
        _ = tray_thread => {}
        _ = voice_recognizer_thread => {}
    }

    process::exit(0);
}
