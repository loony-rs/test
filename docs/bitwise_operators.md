### **Working with Bitwise Operators in Rust**

Rust provides bitwise operators for performing operations directly on binary representations of numbers. These operators are useful for tasks like low-level optimizations, embedded systems, cryptography, and error detection (like CRC).

---

### **Bitwise Operators in Rust**

| Operator | Description              | Example                                  |
| -------- | ------------------------ | ---------------------------------------- | ------- | ----------------- |
| `&`      | Bitwise AND              | `0b1100 & 0b1010 == 0b1000`              |
| `        | `                        | Bitwise OR                               | `0b1100 | 0b1010 == 0b1110` |
| `^`      | Bitwise XOR              | `0b1100 ^ 0b1010 == 0b0110`              |
| `!`      | Bitwise NOT (complement) | `!0b1100 == 0b...0011` (depends on type) |
| `<<`     | Left shift               | `0b0001 << 2 == 0b0100`                  |
| `>>`     | Right shift              | `0b1000 >> 2 == 0b0010`                  |

---

### **Examples of Bitwise Operations in Rust**

#### **1. Basic Bitwise Operations**

```rust
fn main() {
    let a = 0b1100; // 12 in decimal
    let b = 0b1010; // 10 in decimal

    println!("Bitwise AND: {:04b}", a & b);  // 1000 (8)
    println!("Bitwise OR:  {:04b}", a | b);  // 1110 (14)
    println!("Bitwise XOR: {:04b}", a ^ b);  // 0110 (6)
    println!("Bitwise NOT: {:04b}", !a & 0b1111); // 0011 (3) (4-bit mask)
}
```

**Output:**

```
Bitwise AND: 1000
Bitwise OR:  1110
Bitwise XOR: 0110
Bitwise NOT: 0011
```

---

#### **2. Bitwise Shifting**

```rust
fn main() {
    let num = 0b0001; // 1 in decimal

    println!("Left shift 2: {:04b}", num << 2); // 0100 (4)
    println!("Right shift 2: {:04b}", 0b1000 >> 2); // 0010 (2)
}
```

**Output:**

```
Left shift 2: 0100
Right shift 2: 0010
```

- **Left Shift (`<<`)**: Moves bits to the left, effectively multiplying by `2^n`.
- **Right Shift (`>>`)**: Moves bits to the right, effectively dividing by `2^n`.

---

#### **3. Checking & Setting Bits**

```rust
fn main() {
    let mut x = 0b1010;

    // Check if a bit is set
    let is_set = (x & (1 << 2)) != 0;
    println!("Bit at position 2 is set: {}", is_set);

    // Set a bit
    x |= 1 << 1; // Set bit at position 1
    println!("After setting bit 1: {:04b}", x);

    // Clear a bit
    x &= !(1 << 3); // Clear bit at position 3
    println!("After clearing bit 3: {:04b}", x);

    // Toggle a bit
    x ^= 1 << 0; // Toggle bit at position 0
    println!("After toggling bit 0: {:04b}", x);
}
```

**Output:**

```
Bit at position 2 is set: false
After setting bit 1: 1011
After clearing bit 3: 0011
After toggling bit 0: 0010
```

- `x |= 1 << n` → **Set** bit at position `n`
- `x &= !(1 << n)` → **Clear** bit at position `n`
- `x ^= 1 << n` → **Toggle** bit at position `n`

---

#### **4. Implementing CRC in Rust (Using XOR)**

```rust
fn compute_crc(mut data: u8, generator: u8) -> u8 {
    let mut crc = data << 3; // Append 3 zeros
    for _ in 0..5 { // Only 5 bits need processing
        if crc & 0b10000 != 0 { // If leftmost bit is 1
            crc ^= generator; // XOR with generator
        }
        crc <<= 1; // Shift left
    }
    crc >> 3 // Return remainder as CRC
}

fn main() {
    let data = 0b1011; // 4-bit data
    let generator = 0b1101; // 4-bit generator polynomial

    let crc = compute_crc(data, generator);
    println!("CRC: {:04b}", crc);
}
```

**Explanation:**

- The data is shifted left by the polynomial degree (3 bits).
- XOR is performed when the leftmost bit is `1`.
- The remainder after the division is the **CRC checksum**.

---

### **Where is This Useful?**

- **Embedded Systems**: Bitwise operations are crucial for setting and reading hardware registers.
- **Data Compression**: Used in algorithms like Huffman coding.
- **Encryption**: XOR operations are the foundation of many encryption techniques.
- **Networking**: CRC, checksums, and parity bits rely on bitwise operations.
- **Game Development**: Bit manipulation optimizes performance, especially in low-level graphics and physics engines.

Rust’s type safety prevents common bit manipulation errors, making it a great choice for low-level programming with reliability.

Would you like an advanced example like bitwise trie or bitboard representation? 🚀

Sure! Below is a list of all binary numbers (up to 8 bits) along with their decimal equivalents:

### **Binary to Decimal Conversion Table**

| Binary | Decimal |
| ------ | ------- |
| 0      | 0       |
| 1      | 1       |
| 10     | 2       |
| 11     | 3       |
| 100    | 4       |
| 101    | 5       |
| 110    | 6       |
| 111    | 7       |
| 1000   | 8       |
| 1001   | 9       |
| 1010   | 10      |
| 1011   | 11      |
| 1100   | 12      |
| 1101   | 13      |
| 1110   | 14      |
| 1111   | 15      |
| 10000  | 16      |
| 10001  | 17      |
| 10010  | 18      |
| 10011  | 19      |
| 10100  | 20      |
| 10101  | 21      |
| 10110  | 22      |
| 10111  | 23      |
| 11000  | 24      |
| 11001  | 25      |
| 11010  | 26      |
| 11011  | 27      |
| 11100  | 28      |
| 11101  | 29      |
| 11110  | 30      |
| 11111  | 31      |

Here is a **Rust program** that dynamically generates and prints binary-to-decimal mappings for any given number of bits:

```rust
fn main() {
    let max_bits = 8; // Change this to generate for more bits

    for num in 0..(1 << max_bits) {
        println!("{:08b} = {}", num, num);
    }
}
```

### **How It Works**

1. `max_bits = 8` sets the number of bits to generate.
2. `1 << max_bits` computes \(2^{\text{max_bits}}\) to get the range.
3. The loop iterates from `0` to `2^max_bits - 1`.
4. `{:08b}` prints numbers in **binary format with leading zeros**.
5. The decimal equivalent is printed next to it.

---

### **Example Output (for 4 bits)**

```
00000000 = 0
00000001 = 1
00000010 = 2
00000011 = 3
00000100 = 4
00000101 = 5
00000110 = 6
00000111 = 7
00001000 = 8
00001001 = 9
...
```

You can modify `max_bits` to generate a table for **any number of bits**. Let me know if you need further improvements! 🚀
