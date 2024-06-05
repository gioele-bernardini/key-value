### Mockup Midterm 2

#### Part 1: Multiple Choice on Rust Semantics

1. **A: Will the following code compile? Why?**

```rust
fn main() {
    let mut s = String::from("hello");
    let t = &mut s; // line 3
    t.push_str(", world!");
    println!("{}", s);
}
```

Answers:
1. Yes
2. No, s is borrowed as mutable
3. No, t cannot be used after s
4. No, s does not implement the Copy trait

**B: If not, how to fix it?**

Answers:
1. Replace line 3 with `let t = &s;`
2. Replace line 3 with `let t = s.clone();`
3. Change `mut` to `const` in line 2
4. Add `let s = String::new();` before line 1

2. **A: Will the following code compile? Why?**

```rust
fn main() {
    let arr = [1, 2, 3, 4];
    let slice = &arr[1..3];
    println!("{:?}", slice);
    let first = &slice[0];
    println!("{:?}", arr);
}
```

Answers:
1. Yes
2. No, slice out of bounds
3. No, arr is moved
4. No, slice does not implement Debug

**B: If not, how to fix it?**

Answers:
1. Replace `slice` with `&arr[..]`
2. Replace `slice` with `arr`
3. Add `println!("{:?}", arr);` after line 2
4. Add `println!("{:?}", slice);` after line 3

3. **A: What is the owner of `vec`'s original data at `HERE`?**

```rust
fn main() {
    let vec = vec![1, 2, 3];
    let y = vec;
    // HERE
}
```

1. vec
2. y
3. main
4. none

**B: When is the value of `y` dropped?**

Answers:
1. After line 1
2. After line 2
3. After line 3
4. When the scope ends

4. **A: Will the following code compile? Why?**

```rust
fn foo(vec: Vec<i32>) {
    let x = vec;
    let y = x;
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

1. **Write a function `capitalize` that takes a `&str` as input and returns a `String`.**
   
   This function should convert the first letter of each word to uppercase.


2. **Write a struct `Book` with two fields: `title` (a `String`) and `pages` (an `i32`). Then, implement the following methods for `Book`:**

   - `new`: takes a `&str` and an `i32` and returns a `Book` instance.
   - `description`: returns a string describing the book.


3. **Create a function named `reverse_string` that takes a `String` and returns the reversed `String`.**


4. **Write a struct `Rectangle` with two fields: `width` and `height`, both `u32`. Implement the following methods for `Rectangle`:**

   - `new`: takes two `u32` arguments and returns a `Rectangle` instance.
   - `area`: returns the area of the rectangle.


5. **Write a struct `Person` with two fields: `name` (a `String`) and `age` (an `u32`). Then, implement the `Display` trait for `Person` to print "Name: [name], Age: [age]".**

