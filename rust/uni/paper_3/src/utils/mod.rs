use std::{collections::HashMap, hash::Hash};

pub fn is_luhn(number: String) -> bool {
    if number.len() <= 1 {
        return false;
    }

    let mut sum: u32 = 0;
    let number: String = number.trim().chars().filter(|c| !c.is_whitespace()).collect();

    for (index, digit) in number.chars().rev().enumerate() {
        let mut digit = match digit.to_digit(10) {
            Some(d) => d,
            None => return false, // Ritorna false in caso di carattere non numerico
        };

        if index % 2 != 0 {
            digit *= 2;
            if digit > 9 {
                digit -= 9;
            }
        }

        sum += digit;
    }

    sum % 10 == 0
}

pub enum Fuel {
  Diesel,
  Gasoline,
  LPG,
  Methane,
  Electric,
}

pub enum IpVersion {
  IPv4([u8; 3]),
  IPv6([u16; 4]),
}

pub struct Point {
  x: f64,
  y: f64,
  z: f64,
}

pub struct UnnamedPoint(f64, f64, f64);

pub struct ParkingLot {
  remaining_spots: u32,
  // HashMap <plate, owner>
  cars: HashMap<String, String>,
}

const TOTAL_SPOTS: u32 = 100;

impl ParkingLot {
  fn new() -> Self {
    ParkingLot {
      remaining_spots: TOTAL_SPOTS,
      cars: HashMap::new(),
    }
  }

  fn new_custom(cars: HashMap<String, String>) -> Self {
    ParkingLot {
      remaining_spots: TOTAL_SPOTS - cars.len() as u32,
      cars,
    }
  }

  fn recognise_owner(&self, plate: String) -> Option<String> {
    // Alternativa piu' facile
    // self.cars.get(&plate).cloned()

    match self.cars.get(&plate) {
      Some(owner) => return Some(owner.clone()),
      None => return None,
    }
  }
}

pub enum Coin {
  Cent10,
  Cent20,
  Cent50,
  Eur1,
  Eur2,
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum Item {
  Water,
  Coca,
  Soda,
  Tea,
}

pub struct VendingMachine {
  coins: u32,
  items: HashMap<Item, usize>,
}

impl VendingMachine {
  fn new(items: HashMap<Item, usize>) -> Self {
    Self {  // VendingMachine {
      coins: 0,
      items,
    }
  }

  fn add_item(&mut self, item: Item, quantity: usize) {
    
  }
}




