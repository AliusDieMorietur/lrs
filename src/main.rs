use std::{fs, path::PathBuf};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "lrs")]
struct Opt {
  #[structopt(parse(from_os_str))]
  input: Option<PathBuf>,
}

fn run_file(path: PathBuf) {
  let error = format!("Failed to open {:?}", path);
  let content =
    fs::read_to_string(path).expect(&error);
  println!("{}", content);
}

fn main() {
  let opt = Opt::from_args();
  match opt.input {
    Some(f) => {
      run_file(f);
    },
    None => {
      println!("run_prompt");
    },
  }
}
