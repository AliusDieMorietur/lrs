use std::{
  fs,
  io::{self, Write},
  path::PathBuf,
  process,
};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "lrs")]
struct Opt {
  #[structopt(parse(from_os_str))]
  input: Option<PathBuf>,
}

struct Lrs {
  had_error: bool,
}

impl Lrs {
  fn new() -> Lrs { Lrs { had_error: false } }

  fn run(&self, command: &str) {
    for i in command.chars() {
      println!("{}", i);
    }
  }

  fn error(&mut self, line: usize, message: &str) {
    self.report(line, "", message);
    println!("{} {}", line, message);
  }

  fn report(&mut self, line: usize, place: &str, message: &str) {
    println!("[line{}] Error{}: {}", line, place, message);
    self.had_error = true;
  }

  fn run_file(&self, path: PathBuf) {
    let error = format!("Failed to open {:?}", path);
    let content = fs::read_to_string(path).expect(&error);
    self.run(&content);
  }

  fn run_promt(&self) {
    loop {
      print!("> ");
      io::stdout().flush();
      let mut input = String::new();
      match io::stdin().read_line(&mut input) {
        Ok(n) => {
          let trimmed = input.trim();
          match trimmed {
            "quit" => break,
            _ => {
              self.run(trimmed);
            },
          }
        },
        Err(error) => println!("error: {}", error),
      }
    }
  }
}



fn main() {
  let scanner = Lrs::new();
  let opt = Opt::from_args();
  match opt.input {
    Some(f) => {
      scanner.run_file(f);
    },
    None => {
      scanner.run_promt();
    },
  }
}
