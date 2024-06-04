use std::fmt::{ Display, Formatter };

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
    write!("{} {}", self.name, self.surname)
  }
}

struct University {
  name: String,
  students: Vec<Student>,
}

impl University {
  fn new(name: &str, students: &[Student]) -> Self {
    Self {
      name: name.to_string(),
      students,
    }
  }

  fn remove_student(&mut self, id: u32) -> Result<Student, &str> {
    for (index, s) in self.students.iter().enumerate() {
      if s.id == id {
        self.students.remove(index);
        return Ok(id);
      }
    }

    return Result::Err("No such Id found.");
  }
}

impl Display for University {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let mut out = String::new();

    for s in self.students {
      out.push_str(&s.name);
    }

    write!("{}", out)
  }
}

enum AirPlaneCompany {
  Airbus,
  Boeing,
}

struct Airplane {
  company: AirPlaneCompany,
  model: String,
}

struct AirFleet {
  airplanes: Vec<Airplane>,
}

impl AirFleet {
  fn remove_boeing(&mut self) {
    for (index, airplane) in self.airplanes.iter().enumerate() {
      if airplane.company == AirPlaneCompany::Boeing {
        self.airplanes.remove(index);
      }
    }
  }

  fn add_airplane(&mut self, airplane: Airplane) {
    self.airplanes.push(airplane);
  }

  fn search_airplane(&self, model: String) -> Result<AirPlaneCompany, String> {
    for airplane in self.airplanes {
      if airplane.model == model {
        return Ok(airplane.company);
      }
    }

    Err("No such model found!");
  }
}

mod hashmaps {

}