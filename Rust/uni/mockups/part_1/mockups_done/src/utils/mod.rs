use std::{fmt::Display, mem::swap};

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
    // Occhio a ritornare una Option come da firma!
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

#[derive(Clone)]
struct Student {
  name: String,
  id: u32,
}

impl Student {
  fn new(name: &str, id: u32) -> Student {
    Student {
      name: name.to_string(),
      id,
    }
  }
}

impl Display for Student {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{} [{}]", self.name, self.id)
  }
}

struct University {
  name: String,
  students: Vec<Student>,
}

impl University {
  fn new(name: &str, students: &[Student]) -> University {
    University {
      name: name.to_string(),
      students: students.to_vec(),
    }
  }

  fn remove_student(&mut self, id: u32) -> Result<Student, &str> {
    // Occhio al metodo .enumerate()!
    for (index, s) in self.students.iter().enumerate() {
      if s.id == id {
        // Rompo il ciclo e ritorno direttamente!
        return Ok(self.students.remove(index));
      }
    }

    Err("No such Id found.")
  }
}

impl Display for University {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    // Punti interrogativi per dare errore immediatamente!
    writeln!(f, "University: {}", self.name)?;
    writeln!(f, "Students:",)?;
    
    // Prendo il riferimento, non voglio averne il possesso!
    for student in &self.students {
      writeln!(f, " {}", student)?;
    }

    Ok(())
  }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Company {
  Airbus,
  Boeing,
}

#[derive(Clone)]
struct Airplane {
  company: Company,
  model: String,
}

#[derive(Clone)]
struct AirFleet {
  airplanes: Vec<Airplane>,
}

impl AirFleet {
  fn new(airplanes: &[Airplane]) -> AirFleet {
    AirFleet {
      airplanes: airplanes.to_vec(),
    }
  }

  fn remove_boeing(&mut self) {
    self.airplanes.retain(|airplane|
    airplane.company != Company::Boeing
    )
  }
}