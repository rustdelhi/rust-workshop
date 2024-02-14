---
marp: true
---

# Strings Introduction

- A string is a sequence of Unicode scalar values encoded as a stream of UTF-8 bytes
- string Example: `"Hello World"`
- Strings are not null-terminated and can contain null bytes
- Rust's string handling system is different from other languages like C/C++
- Rust strings are of two types: `&str` and `String`

---

# string slice example

```
fn main() {
    let my_str: &str = "Hello World";
    println!("String slice is {}", my_str);
}
```
- Denoted by `&str`
- This string is statically allocated
- It's saved inside the executable
- `my_str` is a string slice (i.e a reference + length)

---

# String example

- A `String` is an in-memory (heap) string.
- `String` can grow, and is also guaranteed to be UTF-8

```
fn main() {
    let my_str:String = String::from("Hello World");
    println!("{}", my_str);
}
```

---

# Appending to a String with push_str

```
fn main() {
    let mut my_str:String = String::from("Hello World");
    println!("{}", my_str);
    my_str.push_str(" 2");
    println!("{}", my_str);
}
```

- Note: push_str accepts a string slice as input

---

# Appending using + operator

```
fn main() {
    println!("{}", String::from("Hello World") + " 2");
}
```

---

# Substring slice

```
fn main() {
    let s = String::from("Hello, world!");
    let substring = &s[0..5];
    println!("{}", substring);
}
```

---

# Handling UTF boundaries

- Note:  UTF-8 characters can vary in length from 1 to 4 bytes

```
fn main() {
    let v = String::from("🗻🗻");
    println!("{:?}", v.get(1..=3));
    println!("{:?}", v.get(0..=3));
}
```

- Note: `char_indices` function also returns an iterator over the utf characters in the string

---

# Looping over UTF chars in a String

```
fn main() {
    let s = String::from("🗻🗻");
    for (_, c_val) in s.char_indices() {
        print!("{}", c_val)
    }
}
```