### Exercise 1: Implementing the `Nextable` Trait

Define the `Nextable` trait with a method `gimme_next`.

1. Implement `Nextable` for `i32`, where `gimme_next` returns the optional successor of `self`.
2. Implement `Nextable` for `char`, where `gimme_next` returns the optional new `char` that is the next character (as a `u32` conversion).
3. Implement a function `printnext` that takes a `Nextable` and prints the argument and its `gimme_next` using the `:?` formatter.

Example:
```rust
pub fn main() {
    let x = 5;
    printnext(x);
    let s = 's';
    printnext(s);
}
// Expected output:
// next of 5 is Some(6)
// next of 's' is Some('t')

pub fn main() {
    let x = 11;
    printnext(x);
    let s = 'f';
    printnext(s);
}
// Expected output:
// next of 11 is Some(12)
// next of 'f' is Some('g')
```

### Exercise 2: Creating an Iterator for a Struct

Define a struct `Wrapper` that contains a field `v` of type `Vec<String>`. 

1. Define an iterator for `Wrapper` to cycle over the elements of the vector.
2. Instead of returning a pointer to the elements of `v`, the iterator returns the length of the elements of `v`.

Example:
```rust
pub fn main(){
    let w = Wrapper{v:vec!["nope".to_string(),"who".to_string(),"gimme".to_string()]};
    let mut iw = w.iter();
    println!("first: {}",iw.next().unwrap());
    for el in iw{
        println!("lens: {}",el);
    }
}
// Expected output:
// first: 4
// lens: 3
// lens: 5

pub fn main(){
    let w = Wrapper{v:vec!["why".to_string(),"whose".to_string()]};
    let mut iw = w.iter();
    println!("first: {}",iw.next().unwrap());
    for el in iw{
        println!("lens: {}",el);
    }
}
// Expected output:
// first: 3
// lens: 5
```

### Exercise 3: Function to Return Vector of Boxed Lengths

Write a function `basicbox_sum` that takes a vector of `String`s and returns a vector of `Box`es of `usize`s. The returned vector contains all the lengths of the input vector followed by a final element that sums all the previous lengths.

Example:
```rust
pub fn main() {
    let s = vec!["asd".to_string(), "where".to_string(), "what".to_string()];
    println!("boxed s: {:?}", basicbox_sum(s));
}
// Expected output:
// boxed s: [3, 5, 4, 12]

pub fn main() {
    let s = vec!["nope".to_string(), "game".to_string(), "bananas".to_string()];
    println!("boxed s: {:?}", basicbox_sum(s));
}
// Expected output:
// boxed s: [4, 4, 7, 15]
```

### Exercise 4: Struct `SharedCommunications`

Create a struct `SharedCommunications` that derives `Debug` with the following methods:

1. `new() -> Self`: Create a new communication object connected to no one, with no message inside.
2. `new_form(other: &Self) -> Self`: Create a new communication object connected to `other`.
3. `send(&mut self, message: String) -> Result<(), ()>`: Try to send a message. If the structure already has a message inside, it returns an error. Otherwise, it memorizes the message and returns `Ok`.
4. `receive(&mut self) -> Option<String>`: If the structure has a message inside, it returns it. Otherwise, returns `None`.

The struct implements a kind of blocking pipe, where messages can be sent only if the previous message has been received. The object must be shareable between multiple owners using the `new_form` method.

Example:
```rust
fn main() {
    let mut c1 = SharedCommunications::new();
    println!("{:?}", c1);
}
// Expected output:
// SharedCommunications { message: RefCell { value: None } }

fn main() {
    let mut c1 = SharedCommunications::new();
    let mut c2 = SharedCommunications::new_form(&c1);
    println!("{:?}", c2);
}
// Expected output:
// SharedCommunications { message: RefCell { value: None } }

fn main() {
    let mut c1 = SharedCommunications::new();
    let mut c2 = SharedCommunications::new_form(&c1);
    let mut c3 = SharedCommunications::new_form(&c1);
    let mut c4 = SharedCommunications::new_form(&c3);

    println!("{:?}", c1.receive());
    println!("{:?}", c2.send("hello".to_owned()));
    println!("{:?}", c3.send("hello2".to_owned()));
    println!("{:?}", c4.receive());
}
// Expected output:
// None
// Ok(())
// Err(())
// Some("hello")

fn main() {
    let mut c1 = SharedCommunications::new();
    let mut c2 = SharedCommunications::new_form(&c1);

    println!("{:?}", c1.send("hello".to_owned()));
    println!("{:?}", c2.receive());
    println!("{:?}", c1.send("hello2".to_owned()));
    println!("{:?}", c2.receive());
    println!("{:?}", c1.send("hello3".to_owned()));
    println!("{:?}", c2.receive());
    println!("{:?}", c1.send("hello4".to_owned()));
    println!("{:?}", c2.receive());
}
// Expected output:
// Ok(())
// Some("hello")
// Ok(())
// Some("hello2")
// Ok(())
// Some("hello3")
// Ok(())
// Some("hello4")
```

### Exercise 5: Finance Module

Define a module `finance`. Inside it, define two public modules `wallet_1` and `wallet_2`.

1. Define a struct `Wallet` inside `wallet_1` with an attribute `euro` of type `f32`.
2. Define a struct `Wallet` inside `wallet_2` with an attribute `euro` of type `u32`, and an attribute `cents` of type `u8`.
3. Derive `Debug` on both `Wallet`, and make all attributes public.
4. Create two public aliases inside `finance`:
   - `Wallet1` for `wallet_1::Wallet`
   - `Wallet2` for `wallet_2::Wallet`
5. Define a public function `compare_wallet` in the module `finance` that takes two arguments: `first` of type `&Wallet1` and `second` of type `&Wallet2`. The function returns `true` if `first` has more money than `second`, otherwise it returns `false`.

Example:
```rust
fn main() {
    let w1 = finance::wallet_1::Wallet{
        euro: 100.
    };
    print!("{:?}", w1);
}
// Expected output:
// Wallet { euro: 100.0 }

fn main() {
    let w2 = finance::wallet_2::Wallet{
        euro: 199,
        cents: 50
    };
    print!("{:?}", w2);
}
// Expected output:
// Wallet { euro: 199, cents: 50 }

fn main() {
    let w1 = finance::wallet_1::Wallet{
        euro: 100.
    };
    let w2 = finance::Wallet2{
        euro: 90,
        cents: 50
    };
    print!("{}", finance::compare_wallet(&w1, &w2));
}
// Expected output:
// true
```


