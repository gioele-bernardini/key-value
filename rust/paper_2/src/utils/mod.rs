use rand::prelude::*;

pub fn modify_odd(slice: &mut [i32]) {
  for num in slice.iter_mut() {
    if *num % 2 != 0 {
      *num = 0;
    }
  }
}

pub fn generate_slice(len: usize) -> Vec<i32> {
  let mut rng = rand::thread_rng();

  // let mut out: [u32; len] = [0; len];
  // sbagliato: la size di una slice deve essere nota a compile time!
  let mut out = Vec::with_capacity(len);

  for _ in 0..len {
    out.push(rng.gen_range(0..=100));
  }

  out
}

pub fn modify_odd2(slice: &mut [i32]) {
  for index in 0..slice.len() {
    if slice[index] % 2 != 0 {
      slice[index] = 0;
    }
  }
}

pub fn split_at_value(slice: &[i32], target: i32)
-> Option<(Vec<i32>, Vec<i32>)> {
  // gli indici degli array sono sempre degli USIZE!
  let mut index_target: usize = 0;

  for (index, value) in slice.iter().enumerate() {
    if *value == target {
      index_target = index;
      let (left, right) = slice.split_at(index_target + 1);
      return Some((left.to_vec(), right.to_vec()));
    }
  }

  None
}

