use std::fmt::{ Display, Formatter, Result };

pub fn prev_str(s: &str) -> String {
  let mut out = String::new();

  for c in s.chars() {
    if c.is_alphabetic() {
      if c != 'a' && c != 'A' {
        out.push((c as u8 -1) as char);
        continue;
      }
    }

    out.push(c);
  }

  out
}

pub struct X {
  s: Option<String>,
  i: i32,
}

impl X {
  fn new(s: &str, i: i32) -> Self {
    Self {
      s: Some(s.to_string()),
      i,
    }
  }

  fn take_str(&mut self) -> Option<String> {
    let out = self.s.clone();

    self.s = None;

    out
  }
}

pub struct NameSurname {
  name: String,
  surname: String,
}

pub fn replace_surname(ns: &mut NameSurname, s: String) -> String {
  let out = ns.surname.clone();

  ns.surname = s;
  out
}

pub struct Student {
  name: String,
  id: u32,
}

impl Student {
  fn new(name: &str, id: u32) -> Self {
    Self {
      name: name.to_string(),
      id
    }
  }
}

impl Display for Student {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
      
  }
}