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

---

# Arrays

- Arrays also allow us to have a collection of values.
- Unlike tuples, array has values of the same type
- Arrays in rust have a fixed length
- If we need the size to grow, we can use a `vector`, covered later
- Example:
    ```
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    ```
---

# Array with same values

```
let a = [0; 5];
```

- This array will contain 5 elements with each of them being `0`.

---

# Accessing Array Elements

```
fn main() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
}
```
---

# Invalid Array Element Access

```
fn main() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let val = a[50];
    println!("The value is: {val}");
}
```

- Error : `index out of bounds: the length is 5 but the index is 50`

---

# Slice

- A `slice` is a reference / view into a sequence with a length associated.
- They are useful for allowing safe, efficient access to a portion of a sequence(ex: an array) without copying.
- Usecase: You might want to reference just one line of a file read into memory.
- Usually, a slice is not created directly, but from an existing variable

---

# slice example

```
fn main() {
    let a = [0, 1, 2];
    let full_slice = &a[..];
    println!("{}", full_slice[0]);
    println!("{}", full_slice[1]);
    println!("{}", full_slice[2]);
}
```

- `full_slice` is a view into the array, extending to the full array

---

# slice example (continued)

```
fn main() {
    let a = [0, 1, 2];
    let part_slice = &a[1..=2];
    println!("{}", part_slice[0]);
    println!("{}", part_slice[1]);
}
```

- `part_slice` is a view into the array, starting from index 1 till index 2