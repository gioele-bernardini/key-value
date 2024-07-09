define the Nextable trait with a method `gimme_next`
implement Nextable for i32, `gimme_next` returns the optional successor of self
implement Nextable for char, `gimme_next` returns the optional new char that is the next char (as a u32 conversion)
implement a function `printnext` that takes a `Nextable`
and prints the argument and its `gimme_next` using the ":?" formatter
     It behaves as the example:
    next of 5 is Some(6)
    next of 's' is Some('t')

Test 	Result

pub fn main() {
        let x = 5;
        printnext(x);
        let s = 's';
        printnext(s);
    }

	

next of 5 is Some(6)
next of 's' is Some('t')

pub fn main() {
        let x = 11;
        printnext(x);
        let s = 'f';
        printnext(s);
    }

	

next of 11 is Some(12)
next of 'f' is Some('g')







Define a struct `Wrapper` that contains a field `v` of type `Vec<String>`
define an iterator for `Wrapper` to cycle over the elements of the vector
instead of returning a pointer to the elements of v, the iterator returns
a the length of the elements of v

Test 	Result

    pub fn main(){
        let w = Wrapper{v:vec!["nope".to_string(),"who".to_string(),"gimme".to_string()]};
        let mut iw = w.iter();
        println!("first: {}",iw.next().unwrap());
        for el in iw{
            println!("lens: {}",el);
        }
    }

	

first: 4
lens: 3
lens: 5

    pub fn main(){
        let w = Wrapper{v:vec!["why".to_string(),"whose".to_string()]};
        let mut iw = w.iter();
        println!("first: {}",iw.next().unwrap());
        for el in iw{
            println!("lens: {}",el);
        }
    }

	

first: 3
lens: 5







write a function `basicbox_sum` that takes a vector of Strings and returns a vector of Boxes of usizes
the returned vector contains all the lengths of the input vector followed by a final element that sums all the previous lengths



Test 	Result

        pub fn main() {
            let s = vec!["asd".to_string(), "where".to_string(), "what".to_string()];
            println!("boxed s: {:?}", basicbox_sum(s));
        }

	

boxed s: [3, 5, 4, 12]

        pub fn main() {
            let s = vec!["nope".to_string(), "game".to_string(), "bananas".to_string()];
            println!("boxed s: {:?}", basicbox_sum(s));
        }

	

boxed s: [4, 4, 7, 15]














Create a struct `SharedCommunications` that derives Debug with the following methods:

-[1] `new()->Self`: create a new communication object connected to no one, with no message inside.

-[1] `new_form(other: &Self)->Self`: create a new communication object connected to other.

-[2] `send(&mut self, message: String)->Result<(),()>`: try to send a message... if the structure already has a message inside, it returns an error. otherwise it memorize the message and return Ok.

-[2] `receive(&mut self)->Option<String>`: if the structure has a message inside it returns it. otherwise returns None.

the struct implement a kind of blocking pipe, where message can be sent only if the previous message has been received. The object must be sharable between multiple owners using the `new_form` method.  

If you found this description confusing, please read the test cases, and it will become clearer.














Test 	Result

fn main() {
    let mut c1 = SharedCommunications::new();

    println!("{:?}",c1);
}

	

SharedCommunications { message: RefCell { value: None } }

fn main() {
    let mut c1 = SharedCommunications::new();
    let mut c2 = SharedCommunications::new_form(&c1);

    println!("{:?}",c2);
}

	

SharedCommunications { message: RefCell { value: None } }

fn main() {
    let mut c1 = SharedCommunications::new();
    let mut c2 = SharedCommunications::new_form(&c1);
    let mut c3 = SharedCommunications::new_form(&c1);
    let mut c4 = SharedCommunications::new_form(&c3);

    println!("{:?}",c1.receive());
    println!("{:?}",c2.send("hello".to_owned()));
    println!("{:?}",c3.send("hello2".to_owned()));
    println!("{:?}",c4.receive());
}

	

None
Ok(())
Err(())
Some("hello")

fn main() {
    let mut c1 = SharedCommunications::new();
    let mut c2 = SharedCommunications::new_form(&c1);

    println!("{:?}",c1.send("hello".to_owned()));
    println!("{:?}",c2.receive());
    println!("{:?}",c1.send("hello2".to_owned()));
    println!("{:?}",c2.receive());
    println!("{:?}",c1.send("hello3".to_owned()));
    println!("{:?}",c2.receive());
    println!("{:?}",c1.send("hello4".to_owned()));
    println!("{:?}",c2.receive());
}

	

Ok(())
Some("hello")
Ok(())
Some("hello2")
Ok(())
Some("hello3")
Ok(())
Some("hello4")

















define a module "finance". inside it, define two public modules "wallet_1" and "wallet_2".

[1] define a struct "Wallet" inside "wallet_1" with an attribute "euro" with type f32.

[1] define a struct "Wallet" inside "wallet_2" with an attribute "euro" with type u32, and an attribute "cents" with type u8

derive Debug on both "Wallet", and make all attributes public

create two public alias inside "finance":

- Wallet1 for wallet_1::Wallet

- Wallet2 for wallet_2::Wallet

[2] define a public function "compare_wallet" in the module "finance" that takes two arguments: "first" with type "&Wallet1" and "second" with type "&Wallet2" the function returns true if "first" has more money that "second", otherwise it returns false










Test 	Result

fn main() {
    let w1 = finance::wallet_1::Wallet{
        euro: 100.
    };
    print!("{:?}",w1);
}

	

Wallet { euro: 100.0 }

fn main() {
    let w2 = finance::wallet_2::Wallet{
        euro: 199,
        cents: 50
    };
    print!("{:?}",w2);
}

	

Wallet { euro: 199, cents: 50 }

fn main() {
    let w1 = finance::wallet_1::Wallet{
        euro: 100.
    };
    let w2 = finance::Wallet2{
        euro: 90,
        cents: 50
    };
    print!("{}",finance::compare_wallet(&w1,&w2));
}

	

true

