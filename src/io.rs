use std::sync::mpsc::{ self, Receiver };
use std::thread;
use std::io::{ self, Write};
use log::info;

pub fn input_listener() -> Receiver<char> {
  let (tx, rx) = mpsc::channel();
  thread::spawn(move || {
    loop {
      info!("Enter `r` to reload, `c` to close, `q` to quit");
      io::stdout().flush().unwrap();

      let mut buffer = String::new();
      if io::stdin().read_line(&mut buffer).is_ok() {
        if let Some(c) = buffer.trim_end().chars().next() {
          if tx.send(c.to_ascii_lowercase()).is_err() {
            break;
          }
        }
      }
    }
  });

  return rx;
}
