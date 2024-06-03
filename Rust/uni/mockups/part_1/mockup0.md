### Mockup Midterm 2

#### Part 1: Multiple Choice on Rust Semantics

1. **A: Will the following code compile? Why?**

```rust
fn main() {
  let x = String::from("hello");
  let y = x; // this is line 3
  println!("{}, world!", y);
  println!("{}, world!", x);

  let x = 10;
  let y = x;
  println!("y={}", y);
  println!("x={}", x);
}

let x = &[1, 2, 3, 4];
let y = x; // this is line 2
println!("{:?}", y);
println!("{:?}", x);
```

Answers:
1. Yes
2. No, x is moved
3. No, x and y are defined twice
4. No, x does not implement the Clone trait
5. No, x is borrowed twice

**B: If not, how to fix it?**

Answers:
1. Replace line 3 with `let y = x.clone();`
2. Replace line 3 with `let y: &String = x;`
3. Add `y.drop();` between the first and the second `println!`
4. Rename `x` and `y` in the second half of the code to `x_1` and `y_1`
5. Delete `println!("{}, world!", y);`

2. **A: Will the following code compile? Why?**

```rust
fn main() {
  let x = String::from("hello");
  let y = x; // this is line 3
  println!("{}, world!", y);
  println!("{}, world!", x);

  let x = 10;
  let y = x;
  println!("y={}", y);
  println!("x={}", x);
}

let x = &[1, 2, 3, 4];
let y = x; // this is line 2
println!("{:?}", y);
println!("{:?}", x);
```

Answers:
1. Yes
2. No, x is moved
3. No, [i32;4] does not implement the Copy trait
4. No, x does not implement Debug

**B: If not, how to fix it?**

Answers:
1. Replace line 2 with `let y = &x;`
2. Replace line 2 with `let y = x.clone();`
3. Add `y.drop();` between the 2 `println!`
4. Replace line 3 with `println!("{:?}", y.clone());`
5. Switch the order of the two `println!`

3. **A: What is the owner of `vec`'s original data at `HERE`?**

```rust
fn main() {
  let vec = vec![1, 2, 3];
  let y = vec;
  // HERE
}
```

Answers:
1. x
2. y
3. z
4. w
5. The code doesn't compile

**B: When is the value of `x` dropped?**

Answers:
1. After line 2
2. After line 3
3. After line 4
4. After line 5

4. **A: Will the following code compile? Why?**

```rust
fn foo(vec: Vec<i32>) { // this is line 1
  let x = vec.clone();
  let y = vec;
  let z = &y;
  let w = y;
  // HERE
}
```

Answers:
1. Yes
2. No, s1 and s2 are moved inside the function
3. No, it is not possible to concatenate to s1 since s1 is not mutable
4. No, the String type does not support the `+` operator

**B: If not, how to fix it?**

Answers:
1. Replace line 4 with `let s3 = concat(s1.clone(), s2.clone())`
2. The function should contain `return (s1 + s2.as_str()).clone();`
3. Replace line 5 with `println!("{} + {} = {}", &s1, &s2, &s3);`

5. **Does the following code compile?**

```rust
fn main() {
  let x = String::from("hello");
  let y = &x;
  println!("{}, {}", x, y);
}
```

Answers:
1. Yes
2. No, the x reference must be initialized at the first line
3. No, the x reference is not mutable and can't be overwritten with &v
4. No, x is possibly uninitialized

6. **Does the following code compile?**

```rust
fn main() {
  let s1 = String::from("hello ");
  let s2 = String::from("world!");
  let s3 = concat(s1, s2);
  println!("{} + {} = {}", s1, s2, s3); // this is line 5
}

fn concat(s1: String, s2: String) -> String {
  s1 + s2.as_str()
}

{
  let x: &u8;
  let v = 10;
  match v {
    0..=10 => x = &v,
    _ => {}
  }
  println!("{x}");
}
```

Answers:
1. No, third has type i32, and can't be assigned to a u8 type
2. No, third's lifetime ends after line 6, and can't be borrowed at line 9
3. Yes, it displays 5
4. Yes, it displays 12

7. **What can be added at line 2 without breaking the code?**

```rust
fn main() {
  let s = String::from("hello");
  // Line 2
  println!("{}", s);
}
```

Answers:
1. `s.push('!');`
2. `let t = &s;`
3. `let slice = &s[0..2];`
4. `s.clear();`
5. None of the above

#### Part 2: Coding Rust Concepts

1. **Write a function `prev_str` that takes a `&str` as input and returns a `String`.**

   This function converts the `&str` by replacing each character with its predecessor. For example:

   - 'b' becomes 'a'
   - 'f' becomes 'e'
   - 'B' becomes 'A'
   - 'A' remains 'A'
   - 'a' remains 'a'

   If the character is not a letter, it remains unchanged.

2. **Write a struct `X` with two fields: `s` (an `Option<String>`) and `i` (an `i32`). Then, implement the following methods for `X`:**

   - `new`: takes a `&str` and an `i32` and returns an `X` instance.
   - `take_str`: takes a mutable reference to `self` and returns the `s` field of `X`, replacing it with `None`.

