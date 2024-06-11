pub fn prev_str(s: &str) -> String {
  s.chars().map(|c| {
    match c {
      'b' ..= 'z' => ((c as u8) -1) as char,
      'B' ..= 'Z' => ((c as u8) -1) as char,
      _ => c,
    }
  }).collect()
}

pub struct X {
  s: Option<String>,
  i: i32,
}

impl X {
  pub fn new(s: &str, i: i32) -> Self {
    Self {
      s: Some(s.to_string()),
      i,
    }
  }

  pub fn take_str(&mut self) -> Option<String> {
    self.s.take()
  }

  
}