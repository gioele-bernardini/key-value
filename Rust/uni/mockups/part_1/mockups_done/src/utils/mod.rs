use std::fmt::{ Display, Formatter };

fn prev_str(input: &str) -> String {
    input.chars().map(|c| {
        match c {
            'b'..='z' => (c as u8 - 1) as char,
            'B'..='Z' => (c as u8 - 1) as char,
            _ => c,
        }
    }).collect()
}

struct X {
    s: Option<String>,
    i: i32,
}

impl X {
    // Costruttore per creare una nuova istanza di X
    fn new(s: &str, i: i32) -> X {
        X {
            s: Some(s.to_string()),
            i,
        }
    }

    // Metodo per prendere il campo s e sostituirlo con None
    fn take_str(&mut self) -> Option<String> {
        self.s.take()
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

use std::fmt::{self, Display, Formatter};

struct Student {
    name: String,
    id: u32,
}

impl Student {
    fn new(name: &str, id: u32) -> Self {
        Self {
            name: name.to_string(),
            id,
        }
    }
}

impl Display for Student {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} (ID: {})", self.name, self.id)
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
            students: students.to_vec(),
        }
    }

    fn remove_student(&mut self, id: u32) -> Result<Student, &str> {
        for (index, s) in self.students.iter().enumerate() {
            if s.id == id {
                return Ok(self.students.remove(index));
            }
        }

        Err("No such Id found.")
    }
}

impl Display for University {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut out = format!("University: {}\nStudents:\n", self.name);

        for s in &self.students {
            out.push_str(&format!("{}\n", s));
        }

        write!(f, "{}", out)
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