3. **Create a function named `replace_surname` that takes a `NameSurname` struct (a struct with the field `name: String` and `surname: String`) and a `String` as input and returns (guess what?) a `String`. The function should replace the surname of the `NameSurname` struct with the `String` and return the old surname.**

4. **Write a struct `Student` with two fields: `name` (a `String`) and `id` (a `u32`). Then, implement the following methods for `Student`:**

   - `new`: takes a `&str` and a `u32` and returns a `Student` instance.
   - `Display`: implement the `Display` trait for `Student` so that it prints the name and the id of the student.

   Then write a struct `University` with two fields: `name` (a `String`) and `students` (a `Vec<Student>`). Then, implement the following methods for `University`:

   - `new`: takes a `&str` and a `&[Student]` and returns a `University` instance.
   - `remove_student`: takes an `id: u32` and returns a `Result<Student, &str>`:
     - `Ok(Student)` if a student with the given id is found and removed
     - `Err("Not found")` otherwise
   - `Display`: implement the `Display` trait for `University` so that it prints the name and the list of students of the university

5. **Write a struct `AirFleet` that contains a vector of `Airplane`. `Airplane` is a struct that contains a `company` (an enum called `AirplaneCompany` with options: Airbus or Boeing) and a `model` (String). Implement the following methods for `AirFleet`:**

   - `remove_boeing`: remove all the airplanes of the company Boeing from the fleet
   - `add_airplane`: add an airplane to the fleet
   - `search_airplane`: search an airplane by model and return the company of the airplane, it must return a `Result<AirplaneCompany, String>`
     - The function must return `OK` if the airplane is found, `Err` if the airplane is not found, or the fleet is empty.

6. **Create the module `hashmaps` that contains a struct `Maps` with a field `map` of type `HashMap

<Unumber, String>`, then create the module `unumber` that contains a type `Unumber` of type `usize`. In another module create a function `string_to_tuple` that takes a `Maps` and returns a `HashMap<Unumber, (Unumber, String)>`. The function should convert the `HashMap<Unumber, String>` to a `HashMap<Unumber, (Unumber, String)>`, the key remains the same, and the value is a tuple, its first field is the length of the `String` and its second field is the `String` itself.**

7. **Write a struct `Size` that has two fields: `width` and `height`, both of type `u32`. Then, implement the following methods for `Size`:**

   - `new`: takes two `u32` arguments and returns a `Size` instance
   - `area`: takes a reference to `self` and returns the area of the `Size` instance
   - `compare`: takes a reference to `self` and another `Size` instance and returns an `Option<bool>`:
     - `None` if the two `Size` instances have the same area
     - `Some(true)` if the area of the first `Size` instance is greater than the area of the second `Size` instance
     - `Some(false)` if the area of the first `Size` instance is less than the area of the second `Size` instance

8. **Write a struct `MaybePoint` that has two fields: `x` and `y`, both of type `Option<i32>`. Then, implement the following methods for `MaybePoint`:**

   - `new`: takes two `Option<i32>` arguments and returns a `MaybePoint` instance
   - `is_some`: takes a reference to `self` and returns `true` if both `x` and `y` are `Some` values, `false` otherwise
   - `maybe_len`: takes a reference to `self` and returns an `Option<f32>`:
     - `None` if `x` or `y` are `None`
     - `Some(len)` where `len` is the length of the vector from `(0, 0)` to `(x, y)`

9. **Write a function `res1` that takes an `i32` and returns a `Result<i32, String>`. The function returns:**

   - `Ok(n)` if `n` is divisible by 10, `Err("error")` otherwise. 
   - Then write a function `res2` that takes a `Result<i32, &str>` and returns a `Result<i32, String>`. The function returns `Ok(n)` if `n` is divisible by 5, `Err("error")` otherwise.
   - Then write a wrapper function that takes an `i32` and returns a `Result<i32, String>`. The function returns `Ok(n)` if `n` is divisible by 10 and 5, `Err("error")` otherwise. Errors should be propagated.

10. **Create a function `order` that takes a `Vec` of `String`s and returns a vector of `String`s, where each string is prepended by its index in the vector followed by a dash and a space. For example, given the vector `["How", "Are", "You"]` the function should return `["0 - How", "1 - Are", "2 - You"]`.**

11. **Define two enums both called `X`, place them in two different modules, `modx` and `mody`. Define the enums like this:**

    - With 2 variants `S` with a `char` and `C` with a `String`
    - With 1 variant `F` with a `f64` and a `usize`
    - Write a function `sum` in the module `modsum` that takes an `X` from `modx` and an `X` from `mody`, the function returns the `u8` equivalent of the `char` or the length of the `String` based on the variant, summed with the product of `f64` by the `usize`.

    The modules can be put in the same file, in this way:

    ```rust
    mod modx {
      // insert your enum
    }

    mod mody {
      // insert your enum
    }

    mod modsum {  
      use super::modx::X as X1;
      use super::mody::X as X2;

      fn sum(x1: X1, x2: X2) -> u8 {
        // insert your code here
      }
    }
    ```

This markdown should now reflect the structure and content based on the blueprint you provided.