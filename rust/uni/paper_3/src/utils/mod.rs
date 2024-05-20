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
    self.items.insert(item, quantity);
  }

  fn insert_coin(&mut self, coin: Coin) {
    match coin {
      Coin::Cent10 => self.coins += 10,
      Coin::Cent20 => self.coins += 20,
      Coin::Cent50 => self.coins += 50,
      Coin::Eur1 => self.coins += 100,
      Coin::Eur2 => self.coins += 200,
    }
  }

  fn get_item_price(&self, item: &Item) -> u8 {
    match item {
      Item::Water => 150,
      Item::Coca => 200,
      Item::Soda => 200,
      Item::Tea => 180,
    }
  }

fn buy(&mut self, item: &Item) -> Result<(), String> {
    let price = self.get_item_price(item) as u32;

    if self.coins < price {
        return Err("Not enough money".to_string());
    }

    if let Some(quantity) = self.items.get_mut(&item) {
        if *quantity > 0 {
            let change = self.coins - price;
            *quantity -= 1;
            return Ok(());
        }
    }

    Err("Item not available".to_string())
}

}

#[derive(PartialEq, Eq, Debug)]
pub struct Date(u8, u8, u16);

#[derive(PartialEq, Eq, Debug)]
pub struct Hour(u8, u8);

pub struct BoxShipping {
  name: String,
  barcode: String,
  shipment_date: Date,
  shipment_hour: Hour,
}

use std::fmt::{ Debug, Display, Formatter };

impl Display for BoxShipping {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{} {} {:?} {:?}", self.name, self.barcode, self.shipment_date, self.shipment_hour)
  }
}

impl Debug for BoxShipping {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "Debug: {} {} {:?} {:?}", self.name, self.barcode, self.shipment_date, self.shipment_hour)
  }
}

// TODO: implement the other methods

// TODO: library system

