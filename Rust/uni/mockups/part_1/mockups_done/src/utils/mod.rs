use std::fmt::Display;

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

pub struct NameSurname {
  name: String,
  surname: String,
}

pub fn replace_surname(ns: &mut NameSurname, s: String) -> String {
  let out = ns.surname.clone();

  ns.surname = s;
  out
}

#[derive(Clone)]
pub struct Student {
  name: String,
  id: u32,
}

impl Student {
  pub fn new(name: &str, id: u32) -> Self {
    Self {
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

pub struct University {
  name: String,
  students: Vec<Student>,
}

impl University {
  pub fn new(name: &str, students: &[Student]) -> Self {
    Self {
      name: name.to_string(),
      students: students.to_vec()
    }
  }

  pub fn remove_student(&mut self, id: u32) -> Result<Student, &str> {
    for (index, student) in self.students.iter().enumerate() {
      if student.id == id {
        return Ok(self.students.remove(index));
      }
    }

    Err("No such Id found.")
  }
}

impl Display for University {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "University: {}\nStudents:\n", self.name)?;
        for student in &self.students {
            write!(f, "{}\n", student)?;
        }
        Ok(())
    }
}

// TODO: hashmaps and airfleet

pub struct Size {
  width: u32,
  height: u32,
}

impl Size {
  pub fn new(width: u32, height: u32) -> Self {
    Self {
      width,
      height,
    }
  }

  pub fn area(&self) -> u32 {
    self.width * self.height
  }

  pub fn compare(&self, size2: Size) -> Option<bool> {
    if self.area() == size2.area() {
      return None;
    } else if self.area() > size2.area() {
      return Some(true);
    } else {
      return Some(false);
    }
  }
}

pub struct MaybePoint {
  x: Option<i32>,
  y: Option<i32>,
}

impl MaybePoint {
  pub fn new(x: Option<i32>, y: Option<i32>) -> Self {
    Self {
      x,
      y,
    }
  }

  pub fn is_some(&self) -> bool {
    self.x.is_some() && self.y.is_some()
  }

  pub fn maybe_len(&self) -> Option<f32> {
    // if self.x.is_none() || self.y.is_none() {
    //   return None;
    // }
    // sbagliato: accetto il caso che sono some, some

    match (self.x, self.y) {
      (Some(x), Some(y)) => {
        let len = ((x.pow(2) as f32 + y.pow(2) as f32) as f32).sqrt();
        return Some(len)
      },
      _ => return None,
    }
  }
}

pub fn res1(n: i32) -> Result<i32, String> {
  if n % 10 == 0 {
    return Ok(n);
  }

  return Err("error".to_string());
}

pub fn res2(res: Result<i32, &str>) -> Result<i32, String> {
  match res {
    Ok(n) => {
      if n % 5 == 0 {
        return Ok(n);
      } else {
        return Err("error".to_string());
      }
    },
    Err(_) => return Err("error".to_string())
  }
}

pub fn wrapper(n: i32) -> Result<i32, String> {
  let result1 = res1(n)?;
  let result2 = res2(Ok(result1))?;

  Ok(result2)
}

pub fn order(v: Vec<String>) -> Vec<String> {
  let mut out: Vec<String> = Vec::new();

  for (index, string) in v.iter().enumerate() {
    let temp = format!("{} - {}", index, string);
    out.push(temp);
  }

  out
}

pub fn order2(strings: Vec<String>) -> Vec<String> {
  strings.iter().enumerate().map(|(index, s)| {
    format!("{} - {}", index, s)
  }).collect()
}

