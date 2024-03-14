---
marp: true
---

# Exercise 1

Build a rust binary package which prints the following pattern for 50 lines :

```
*
**
***
****
*****
```

---

# Exercise 2

Build a rust binary package which prints the maximum and minimum of a u32 array containing 10 numbers.

```rust
let arr:[u32;10] = [4,8,9,7,6,5,2,3,1,0];
```

---

# Exercise 3

Add a new shape "Rectangle" with fields 
- width: u32
- height: u32

impl "Shape" trait for "Rectangle" with the "get_area" method

---

# Exercise 4

Prepare a rust program to create an enum `RainbowColors` and demonstrate use of match keyword with it.

---

# Exercise 5

Create a generic 2D point struct called Point<T>. implement display_point method for the struct.

---

# Exercise 6

Implement the `Eatable` trait for `FoodBasket`

```rust
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

---

# Exercise 7

Implement From<T> for the following struct where T is a tuple of 3 numbers:

```rust
struct Pixel {
    r: u8,
    g: u8,
    b: u8
}
```