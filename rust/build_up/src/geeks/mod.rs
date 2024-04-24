use std::env::{self, args_os};

fn greet() {
  let args: Vec<String> = env::args().collect();
  let arg_count = args.len();

  if arg_count != 2 {
    eprint!("Usage: {} <name>\n", args[0]);
    std::process::exit(1);
  }

  let name = &args[1];
  println!("Hello, {}", name);
}
