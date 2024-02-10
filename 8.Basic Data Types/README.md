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
# Bits and Bytes

- A `bit` can be the smallest piece of data in a digital system.
- A `bit` can be 0 or 1, and hence can only encode two different states.
- A `bit` may not be sufficient to represent larger number of states/range.
- N `bits` allow representation of 2^N different states.
- A `byte` has `8 bits` and can represent 2^8 = 256 different states.

---

# Multiple states

| Bit 1 | Bit 2 | State |
|-------|-------|-------|
| 0     | 0     | State1|
| 1     | 0     | State2|
| 0     | 1     | State3|
| 1     | 1     | State4|

---

# Representing Numbers

| Bit 1 | Bit 2 | State  | Number |
|-------|-------|--------| -------|
| 0     | 0     | State1 | 0      |
| 1     | 0     | State2 | 1      |
| 0     | 1     | State3 | 2      |
| 1     | 1     | State4 | 3      |

---

# Representing Negative Numbers

| Bit 1 | Bit 2 | State  | Number |
|-------|-------|--------| -------|
| 0     | 0     | State1 | -1     |
| 1     | 0     | State2 | 0      |
| 0     | 1     | State3 | 1      |
| 1     | 1     | State4 | 2      |

---
# Multiple bits

| No. of bits (N) | No. of unique states (2^N) |
|----------|----------|
| 1 | 2 |
| 2 | 4 |
| 4 | 16 |
| 8 | 256 |
| 16 | 65536 |
| 32 | 4,294,967,296 |
| 64 | 1.844674407×10¹⁹ |


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
- [Methods](https://doc.rust-lang.org/std/primitive.u8.html)

---

# f32 Example

```
fn main() {
	let num1:f32 = 1.1;
	let num2:f32 = 1.2;
	let added_numbers:f32 =num1+num2;
	println!("Value is {}", added_numbers);
	println!("Size of f32 value {} is {} bytes", added_numbers, std::mem::size_of_val(&added_numbers));
}
```
---
# Floats

- Defined in IEEE 754-2008
- A float data type can represent a more extensive range of values compared to Integers
- This type can represent numbers, like `3.5`, `27`, `-113.75`, `0.0078125`, `34359738368`, `0`, `-1`
- Being able to represent this wide range of numbers comes at the cost of precision
- Floats are used represent the closest value instead of the exact value
- Floats should not be used for the purpose of currency calculations
- [Methods](https://doc.rust-lang.org/std/primitive.f32.html)

---

# bool Type

```
fn main() {
	let experiment_result:bool = false;
	println!("Result of experiment is {}", experiment_result);
}
```
- Boolean type (similar to a `bit`) can represent only 2 values : `true` or `false`

---

# char Type

```
fn main() {
	let symbol:char = 'x';
	println!("Character is: {}", symbol);
	println!("Size of char value {} is {} bytes", symbol, std::mem::size_of_val(&symbol));
}
```