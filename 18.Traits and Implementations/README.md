---
marp: true
---

# Trait

- A trait is a collection of methods defined for an unknown type: `Self`.
- Traits can then be implemented on any data type.


---

# Implementation

- `impl` allows us to implement some functionality/trait for a type
- Using an implementation we can define `const`s and functions

---

# Trait + Impl example

```
struct Square {
    side: u32
}

trait Shape {
    fn get_area(&self) -> u32;
}

impl Shape for Square {
    fn get_area(&self) -> u32 {
        self.side * self.side
    }    
}

fn main() {
    let my_square = Square { side: 4 };
    println!("Area of my_square is {}", my_square.get_area());
}
```
---
# Methods

- Methods are similar to functions
- We declare them with the `fn` keyword
- Unlike functions, methods are defined within the context of a struct (or enum etc)
- In a method, The first `&self` parameter is actually short for `self: &Self`

---

# Exercise

- Add a new shape `Rectangle` with fields `width: u32` and `height: u32`
- impl `Shape` for `Rectangle` with the `get_area` method