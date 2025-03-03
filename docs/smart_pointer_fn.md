Rust provides several built-in utility functions for working with references, borrowing, and conversions. Here’s an explanation of some commonly used ones, along with examples:

---

## 1. **`as_ref()`**

Converts `&T` into `&U` if `T: AsRef<U>`. It is commonly used for generic code where you want to obtain a reference to a value in a uniform way.

### Example:

```rust
fn print_length<T: AsRef<str>>(s: T) {
    println!("Length: {}", s.as_ref().len());
}

fn main() {
    let string = String::from("Hello");
    let str_slice: &str = "World";

    print_length(string); // Works with String
    print_length(str_slice); // Works with &str
}
```

**Use Case**: When writing functions that accept multiple reference-like types (e.g., `String`, `&str`, `Vec<T>`, `&[T]`, etc.).

---

## 2. **`as_mut()`**

Similar to `as_ref()`, but for mutable references (`&mut T` to `&mut U`).

### Example:

```rust
fn make_uppercase<T: AsMut<String>>(mut s: T) {
    s.as_mut().make_ascii_uppercase();
}

fn main() {
    let mut string = String::from("hello");
    make_uppercase(&mut string);
    println!("{}", string); // Output: HELLO
}
```

**Use Case**: Useful for modifying owned types generically.

---

## 3. **`borrow()`**

Used for borrowing references, commonly used with smart pointers like `Rc<T>` and `RefCell<T>`. Requires the `Borrow` trait.

### Example:

```rust
use std::borrow::Borrow;

fn print_borrowed<T: Borrow<str>>(s: T) {
    println!("{}", s.borrow());
}

fn main() {
    let string = String::from("Hello");
    print_borrowed(&string);
}
```

**Use Case**: Helps in writing functions that work with both owned and borrowed data.

---

## 4. **`borrow_mut()`**

Similar to `borrow()`, but for mutable references. Requires `BorrowMut` trait.

### Example:

```rust
use std::borrow::BorrowMut;

fn modify<T: BorrowMut<String>>(mut s: T) {
    s.borrow_mut().push_str(" World!");
}

fn main() {
    let mut string = String::from("Hello");
    modify(&mut string);
    println!("{}", string); // Output: Hello World!
}
```

**Use Case**: Useful when working with `RefCell<T>` to get a mutable reference dynamically.

---

## 5. **`deref()`**

Used for dereferencing smart pointers like `Box<T>` and `Rc<T>`. Implemented via the `Deref` trait.

### Example:

```rust
use std::ops::Deref;

struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let x = MyBox(5);
    println!("{}", *x); // Output: 5
}
```

**Use Case**: Enables custom smart pointers to behave like regular references.

---

## 6. **`deref_mut()`**

Similar to `deref()`, but for mutable references.

### Example:

```rust
use std::ops::DerefMut;

struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn main() {
    let mut x = MyBox(5);
    *x += 10;
    println!("{}", *x); // Output: 15
}
```

**Use Case**: Enables mutation when working with custom smart pointers.

---

## 7. **`try_into()`**

Attempts to convert a value into another type, returning `Result<T, E>`. Requires `TryInto` trait.

### Example:

```rust
fn main() {
    let num: i32 = 10;
    let result: Result<u8, _> = num.try_into(); // Convert i32 -> u8
    match result {
        Ok(v) => println!("Converted: {}", v),
        Err(e) => println!("Error: {}", e),
    }
}
```

**Use Case**: Useful for fallible conversions where the conversion may fail.

---

## 8. **`into()`**

Consumes `self` and converts it into another type. Requires `Into` trait.

### Example:

```rust
fn takes_string(s: String) {
    println!("{}", s);
}

fn main() {
    let s: &str = "Hello";
    takes_string(s.into()); // Converts &str -> String
}
```

**Use Case**: Used in APIs where conversion should be implicit.

---

## 9. **`to_string()`**

Converts a type into a `String`.

### Example:

```rust
fn main() {
    let num = 42;
    let str_num = num.to_string();
    println!("{}", str_num); // Output: "42"
}
```

**Use Case**: Works on types implementing `Display` or `ToString`.

---

## 10. **`clone()`**

Creates a duplicate of a value if it implements `Clone`.

### Example:

```rust
fn main() {
    let s1 = String::from("Hello");
    let s2 = s1.clone();
    println!("{}", s2); // Output: Hello
}
```

**Use Case**: Used when explicit duplication of data is needed.

---

## 11. **`copy()`**

Used for types implementing `Copy` (like integers, booleans, and certain small structs).

### Example:

```rust
fn main() {
    let x = 5;
    let y = x; // Implicit copy
    println!("{}", y);
}
```

**Use Case**: Enables copying without explicit `clone()`.

---

## 12. **`default()`**

Creates a default value for a type implementing `Default`.

### Example:

```rust
#[derive(Default)]
struct Config {
    debug: bool,
}

fn main() {
    let config = Config::default();
    println!("{}", config.debug); // Output: false
}
```

**Use Case**: Used for initializing types with sensible defaults.

---

### Summary Table

| Function       | Description                                        |
| -------------- | -------------------------------------------------- |
| `as_ref()`     | Convert `&T` to `&U` generically.                  |
| `as_mut()`     | Convert `&mut T` to `&mut U` generically.          |
| `borrow()`     | Borrow `T` as `&U`, mainly for smart pointers.     |
| `borrow_mut()` | Mutable borrow of `T` as `&mut U`.                 |
| `deref()`      | Deref smart pointers (`Box`, `Rc`, etc.).          |
| `deref_mut()`  | Mutable deref smart pointers.                      |
| `try_into()`   | Fallible conversion, returns `Result<T, E>`.       |
| `into()`       | Consumes `self` and converts to another type.      |
| `to_string()`  | Converts a type to a `String`.                     |
| `clone()`      | Creates a duplicate of a value.                    |
| `copy()`       | Performs implicit bitwise copy (for `Copy` types). |
| `default()`    | Generates a default instance of a type.            |

---

These functions and traits are heavily used in Rust for ergonomic and generic programming. Let me know if you need more explanations or examples! 🚀
