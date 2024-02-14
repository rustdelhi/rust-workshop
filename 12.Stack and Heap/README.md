---
marp: true
---

# Stack

- Data structure that works with LIFO (Last-In-First-Out)

- Example of Stack in real life: 
    ![height:300px](stack.jpg)

---

# Stack and Data

- Data stored on stack must have a known and fixed size at compile time
- This includes 
    - Most primitive types (`i32`, `f64`, `char`, and `bool` etc)
    - Fixed size arrays
    - Tuples containing types that also have a known size at compile time
- Functions and their local variables also utilize stack space
- Stack's nature allows for deterministic, automatic memory management
- Rust can clean up memory without the need for a garbage collector

---

# Heap

- Heap allows storage of data whose size is unknown or can vary
- Heap can be unlimited in size (upto hardware limits)
- Note: We are not talking about the heap data structure
- Rust stack-allocates by default

---

# Box type

- In Rust, we allocate memory on heap by using the `Box` type

    ```
    fn main() {
        let x = Box::new(5);
        println!("{}", *x);
    }
    ```
- The Box<T> type implements the `Drop` trait
- The implementation of `Drop` for `Box<T>` deallocates the memory that was allocated when it was created.