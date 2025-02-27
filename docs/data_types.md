In Rust, data types are broadly categorized into **scalars** (like integers, floats, booleans, and characters) and **compound types** (like tuples, arrays, strings, structs, and enums). Below is a comprehensive list with **examples** for each.

---

# **1. Integer Types**

Rust provides both **signed (`i` types)** and **unsigned (`u` types)** integers.

| Type    | Signed/Unsigned | Size           | Value Range                         |
| ------- | --------------- | -------------- | ----------------------------------- |
| `i8`    | Signed          | 8-bit          | `-128` to `127`                     |
| `u8`    | Unsigned        | 8-bit          | `0` to `255`                        |
| `i16`   | Signed          | 16-bit         | `-32,768` to `32,767`               |
| `u16`   | Unsigned        | 16-bit         | `0` to `65,535`                     |
| `i32`   | Signed          | 32-bit         | `-2,147,483,648` to `2,147,483,647` |
| `u32`   | Unsigned        | 32-bit         | `0` to `4,294,967,295`              |
| `i64`   | Signed          | 64-bit         | `-9 quintillion` to `9 quintillion` |
| `u64`   | Unsigned        | 64-bit         | `0` to `18 quintillion`             |
| `i128`  | Signed          | 128-bit        | Very large                          |
| `u128`  | Unsigned        | 128-bit        | Very large                          |
| `isize` | Signed          | Arch dependent | `-2^N/2` to `2^N/2 - 1`             |
| `usize` | Unsigned        | Arch dependent | `0` to `2^N - 1`                    |

### **Examples**

```rust
fn main() {
    let a: u8 = 255;
    let b: i8 = -128;
    let c: i32 = 10_000; // Underscore for readability
    let d: usize = 42; // Architecture-dependent

    println!("a: {}, b: {}, c: {}, d: {}", a, b, c, d);
}
```

---

# **2. Floating Point Types**

Rust has two floating-point types:

- `f32` (32-bit precision)
- `f64` (64-bit precision, default)

### **Example**

```rust
fn main() {
    let x: f32 = 3.14;
    let y: f64 = 2.71828;

    println!("x: {}, y: {}", x, y);
}
```

---

# **3. Boolean Type (`bool`)**

Booleans in Rust store `true` or `false`.

### **Example**

```rust
fn main() {
    let is_rust_fun: bool = true;
    let is_hard: bool = false;

    println!("Rust is fun? {}", is_rust_fun);
    println!("Rust is hard? {}", is_hard);
}
```

---

# **4. Character Type (`char`)**

The `char` type represents a **Unicode scalar value**, allowing emojis and special characters.

### **Example**

```rust
fn main() {
    let letter = 'A';
    let emoji = '😀';

    println!("Letter: {}, Emoji: {}", letter, emoji);
}
```

---

# **5. String Types (`String` and `&str`)**

### **Differences**

- `String`: Growable, stored on **heap**.
- `&str`: Immutable string slice, stored on **stack** or part of a `String`.

### **Examples**

```rust
fn main() {
    let s1: String = String::from("Hello, Rust!"); // Heap allocated
    let s2: &str = "Hello, World!"; // String slice

    println!("String: {}", s1);
    println!("Str slice: {}", s2);
}
```

### **Modifying a `String`**

```rust
fn main() {
    let mut s = String::from("Hello");
    s.push_str(", Rust!"); // Append
    println!("{}", s);
}
```

---

# **6. Arrays (`[T; N]`)**

Fixed-size, stored on the **stack**.

### **Example**

```rust
fn main() {
    let numbers: [i32; 3] = [1, 2, 3];
    let repeated = [0; 5]; // [0, 0, 0, 0, 0]

    println!("First number: {}", numbers[0]);
    println!("Repeated array: {:?}", repeated);
}
```

---

# **7. Tuples (`(T1, T2, T3, ...)`)**

Fixed-size, can hold multiple types.

### **Example**

```rust
fn main() {
    let person: (&str, i32, f64) = ("Alice", 30, 5.5);

    println!("Name: {}, Age: {}, Height: {}", person.0, person.1, person.2);
}
```

---

# **8. Structs**

Used to group related data.

### **1. Basic Struct**

```rust
struct Person {
    name: String,
    age: u8,
}

fn main() {
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };

    println!("Name: {}, Age: {}", person.name, person.age);
}
```

### **2. Tuple Struct**

```rust
struct Point(i32, i32);

fn main() {
    let p = Point(10, 20);
    println!("Point: ({}, {})", p.0, p.1);
}
```

### **3. Struct with Methods**

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect = Rectangle { width: 30, height: 50 };
    println!("Area: {}", rect.area());
}
```

---

# **9. Enums**

Used to define **multiple variants** of a type.

### **Example**

```rust
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let dir = Direction::Left;

    match dir {
        Direction::Up => println!("Moving up"),
        Direction::Down => println!("Moving down"),
        Direction::Left => println!("Moving left"),
        Direction::Right => println!("Moving right"),
    }
}
```

### **Enum with Data**

```rust
enum Message {
    Text(String),
    Move { x: i32, y: i32 },
    Quit,
}

fn main() {
    let msg = Message::Move { x: 10, y: 20 };

    match msg {
        Message::Text(text) => println!("Text message: {}", text),
        Message::Move { x, y } => println!("Move to ({}, {})", x, y),
        Message::Quit => println!("Quitting"),
    }
}
```

---

# **10. Option and Result**

### **1. `Option<T>` (Nullable Alternative)**

```rust
fn divide(x: i32, y: i32) -> Option<i32> {
    if y == 0 {
        None
    } else {
        Some(x / y)
    }
}

fn main() {
    let result = divide(10, 2);
    match result {
        Some(v) => println!("Result: {}", v),
        None => println!("Cannot divide by zero"),
    }
}
```

### **2. `Result<T, E>` (Error Handling)**

```rust
fn read_file() -> Result<String, std::io::Error> {
    std::fs::read_to_string("example.txt")
}

fn main() {
    match read_file() {
        Ok(content) => println!("File Content: {}", content),
        Err(e) => println!("Error reading file: {}", e),
    }
}
```

---

# **Conclusion**

Rust has a rich type system, from basic integers to powerful enums and structs. Each type serves a specific purpose, ensuring **safety and performance**.

Would you like more examples on a specific type? 🚀
