use std::path::Iter;

trait Nextable {
  fn gimme_next(&self) -> Option<Self> where Self: Sized;
}

impl Nextable for i32 {
  fn gimme_next(&self) -> Option<Self> where Self: Sized {
    return Some(self +1);
  }
}

impl Nextable for char {
  fn gimme_next(&self) -> Option<Self> where Self: Sized {
    match *self {
      'a'..='y' | 'A'..='Y' => Some((*self as u8 +1) as char),
      _ => None,
    }
  }
}

fn printnext<T: Nextable + std::fmt::Debug>(value: T) {
  println!("Next of {:?} is {:?}", value, value.gimme_next());
}

struct Wrapper {
  v: Vec<String>,
}

impl Wrapper {
  fn new(v: Vec<String>) -> Wrapper {
    Wrapper {
      v,
    }
  }

  fn iter(&self) -> WrapperIterator {
    WrapperIterator {
      wrapper: self,
      index: 0,
    }
  }
}

struct WrapperIterator<'a> {
  wrapper: &'a Wrapper,
  index: usize,
}

impl<'a> Iterator for WrapperIterator<'a> {
  type Item = usize;

  fn next(&mut self) -> Option<Self::Item> {
    if self.index < self.wrapper.v.len() {
      let length = self.wrapper.v[self.index].len();
      self.index += 1;

      return Some(length);
    } else {
      return None;
    }
  }
}