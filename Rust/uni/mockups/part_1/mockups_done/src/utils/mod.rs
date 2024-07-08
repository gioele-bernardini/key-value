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
    // Alternativa ciclo while con incremento condizionale!
    self.airplanes.retain(|airplane|
    airplane.company != Company::Boeing
    )
  }

  fn add_airplane(&mut self, airplane: Airplane) {
    self.airplanes.push(airplane);
  }

  fn search_airplane(&self, model: String) -> Result<Company, String> {
    for airplane in &self.airplanes {
      if airplane.model == model {
        // Clona il valore di ritorno!
        return Ok(airplane.company.clone());
      }
    }

    Err("No such model found.".to_string())
  }
}

struct Size {
  width: u32,
  height: u32,
}

impl Size {
  fn new(width: u32, height: u32) -> Size {
    Size {
      width,
      height,
    }
  }

  fn area(&self) -> u32 {
    self.height * self.width
  }

  fn compare(&self, size2: Size) -> Option<bool> {
    if self.area() == size2.area() {
      return None;
    } else if self.area() > size2.area() {
      return Some(true);
    }

    return Some(false);
  }
}

struct MaybePoint {
  x: Option<i32>,
  y: Option<i32>,
}

impl MaybePoint {
  fn new(x: Option<i32>, y: Option<i32>) -> MaybePoint {
    MaybePoint {
      x,
      y,
    }
  }

  fn is_some(&self) -> bool {
    match (self.x, self.y) {
      // Occhio alla sintassi!
      (Some(_), Some(_)) => return true,
      _ => return false,
    }
  }

  fn maybe_len(&self) -> Option<f32> {
    match (self.x, self.y) {
      // Occhio alla sintassi, questi valori poi devi usarli!
      (Some(x), Some(y)) => {
        let distance = (x.pow(2) as f32 + y.pow(2) as f32).sqrt();
        Some(distance)
      }
      _ => None,
    }
  }
}

fn res1(n: i32) -> Result<i32, String> {
  if n % 10 == 0 {
    Ok(n)
  } else {
    Err("error".to_string())
  }
}

fn res2(result: Result<i32, &str>) -> Result<i32, String> {
  match result {
    Ok(n) => {
      if n % 5 == 0 {
        Ok(n)
      } else {
        Err("error".to_string())
      }
    }
    Err(e) => Err(e.to_string())
  }
}

fn wrapper(n: i32) -> Result<i32, String> {
  // Prima propagazione (primo "check")
  let res1_result = res1(n)?;

  // Secondo check con input il primo risultato
  // Se il primo check va male la funzione
  // Si e' gia' interrotta la riga prima di questa!
  res2(Ok(res1_result))
}

fn order(s: Vec<String>) -> Vec<String> {
  let mut out: Vec<String> = Vec::new();

  for (index, string) in s.iter().enumerate() {
    let temp = format!("{} - {}", index, string);
    out.push(temp);
  }

  out
}

