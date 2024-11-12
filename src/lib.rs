mod args;
mod command;
mod io;
mod logger;

use anyhow::Result;
use clap::Parser;
use command::CommandRunner;
use io::start_input_listener;
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
        let receiver = start_input_listener();

        let cmd_str = format!("{}", args.command.join(" "));
        for input in receiver {
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

// pub struct Reloader {
//     command: String,
//     child_process: Arc<Mutex<Option<Child>>>,
//     should_stop: Arc<Mutex<bool>>
// }

// impl Reloader {
//     pub fn new(command: String) -> Self {
//         Self {
//             command,
//             child_process: Arc::new(Mutex::new(None)),
//             should_stop: Arc::new(Mutex::new(false))
//         }
//     }

//     pub fn terminate(&mut self) {
//         *self.should_stop.lock().unwrap() = true;
//         if let Some(mut child) = self.child_process.lock().unwrap().take() {
//             let _ = child.kill();
//             info!("Process terminated.");
//         }
//         // *self.child_process.lock().unwrap() = None;
//     }

//     pub fn spawn(&mut self) -> anyhow::Result<()> {
//         *self.should_stop.lock().unwrap() = false;
//         let mut new_child = Self::spawn_child(self.command.clone())?;

//         let stdout = new_child.stdout.take().expect("Failed to capture stdout");
//         let stderr = new_child.stderr.take().expect("Failed to capture stderr");

//         *self.child_process.lock().unwrap() = Some(new_child);

//         let child_clone = Arc::clone(&self.child_process);
//         let stop_clone = Arc::clone(&self.should_stop);

//         // Spawn a thread to read stdout in real-time, byte-by-byte
//         thread::spawn(move || {
//             Self::read_output(stdout, false, child_clone, stop_clone);
//         });
//         let child_clone = Arc::clone(&self.child_process);
//         let stop_clone = Arc::clone(&self.should_stop);
//         // Spawn another thread to read stderr in real-time, byte-by-byte
//         thread::spawn(move || {
//             Self::read_output(stderr, true, child_clone, stop_clone);
//         });

//         Ok(())
//     }

//     fn spawn_child(command: String) -> anyhow::Result<Child> {
//         if cfg!(target_os = "windows") {
//             Command::new("cmd")
//                 .args(&["/C", command.as_str()])
//                 .stdout(Stdio::piped())
//                 .stderr(Stdio::piped())
//                 .spawn()
//                 .map_err(|e| anyhow::anyhow!("Failed to spawn command: {}", e))
//         } else {
//             Command::new("bash")
//                 .arg("-c")
//                 .arg(command.as_str())
//                 .stdout(Stdio::piped())
//                 .stderr(Stdio::piped())
//                 .spawn()
//                 .map_err(|e| anyhow::anyhow!("Failed to spawn command: {}", e))
//         }
//     }

//     fn read_output(mut stream: impl Read, is_stderr: bool, child_clone: Arc<Mutex<Option<Child>>>, stop_flag: Arc<Mutex<bool>>) {
//         let mut buffer = [0; 1]; // Read one byte at a time
//         let mut line = String::new();

//         loop {
//             if *stop_flag.lock().unwrap() {
//                 break;
//             }
//             if let Ok(bytes_read) = stream.read(&mut buffer) {
//                 if bytes_read == 0 {
//                     break; // End of stream
//                 }

//                 // Convert the byte to a char and append it to the line
//                 if let Ok(character) = std::str::from_utf8(&buffer) {
//                     line.push_str(character);

//                     // If we reach a newline, print the line and clear it
//                     if character == "\n" {
//                         if is_stderr {
//                             error!("{}", line.trim_end());
//                         } else {
//                             info!("{}", line.trim_end());
//                         }
//                         line.clear();
//                     }
//                 }
//             }

//             // Check if the child process is still running
//             if let Some(child) = &mut *child_clone.lock().unwrap() {
//                 if child.try_wait().unwrap().is_some() {
//                     break; // Exit loop if the process has exited
//                 }
//             } else {
//                 break;
//             }
//         }
//     }
// }
