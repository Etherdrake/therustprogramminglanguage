fn main() {                 // s is not valid here, it's not yet declared
    let s = "hello";  // s is valid from this point forward

    // do stuff with s

    mover()
}                       // this scope is now over, and s is no longer valid

fn mutable_string() {
    let s = String::from("hello");

    let mut t = String::from("hello");

    t.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", t) // This will print 'hello, world!'
}

fn mover() {
    let x = 5;
    let y = x;

    let s1 = String::from("hello");
    let s2 = s1;

    println!("{}", s2)


}