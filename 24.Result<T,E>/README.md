---
marp: true
---

# Result<T,E>

- Result<T,E> is an enum defines as:
    ```
    pub enum Result<T, E> {
        Ok(T),
        Err(E),
    }
    ```
- It is used to represent Success and Failure conditions

---

# Example

```
fn api_call(payload: &str) -> Result<u32, String> {
    match payload {
        "correct_payload" => Ok(1),
        _ => Err(String::from("Incorrect Payload"))
    }
}

fn main(){
    println!("Attemp1: {:?}", api_call("correct_payload"));
    println!("Attemp1: {:?}", api_call("incorrect_payload"));
}
```

---

# General Errors

- Rust standard library provides `ErrorKind` enum which can be used for general I/O errors.
- Examples:
    ```
    NotFound,
    Unsupported,
    OutOfMemory,
    FileTooLarge,
    InvalidInput,
    InvalidData,
    TimedOut
    .
    .
    etc
    ```
- Full list here: https://doc.rust-lang.org/std/io/enum.ErrorKind.html

---

# Refactoring previous example

```
use std::io::ErrorKind;

fn api_call(payload: &str) -> Result<u32, ErrorKind> {
    match payload {
        "correct_payload" => Ok(1),
        _ => Err(ErrorKind::InvalidData)
    }
}

fn main(){
    println!("Attemp1: {:?}", api_call("correct_payload"));
    println!("Attemp1: {:?}", api_call("incorrect_payload"));
}
```

---

# unwrap()

- Both `Option<T>` and `Result<T,E>` enums implement/have the `unwrap()` function
- It returns the value `T` or panics otherwise

    ```
    use std::io::ErrorKind;

    fn api_call(payload: &str) -> Result<u32, ErrorKind> {
        match payload {
            "correct_payload" => Ok(100),
            _ => Err(ErrorKind::InvalidData)
        }
    }

    fn main(){
        let result = api_call("correct_payload").unwrap();
        println!("Result: {:?}", result);
    }
    ```