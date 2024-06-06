use std::fmt::Debug;
use std::ops::Add;

pub mod odd_module {
  pub const CONSTANT: i32 = 123;
}

pub mod even_module {
  pub const CONSTANT: i32 = 246;
}

pub mod getter_function {
  fn get_constant(x: u32) -> i32 {
    if x % 2 == 0 {
      return super::even_module::CONSTANT;
    } else {
      return super::odd_module::CONSTANT;
    }
  }
}

trait CloneAndDouble {
  fn clone_and_double(&self) -> Self;
}

impl<T> CloneAndDouble for T
where
  T: Clone + Add<Output = T>,
{
  fn clone_and_double(&self) -> Self {
    self.clone() + self.clone()
  }
}

trait Unknown {
  fn serialize(&self) -> String;
}

impl Unknown for i32 {
  fn serialize(&self) -> String {
    self.to_string()
  }
}

impl Unknown for String {
  fn serialize(&self) -> String {
    self.clone()
  }
}

impl<T: Debug> Unknown for Vec<T> {
  fn serialize(&self) -> String {
    format!("{:?}", self)
  }
}

fn get_vec() -> Vec<Box<dyn Unknown>> {
  Vec::new()
}

fn print_vec(vec: &Vec<Box<dyn Unknown>>) {
  for item in vec {
    println!("{}", item.serialize());
  }
}