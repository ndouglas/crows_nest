use std::io::Write;
use std::io::{self, BufRead};

pub struct Repl {}

impl Repl {
  pub fn new() -> Self {
    Self {}
  }

  pub fn run(&self) {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    loop {
      print!("> ");
      let _ = io::stdout().flush();

      let line = lines.next().unwrap().unwrap_or_else(|e| {
        eprintln!("error: {}", e);
        std::process::exit(1);
      });

      let trimmed = line.trim();

      if trimmed == "quit" {
        break;
      } else if !trimmed.is_empty() {
        self.process_input(trimmed);
      }
    }
  }

  pub fn process_input(&self, _input: &str) {
    // ...process the input...
  }
}
