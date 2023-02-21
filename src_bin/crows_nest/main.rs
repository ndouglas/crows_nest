use clap::Parser;

mod options;
use options::Options;
mod repl;
use repl::Repl;

fn main() {
  let opts = Options::parse();

  if opts.repl {
    let repl = Repl::new();
    repl.run();
  } else {
    // ...run the game normally...
  }
}
