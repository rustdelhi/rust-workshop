---
marp: true
---

# Struct

- Structs allow us to group data

```
struct User {
   active: bool,
   username: String,
   email: String,
   sign_in_count: u64,
}
```

- `struct` allows us to group together `fields`

---

# Struct Instance

```
let mut user1 = User {
   active: true,
   username: String::from("someusername123"),
   email: String::from("someone@example.com"),
   sign_in_count: 1,
};
user1.sign_in_count = 2;
```
- A `struct` instance is created using `{` `}` after the name of the `struct`
- To access a `struct` `field` we use the `dot` notation

---

# Creating instance from another instance

```
fn main() {
    // --snip--

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
}
```

- Note the `..` operator allow `user2`'s fields to be instantiated from `user1`

---

# Tuple structs

- Rust also supports structs that look similar to tuples, called tuple structs
- tuple structs are just like tuples but have a name associated with it to differentiate them from other tuples

```
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
```

---

# Unit-Like Structs Without Any Fields

```
struct User;
```

- This type of struct has no fields
- Used to implement a trait on your type, which does not have any data

---