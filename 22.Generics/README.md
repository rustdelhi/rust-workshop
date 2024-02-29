---
marp: true
---

# Generics

- Generics allow us to create re-usable code that can work with many types
- When defining a function, struct, or enum with generics, you specify type parameters within angle brackets <T>
- Rust’s generics are often used with traits to define behavior that generic types must have
- It reduces code duplication and potential for errors, while still being type safe.

---

# Generics Example

```
struct Rectangle<T> {
    width: T,
    height: T
}

fn main(){
    let rec1:Rectangle<u32> = Rectangle { width: 1u32, height: 2u32};
    println!("rec1 width: {} height: {}", rec1.width, rec1.height);
    
    let rec2:Rectangle<f32> = Rectangle { width: 3.3f32, height: 4.4f32};
    println!("rec2 width: {} height: {}", rec2.width, rec2.height);
}
```

---

# Exercise

- Create a generic 2D point struct called Point<T>. implement `display_point` method for the struct.