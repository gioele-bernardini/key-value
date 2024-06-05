### Mockup Midterm 3

#### Part 1: Multiple Choice on Rust Semantics

1. **A: Will the following code compile? Why?**

```rust
fn main() {
    let s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{}, {}", r1, r2);
}
```

Answers:
1. Yes
2. No, s is moved
3. No, r1 is borrowed
4. No, s does not implement Copy

**B: If not, how to fix it?**

Answers:
1. Replace `let r1 = &s;` with `let r1 = s.clone();`
2. Replace `let r2 = &s;` with `let r2 = s.clone();`
3. Change `s` to `mut s`
4. Add `println!("{}", s);`

2. **A: Will the following code compile? Why?**

```rust
fn main() {
    let x = 5;
    let y = &x;
    println!("{}", y);
    println!("{}", x);
}
```

Answers:
1. Yes
2. No, y is moved
3. No, x is immutable
4. No, y does not implement Copy

**B: If not, how to fix it?**

Answers:
1. Replace `let y = &x;` with `let y = x.clone();`
2. Add `println!("{}", x);` after line 2
3. Change `x` to `mut x`
4. Use `y.clone()` in line 3

3. **A: What is the owner of `v`'s original data at `HERE`?**

```rust
fn main() {
    let v = vec![1, 2, 3];
    let w = &v;
    // HERE
}
```

1. v
2. w
3. main
4. none

**B: When is the value of `v` dropped?**

Answers:
1. After line 1
2. After line 2
3. After line 3
4. When the scope ends

4. **A

: Will the following code compile? Why?**

```rust
fn foo(vec: Vec<i32>) {
    let x = vec;
    let y = x.clone();
    println!("{:?}", y);
}
```

Answers:
1. Yes
2. No, vec is moved
3. No, y is not used
4. No, vec does not implement Clone

**B: If not, how to fix it?**

Answers:
1. Replace line 3 with `let y = vec.clone();`
2. Add `let y = vec;`
3. Change `vec` to `mut vec`
4. Use `vec.clone()` in line 3

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
2. No, x is moved
3. No, x is borrowed
4. No, x does not implement Copy

6. **Does the following code compile?**

```rust
fn main() {
    let mut x = 5;
    let y = &mut x;
    *y += 1;
    println!("{}", x);
}
```

Answers:
1. Yes
2. No, x is immutable
3. No, y cannot be mutable
4. No, x is not used

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

1. **Write a function `lowercase` that takes a `&str` as input and returns a `String`.**
   
   This function should convert all characters to lowercase.
   

2. **Write a struct `Car` with two fields: `brand` (a `String`) and `year` (an `i32`). Then, implement the following methods for `Car`:**

   - `new`: takes a `&str` and an `i32` and returns a `Car` instance.
   - `description`: returns a string describing the car.


3. **Create a function named `count_words` that takes a `&str` and returns the number of words.**


4. **Write a struct `Circle` with a single field: `radius` (a `f64`). Implement the following methods for `Circle`:**

   - `new`: takes a `f64` argument and returns a `Circle` instance.
   - `area`: returns the area of the circle.


5. **Write a struct `Teacher` with two fields: `name` (a `String`) and `subject` (a `String`). Then, implement the `Display` trait for `Teacher` to print "Teacher: [name], Subject: [subject]".**


### Mockup Midterm 4

#### Part 1: Multiple Choice on Rust Semantics

