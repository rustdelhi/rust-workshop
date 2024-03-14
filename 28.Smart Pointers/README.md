---
marp: true
---

# Pointers Vs Smart Pointers

- A **pointer** in general is a variable that stores a memory address
- **Smart pointers** are data structures that act like a pointer but also have additional metadata and capabilities.
- The concept of smart pointers isn’t unique to Rust: smart pointers originated in C++ and exist in other languages as well.

---

# Box<T>

- Written as `Box<T>`
- Boxes allow us to store data on the heap rather than the stack
- The pointer to the heap data remains on the stack but data is in the heap
- Boxes don’t have a performance overhead
- Used mostly in these situations:
    - When you have a type whose size can’t be known at compile time
    - When you have a large amount of data and you want to transfer ownership but ensure the data won’t be copied when you do so
    - When you want to own a value and you care only that it’s a type that implements a particular trait rather than being of a specific type
- Box<T> implements the `Deref` and `Drop` trait

---

# Box<T> Example 1

- The following code moves a value from the stack to the heap by creating a Box:

    ```rust
    fn main() {
        let val: u8 = 5;
        let boxed: Box<u8> = Box::new(val);
    }
    ```

---

# Box<T> Example 2

- The following code moves a value from the heap to the stack by dereferencing

    ```rust
    fn main() {
        let boxed: Box<u8> = Box::new(5);
        let val: u8 = *boxed;
    }
    ```

---

# Box<T> Example 3

- The following code shows how to create a Box with a custom struct

    ```rust
    struct Rectangle {
        width: u32,
        height: u32
    }

    fn main(){
        let r : Box<Rectangle> = Box::<Rectangle>::new(Rectangle { width: 1, height: 4 });
        println!("Rect Width: {}", r.width);
        println!("Rect Height: {}", r.height);
    }
    ```
---

# Reference Counting using Rc<T>

- Reference counting pointer enables us to 
    - create data which can have multiple owners 
    - keep track of the number of owners
    - cleaning up the data, when no owners remains
- Does not work in multi-threaded environment
---

# Rc<T> Example

```rust
use std::rc::Rc;

fn main(){
    let num: u32 = 5;
    
    let ref1 = Rc::new(num);
    println!("Ref count : {}", Rc::strong_count(&ref1));

    {
        let ref2 = Rc::clone(&ref1);
        println!("Ref count : {}", Rc::strong_count(&ref2));
    }
    
    println!("Ref count : {}", Rc::strong_count(&ref1));
}
```

---

# Refcell<T>

- A reference whose borrows are checked at runtime (instead of compile time)

```rust
use std::cell::RefCell;

fn main(){
    
    let c = RefCell::new(5);
    
    let borrowed1 = c.borrow_mut();
    println!("Value is {}", borrowed1);
    
    let borrow2 = c.borrow_mut(); //<== This will panic at runtime

}
```

---

# Thread Safety

- `Rc<T>` is not thread-safe, i.e it won't work with multiple threads
- To solve this we have other types namely :
    - `Arc<T>`
    - `Mutex<T>`
    - `RwLock<T>`

---

# Example of `Arc<T>`

```rust
use std::sync::Arc;
use std::thread;

fn task(arc_data: Arc<u32>) {
    println!("Ref count in task thread : {}", Arc::strong_count(&arc_data));
    
    let arc_data2 = Arc::clone(&arc_data);
    println!("Ref count in task thread : {}", Arc::strong_count(&arc_data2));
    
    let arc_data3 = Arc::clone(&arc_data);
    println!("Ref count in task thread : {}", Arc::strong_count(&arc_data3));

}

fn main(){
    let num: u32 = 5;
    
    let arc_data = Arc::new(num);
    println!("Ref count in main thread : {}", Arc::strong_count(&arc_data));

    let t1 = thread::spawn(|| task(arc_data));
    let _ = t1.join();
}
```