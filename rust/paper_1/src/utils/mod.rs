pub fn string_reverse(s: &str) -> String {
  // s.chars().rev()
  let mut s = s.to_string();
  let mut out = String::new();

  while let Some(c) = s.pop() {
    out.push(c);
  }

  out
}

pub fn bigger(x: i32, y: i32) -> i32 {
  if x > y {
    return x;
  }
  return y;
}

pub fn multiply(x: i32, y: f32, z: f64) -> f64 {
  x as f64 * y as f64 * z
}

const C: i32 = 299_792_458; // [m/s]

pub fn e_equals_mc_squared(mass: f32) -> f64 {
  return mass as f64 * (C.pow(2) as f64);
}

pub fn max_min(vector: Vec<i32>) -> (i32, i32) {
  let mut max: i32 = vector[0];
  let mut min: i32 = vector[0];

  for val in vector {
    if val > max {
      max = val;
    }
    if val < min {
      min = val;
    }
  }

  (max, min)
}

fn max_min_inner(vector: &Vec<i32>, index: &mut usize, max: &mut i32, min: &mut i32) {
    if *index == vector.len() {
        return;
    }

    if vector[*index] > *max {
        *max = vector[*index];
    }
    if vector[*index] < *min {
        *min = vector[*index];
    }

    *index += 1;
    max_min_inner(vector, index, max, min);
}

pub fn max_min2(vector: Vec<i32>) -> (i32, i32) {
    let mut max: i32 = vector[0];
    let mut min: i32 = vector[0];

    // OCCHIO CHE E' USIZE PER L'INDEXING!
    let mut index: usize = 0;

    max_min_inner(&vector, &mut index, &mut max, &mut min);
    // come mai passo il riferimento?

    (max, min)
}

pub fn lord_farquaad(s: String) -> String {
  // s.replace("e", "💥")
  let mut out = String::new();

  for c in s.chars() {
    match c {
      'e' => out.push('💥'),
      _ => out.push(c),
    }
  }

  out
}
