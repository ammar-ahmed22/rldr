mod args;
mod command;
mod io;
mod logger;

use anyhow::Result;
use args::Args;
use clap::Parser;
use colored::*;
use command::CommandRunner;
use io::input_listener;
use log::{error, info, warn};
use logger::init_logger;

pub struct Reloader;

impl Reloader {
    pub fn new() -> Self {
        return Self {};
    }

    pub fn run(&self) -> Result<()> {
        init_logger()?;
        let args = Args::parse();
        let runner = CommandRunner::new();
        let listener = input_listener();

        let cmd_str = format!("{}", args.command.join(" ")).purple();
        info!("Starting `{}`", cmd_str);
        runner.start(&args.command)?;
        for input in listener {
            match input {
                'r' => {
                    info!("Reloading `{}`", cmd_str);
                    runner.start(&args.command)?;
                }
                'c' => {
                    if !runner.is_running() {
                        warn!("`{}` is already closed!", cmd_str);
                        continue;
                    }
                    warn!("Closing `{}`", cmd_str);
                    runner.stop()?;
                    warn!("Closed `{}`", cmd_str);
                }
                'q' => {
                    error!("Quitting `{}`", cmd_str);
                    runner.stop()?;
                    break;
                }
                _ => {}
            }
        }

        return Ok(());
    }
}
