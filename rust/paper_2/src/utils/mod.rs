pub fn modify_odd(slice: &mut [i32]) {
  for num in slice.iter_mut() {
    if *num % 2 != 0 {
      *num = 0;
    }
  }
}

pub fn modify_odd2(slice: &mut [i32]) {
  for index in 0..slice.len() {
    if slice[index] % 2 != 0 {
      slice[index] = 0;
    }
  }
}