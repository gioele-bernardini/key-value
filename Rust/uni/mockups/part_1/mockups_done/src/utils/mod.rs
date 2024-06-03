// Write a function capitalize that takes a &str as input and returns a String.

// This function should convert the first letter of each word to uppercase.

// Write a struct Book with two fields: title (a String) and pages (an i32). Then, implement the following methods for Book:

// new: takes a &str and an i32 and returns a Book instance.
// description: returns a string describing the book.
// Create a function named reverse_string that takes a String and returns the reversed String.

// Write a struct Rectangle with two fields: width and height, both u32. Implement the following methods for Rectangle:

// new: takes two u32 arguments and returns a Rectangle instance.
// area: returns the area of the rectangle.
// Write a struct Person with two fields: name (a String) and age (an u32). Then, implement the Display trait for Person to print "Name: [name], Age: [age]".

pub fn capitalize(s: &str) -> String {
  s
    .split_whitespace()
    .map(|word| {
      let mut chars = word.chars();

      match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
      }
    })
    .collect::<Vec<String>>()
    .join(" ")
}


// Write a struct Book with two fields: title (a String) and pages (an i32). Then, implement the following methods for Book:

// new: takes a &str and an i32 and returns a Book instance.
// description: returns a string describing the book.
// Create a function named reverse_string that takes a String and returns the reversed String.

pub struct Book {
  title: String,
  pages: i32,
}

impl Book {
  fn new(title: &str, pages: i32) -> Self {
    Self {
      title: title.to_string(),
      pages,
    }
  }

  
}

pub fn reverse_string(s: String) -> String {
  return s.chars().rev().collect();
}
