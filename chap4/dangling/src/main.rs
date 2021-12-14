fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String { // dangle returns a reference to a String
    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!

fn main_sol() {
    let reference_to_nothing = dangle_sol();
}

fn dangle_sol() -> String { // dangle returns a String
    let s = String::from("hello"); // s is a new String

    s // we return a String, s
} // Here, s goes out of scope, but ownership is moved out and nothing is deallocated.