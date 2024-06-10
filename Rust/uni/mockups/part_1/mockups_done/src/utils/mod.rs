use std::fmt::Display;

pub fn prev_str(input: &str) -> String {
  input.chars().map(|c|
    if c.is_ascii_alphabetic() {
      match c {
        'a' | 'A' => c,
        'b' ..= 'z' => (c as u8 -1) as char,
        'B' ..= 'Z' => (c as u8 -1) as char,
        _ => c,
      }
    } else {
      c
    }
  ).collect()
}

pub fn prev_str2(input: &str) -> String {
  let mut out = String::new();

  for c in input.chars() {
    match c {
      'a' | 'A' => out.push(c),
      'b' ..= 'z' => out.push((c as u8 -1) as char),
      'B' ..= 'Z' => out.push((c as u8 -1) as char),
      _ => out.push(c)
    }
  }

  out
}

pub struct X {
  s: Option<String>,
  i: i32
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

  pub fn take_str2(&mut self) -> Option<String> {
    let out = self.s.clone();
    self.s = None;

    out    
  }
}

pub struct NameSurname {
  name: String,
  surname: String
}

pub fn replace_surname(ns: &mut NameSurname, s: String) -> String {
  let out = ns.surname.clone();

  ns.surname = s;

  out
}

fn replace_surname2(mut person: NameSurname, new_surname: String) -> String {
    let old_surname = person.surname;
    person.surname = new_surname;
    old_surname
}

pub struct Student {
  name: String,
  id: u32
}

impl Display for Student {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{} [{}]", self.name, self.id)
  }
}

impl Student {
  pub fn new(name: &str, id: u32) -> Self {
    Self {
      name: name.to_string(),
      id,
    }
  }
}

