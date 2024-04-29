use core::slice;
use std::{collections::btree_map::Values, slice::Iter};

use rand::{prelude::*, seq::index};

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

pub fn sub_slice(vector: &Vec<i32>, sub_vector: &Vec<i32>) {
  let n_cases = vector.len() - sub_vector.len() + 1;
  let slice_size = sub_vector.len();

  let mut found = false;
  
  for i in 0..n_cases {
    let check = vector.split_at(i).1.split_at(slice_size).0;
    println!("{:?}", check);

    if check == sub_vector {
      println!("Found");
      found = true;
    }
  }
  if !found {
    println!("Not found");
  }
}

pub fn max(v: &[i32]) -> i32 {
  let mut max = 0;

  for value in v.iter() {
    if *value > max {
      max = *value;
    }
  }

  max
}

pub fn max_recursive(v: &[i32]) -> Option<i32> {
  if v.len() == 0 {
    return None
  }
  if v.len() == 1 {
    return Some(v[0]);
  }

  let first = v[0];
  let rest_max = max_recursive(&v[1..]);

  match rest_max {
    None => Some(first),
    Some(max_val) => {
      if first > max_val {
        Some(first)
      } else {
        Some(max_val)
      }
    }
  }
}

pub fn swap(v: &mut [i32]) {
  let temp = v[1];

  v[0] = v[v.len() -1];
  v[v.len() -1] = temp;
}

pub fn is_sorted(v: &[i32]) -> bool {
  let mut temp = v[0];

  for i in 1..v.len() {
    if v[i] < temp {
      return false;
    }

    temp = v[i];
  }

  return true;
}

pub fn is_sorted_recursive(v: &[i32]) -> bool {
  if v.len() <= 1 {
    return true;
  }

  if v[0] > v[1] {
    return false;
  }

  return is_sorted_recursive(&v[1..]);
}

pub fn insert_if_longer(v: &mut Vec<String>, s: &String) {
  if v.len() > 10 {
    v.push(s.clone());
  }
}

pub fn build_vector(iter: Iter<i32>) -> Vec<&i32> {
  let mut out: Vec<&i32> = Vec::new();
  // let vector = iter.collect();

  for i in iter {
    out.push(i);
  }

  out
}

fn flip(vector: &mut Vec<i32>) {

}

fn find_max(vector: &Vec<i32>) -> usize {
  let mut index_max: usize = 0;

  for i in 1..vector.len() {
    if vector[i] > vector[index_max] {
      index_max = i;
    }
  }

  index_max
}

fn pancake_sort(vector: &mut Vec<i32>) {
  let mut index = vector.len() -1;

  for value in vector {
    let index_max = find_max(&vector[0..index +1]);
    if index_max != index {
      flip(&mut vector[..index_max]);
      flip(&mut vector[..index+1]);
    }
  }
}