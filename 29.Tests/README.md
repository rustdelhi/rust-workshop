---
marp: true
---

# Tests in Rust

- We run tests using `cargo test` command
- Test functions are annotated using `#[test]` attribute on top of functions


---

# Example

```rust
fn sum(one: i32, two: i32) -> i32 {
    one + two
}

#[test]
fn test_positive_sum(){
    let res = sum(2,2);
    assert_eq!(res, 4);
}

#[test]
fn test_negative_sum(){
    let res = sum(-2,-2);
    assert_eq!(res, -4);
}
```

- `assert_eq!(param1, param2)` helps check if param1 and param2 are equal