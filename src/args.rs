use clap::Parser;

#[derive(Parser, Debug)]
#[command(
  name = "rldr",
  version = env!("CARGO_PKG_VERSION"),
  about = "A simple command line utility to run and manage commands with keypress controls."
)]
pub struct Args {
    /// The command to execute with or without arguments
    #[arg(required = true, trailing_var_arg = true)]
    pub command: Vec<String>,

    /// An additional command to execute with arguments
    /// Useful for compound commands which will fail using positional arguments
    #[arg(long, value_name = "COMMAND")]
    pub exec: Option<Vec<String>>,
}
