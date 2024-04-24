use std::mem;
use std::io::{self, Write};
use std::env::{ self };

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

fn sum() {
  let mut A = 0;
  let mut B = 0;

  loop {
    print!("Please insert two valid numbers => ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let mut parts = input.trim().split_whitespace();

    if let (Some(a_str), Some(b_str)) = (parts.next(), parts.next()) {
      if let (Ok(A_num), Ok(B_num)) = (a_str.parse::<i32>(), b_str.parse::<i32>()) {
        A = A_num;
        B = B_num;
        break;
      }
    }
  }

  let sum = A + B;
  println!("The result is => {}", sum);
}

fn size_of<T>() -> usize {
  // let size_of_i32 = size_of::<i32>();
  mem::size_of::<T>()
}


