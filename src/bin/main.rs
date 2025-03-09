use std::{borrow::Borrow, fmt::Display};

fn takes_borrowed<T: Borrow<str> + Display>(s: T) {
    println!("{}", s); // Borrow as &str
}

fn takes_ref<T: AsRef<str> + Display>(s: T) {
    println!("{}", s); // Borrow as &str
}

fn main() {
    let my_string = String::from("Hello, Rust!");
    let t = my_string.as_ref();
    takes_borrowed(t); // Works with &String
    takes_ref(t);  // Also works with owned String
    println!("{}", my_string);
}