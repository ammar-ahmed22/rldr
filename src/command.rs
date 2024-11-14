use anyhow::Result;
use colored::*;
use libc::{pid_t, SIGKILL};
use std::os::unix::process::CommandExt;
use std::process::{Child, Command};
use std::sync::{Arc, Mutex};

pub struct CommandRunner {
    current_process: Arc<Mutex<Option<Child>>>,
}

impl CommandRunner {
    pub fn new() -> Self {
        Self {
            current_process: Arc::new(Mutex::new(None)),
        }
    }

    pub fn spawn_child(command: &[String]) -> Result<Child> {
        if cfg!(target_os = "windows") {
            let child = Command::new("cmd")
                .args(&["/C", command.join(" ").as_str()])
                .spawn()?;
            return Ok(child);
        } else {
            let child = unsafe {
                Command::new("sh")
                    .arg("-c")
                    .arg(command.join(" ").as_str())
                    .pre_exec(|| {
                        if libc::setsid() == -1 {
                            return Err(std::io::Error::last_os_error());
                        }
                        return Ok(());
                    })
                    .spawn()
            }?;
            return Ok(child);
        }
    }

    pub fn is_running(&self) -> bool {
        match *self.current_process.lock().unwrap() {
            Some(_) => {
                return true;
            }
            None => {
                return false;
            }
        }
    }

    pub fn start(&self, command: &[String]) -> Result<()> {
        self.stop()?;

        let child = Self::spawn_child(command)?;
        let cmd_str = format!("{}", command.join(" ")).purple();
        log::info!("Started `{}`", cmd_str);
        let mut process_lock = self.current_process.lock().unwrap();
        *process_lock = Some(child);
        if let Some(ref mut process) = *process_lock {
            match process.wait() {
                Ok(status) => {
                    log::warn!(
                        "`{}` exited with status {:?}",
                        cmd_str,
                        status.code().unwrap_or(-1)
                    );
                    *process_lock = None;
                }
                Err(e) => {
                    log::error!("`{}` failed to exit with error: {}", cmd_str, e);
                    *process_lock = None;
                }
            }
        }
        return Ok(());
    }

    pub fn stop(&self) -> Result<()> {
        let mut process_lock = self.current_process.lock().unwrap();
        if let Some(ref mut process) = *process_lock {
            let _ = process.kill();
            if cfg!(target_os = "windows") {
                Command::new("taskill")
                    .arg("/PID")
                    .arg(process.id().to_string())
                    .arg("/T")
                    .arg("/F")
                    .spawn()?;
            } else {
                let pid = process.id() as pid_t;
                unsafe {
                    libc::kill(-pid, SIGKILL);
                }
            }
            *process_lock = None;
        }

        return Ok(());
    }
}
