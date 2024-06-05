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

#[derive(Debug, Clone, PartialEq)]
enum AirplaneCompany {
    Airbus,
    Boeing,
}

#[derive(Debug, Clone)]
struct Airplane {
    company: AirplaneCompany,
    model: String,
}

struct AirFleet {
    airplanes: Vec<Airplane>,
}

impl AirFleet {
    // Metodo per rimuovere tutti gli aerei Boeing dalla flotta
    fn remove_boeing(&mut self) {
        self.airplanes.retain(|airplane| airplane.company != AirplaneCompany::Boeing);
    }

    // Metodo per aggiungere un aereo alla flotta
    fn add_airplane(&mut self, airplane: Airplane) {
        self.airplanes.push(airplane);
    }

    // Metodo per cercare un aereo per modello
    fn search_airplane(&self, model: &str) -> Result<AirplaneCompany, String> {
        for airplane in &self.airplanes {
            if airplane.model == model {
                return Ok(airplane.company.clone());
            }
        }
        Err(format!("No such model found!"))
    }
}