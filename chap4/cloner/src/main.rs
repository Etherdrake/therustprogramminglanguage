fn main() {
    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
                        // .. and is nog longer valid here

    let x = 5; // comes into scope

    makes_copy(x); // x would move into the function,
                   // but i32 is Copy, so it's okay to still
                   // use x afterward
    return_main();

    transfer_ownership();

} // Here, x goes out of scope, then s. But because s's value was moved,
  // nothing special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and 'drop' is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn return_main() {
    let s1 = gives_ownership(); // gives_ownership moves its return
                                // value into s1
    println!("{}", s1);

    let s2 = String::from("hello"); // s2 comes into scope

    println!("{}", s2);

    let s3 = takes_and_gives_back(s2); // s2 is moved into takes_and_gives_backs,
                                       // which also moves its return value into s3
    println!("{}", s3);
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing happens.
  // s1, goes out of scope and is dropped.



fn gives_ownership() -> String {       // gives_ownership will move its return value into
                                       // the function that calls it.
    let some_string = String::from("yours"); // some_string comes into scope

    some_string                        // some_string is returned and moves out to the calling function

}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope

    a_string // a_string is returned and moves out to the calling function

}

fn transfer_ownership() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}