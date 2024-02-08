---
marp: true
---


# Data types available in Rust

- Integer Types
- Floating Point Types
- Boolean Types
- Character Type
- Others
    - Arrays, Tuples, Strings, Structs and Enums

---

# Integer types in Rust

- 8 Bits: i8, u8
- 16 Bits: i16, u16
- 32 Bits: i32, u32
- 64 Bits: i64, u64
- 128 Bits: i128, u128
- arch: isize, usize
- The pointer sized types. The size of this primitive is how many bytes it takes to reference any location in memory.

---

# u8 Example

```
fn main() {
	let my_number:u8 = 255;
	println!("Value is {}", my_number);
}
// The values b/w 0 and 255 work here
// The values -1 and 256 won’t work here
```

---
# Other Types

- Floating Point Types
    - f32 and f64
- Boolean Type
    - Bool
- Character Type
    - Char