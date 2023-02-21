use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Options {
  #[clap(short, long, default_value_t = false)]
  pub repl: bool,
}
