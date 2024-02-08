---
marp: true
---

- A library consists of functions or modules that can be re-used in another crate.
- A library created in rust has the `.rlib` file extension.
- Libraries are not executables

---

# Creating a library

- To create a project which builds a library, use the following command:
    ```
    mkdir sample
    cd sample
    cargo init --lib
    ```
- Notice that `src/lib.rs` is created, as a Cargo convention.

---

# Using a library

- To use an external library using cargo you can add it to the Cargo.toml
    ```
    [dependencies]
    sample = { path = "../sample" }
    ```
- Refer to an external crate
    ```
    extern crate sample;
    
    fn main() {
        let res = sample::add(1,2);
        println!("Sum: {}", res);
    }
    ```