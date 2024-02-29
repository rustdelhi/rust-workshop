---
marp: true
---

# Option<T>

- Option<T> is a commonly used enum in Rust
- Defined as 

    ```
    pub enum Option<T> {
        None,
        Some(T),
    }
    ```

---

# How to use Option<T>

```
fn api_call_success() -> Option<u32> {
    Some(5)
}

fn api_call_fail() -> Option<u32> {
    None
}

fn check(result: Option<u32>) {
    match result {
        Some(val) => println!("Result was success, val is {}", val),
        None => println!("Result is None")
    }
}
fn main(){
    check(api_call_success());
    check(api_call_fail());
}
```

---

# Exercise

- Implement the `Eatable` trait for `FoodBasket`

    ```
    struct FoodBasket {
        food_items_remaining: Option<u32>
    }

    trait Eatable {
        fn display_status(&self); 
        fn eat_all(&mut self);
    }
    ```

- Note: display "No food remaining" if basket is empty, else count of food items.
- eat_all clears ALL items in the food basket