use std::mem::swap;

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

struct NameSurname {
  name: String,
  surname: String,
}

fn replace_surname(ns: &mut NameSurname, s: String) -> String {
  // ns.surname e basta non va, avviene un move!
  let out = ns.surname.clone();
  ns.surname = s;

  out
}

fn replace_surname2(ns: &mut NameSurname, s: String) -> String {
  let mut old_surname = s;
  swap(&mut ns.surname, &mut old_surname);

  old_surname
}

struct Student {
  name: String,
  id: u32,
}

impl Student {

}