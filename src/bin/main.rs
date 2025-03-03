use std::borrow::Borrow;

fn takes_borrowed<T: Borrow<str>>(s: T) {
    println!("{}", s.borrow()); // Borrow as &str
}

fn main() {
    let my_string = String::from("Hello, Rust!");
    let t = my_string.borrow();
    takes_borrowed(t); // Works with &String
    takes_borrowed(t);  // Also works with owned String
    println!("{}", my_string);
}