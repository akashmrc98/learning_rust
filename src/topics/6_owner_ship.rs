fn basics() {
    let s: &str = "hello"; // s is valid from this point forward
    println!("{}", s);

    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", s);
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

fn topic() {
    // to string vars
    let s1 = String::from("hello");
    println!("s1 = {}", s1);

    // borrowing s1 for s2
    let s2 = s1;

    // s1 is freed from memeory
    println!("s2 = {}", s2);

    // cant able access s1 here
    // throws error, because it was borrowed to s2 and freed
    // println!("s1 = {}", s1);

    // in order to make s1 work use clone
    let s1 = String::from("hello");
    // cloning s1 for s2
    let s2 = s1.clone();

    // now s1 also in memory can be accessed
    println!("s1 = {}, s2 = {}", s1, s2);

    // contradiction
    // for fixed data types with fixed lengths it auto clones instead of borrowing
    // it is only exceptional for data types with dynamic lengths
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);

    // Rust has a special annotation called the Copy trait that we can place on types that are stored on the stack, as integers are (we’ll talk more about traits in Chapter 10). If a type implements the Copy trait, variables that use it do not move, but rather are trivially copied, making them still valid after assignment to another variable.

    //Rust won’t let us annotate a type with Copy if the type, or any of its parts, has implemented the Drop trait. If the type needs something special to happen when the value goes out of scope and we add the Copy annotation to that type, we’ll get a compile-time error. To learn about how to add the Copy annotation to your type to implement the trait, see “Derivable Traits” in Appendix C.

    //Rust won’t let us annotate a type with Copy if the type, or any of its parts, has implemented the Drop trait. If the type needs something special to happen when the value goes out of scope and we add the Copy annotation to that type, we’ll get a compile-time error. To learn about how to add the Copy annotation to your type to implement the trait, see “Derivable Traits” in Appendix C.

    // all ints
    // all floats
    // char type
    // bool with true, false
    // tuples

    let s = String::from("hello"); // s comes into scope
    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here

    let x = 5; // x comes into scope

    makes_copy(x); // x would move into the function,
                   // but i32 is Copy, so it's okay to still
                   // use x afterward

    // example
    // give ownership to calculate_length and take back as s2, len
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate(s: &String) -> usize {
    s.len()
}

fn ref_modify(s: &mut String) -> usize {
    s.push_str("string");
    return s.len();
}

fn referencing() {
    // referencing should not modified
    let s1 = String::from("hello");
    let len = calculate(&s1);
    println!("The length of '{}' is {}.", s1, len);
    let mut s1: String = String::from("ok");
    ref_modify(&mut s1);
    println!("The length of '{}' is {}.", s1, len);

    // in order to modify the references
    // note :also we cannot borrow mutable reference twice
    // in order to overcome this we can use scopes

    // we also cannot have a mutable reference while we have an immutable one to the same value.
    //  let r1 = &s; // no problem
    // let r2 = &s; // no problem
    // let r3 = &mut s; // BIG PROBLEM

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
}

// fn dangle() -> &String {
// dangle returns a reference to a String
//     let s = String::from("hello"); // s is a new String
//     &s // we return a reference to the String, s
// }
// Here, s goes out of scope, and is dropped. Its memory goes away! // Danger!

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}

pub fn learn() {
    basics();
    topic();
    referencing();
    no_dangle();
}
