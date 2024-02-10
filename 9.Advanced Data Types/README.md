---
marp: true
---

# Compound Types

- Compound types can group multiple values into one type
-  Rust has two primitive compound types: 
    - tuples
    - arrays

---

# Tuples

```
fn main() {
    let tup: (i32, f64, u8) = (-1, 1.2, 1);
}
```

---

# Accessing elements of a Tuple

```
fn main() {
    let tup: (i32, f64, u8) = (-1, 1.2, 1);

    let neg_one = tup.0;
    let one = tup.2;
}
```
---

# Destructuring

```
fn main() {
    let tup = (-1, 1.2, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");
}
```