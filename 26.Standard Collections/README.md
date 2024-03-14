---
marp: true
---

# Standard collection

- Contains bunch of useful types that usually used in Programs
- The include:
    - Vec<T>
    - String
    - HashMap<K,V>
    - HashSet<T>
    - etc

---

# Vec<T>

- A dynamic array that can grow or shrink in size. 
- It is the most commonly used collection for storing a list of items of the same type. 
- Vectors are useful when you need to store an unknown number of items
- Example:
    ```
    let v: Vec<i32> = Vec::new();
    ```

---

# HashMap<K,V>

 - A hash table implementation that maps keys of type K to values of type V. 
 - It provides fast lookup, insertion, and removal of key-value pairs based on the hash of the key. 
 - It's useful for storing and accessing data through unique keys.

 ---

 # HashMap Example

 ```rust
 use std::collections::HashMap;

fn main(){
    let mut capitals:HashMap<&str,&str> = HashMap::new();
    capitals.insert("India", "Delhi");
    capitals.insert("Italy", "Rome");
    
    println!("Capital of India is {}", capitals.get("India").unwrap());
    println!("Capital of Italy is {}", capitals.get("Italy").unwrap());
}
```

---

# Exercise

- Create a HashMap of Rainbow colors with keys being the first letter of the color.