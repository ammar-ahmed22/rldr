mod args;
mod command;
mod io;
mod logger;

use anyhow::Result;
use clap::Parser;
use command::CommandRunner;
use io::input_listener;
use log::{info, warn};
use args::Args;
use logger::init_logger;

pub struct Reloader;

impl Reloader {
    pub fn new() -> Self {
        return Self{}
    }

    pub fn run(&self) -> Result<()> {
        init_logger()?;
        let args = Args::parse();
        let runner = CommandRunner::new();
        let listener = input_listener();

        let cmd_str = format!("{}", args.command.join(" "));
        info!("Starting `{}`", cmd_str);
        runner.start(&args.command)?;
        for input in listener {
            match input {
                'r' => {
                    info!("Reloading `{}`", cmd_str);
                    runner.start(&args.command)?;
                },
                'c' => {
                    warn!("Closing `{}`", cmd_str);
                    runner.stop()?;
                    warn!("Closed `{}`", cmd_str);
                },
                'q' => {
                    warn!("Quitting.");
                    runner.stop()?;
                    break;
                },
                _ => {}
            }
        }

        return Ok(());
    }
}