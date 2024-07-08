fn prev_str(s: &str) -> String {
  s.chars().map(|c|
  match c {
    'b'..='z' | 'B'..='Z' => (c as u8 -1) as char,
    '1'..='9' => (c as u8 -1) as char,
    _ => c,
  }
  ).collect()
}

struct X {
  s: Option<String>,
  i: i32,
}

impl X {
  fn new(s: &str, i: i32) -> X {
    X {
      s: Some(s.to_string()),
      i,
    }
  }

  fn take_str(&mut self) -> Option<String> {
    self.s.take()
  }
}