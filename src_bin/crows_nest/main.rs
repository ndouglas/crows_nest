use clap::Parser;
use std::io::Write;
use std::io::{self, BufRead};

mod options;
use options::Options;

struct Repl {}

impl Repl {
  fn new() -> Self {
    Self {}
  }

  fn run(&self) {
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

  fn process_input(&self, input: &str) {
    // ...process the input...
  }
}

fn main() {
  let opts = Options::parse();

  if opts.repl {
    let repl = Repl::new();
    repl.run();
  } else {
    // ...run the game normally...
  }
}
