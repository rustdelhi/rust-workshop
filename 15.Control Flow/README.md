---
marp: true
---

# Control Flow

## If Expression

```
fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}
```
- All if expressions start with the keyword `if`, followed by a `condition` which resolves to a `bool` value

---

# Handling Multiple Conditions with else if

```
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
```

- Use `else if` to define multiple arms

---

# Loops

- Loops helps us repeat the execution of statements in a block
- There are 3 kinds of loops
    - `loop`
    - `while`
    - `for`

---

# `loop` keyword

- The loop keyword tells Rust to execute a block of code over and over again forever or until you explicitly tell it to stop.

```
fn main() {
    loop {
        println!("again!");
    }
}
```
---

# Breaking out of a loop

- To break out of a loop use `break` keyword
```
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}
```
- Notice that `break` also allows returning values

---

# While loop

- A program will often need to evaluate a condition within a loop
- While the condition is true, the loop runs

```
fn main() {
    let mut number = 5;

    while number != 0 {
        println!("{number}");
        number -= 1;
    }
}
```

---

# `for in` loop

- `for in` loop operates over any collection implementing the `IntoIterator` trait.
```
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}
```

---

# Ranges

- Example : 1 to 10 (exclusive)
```
fn main() {
    for i in 1..10 {
        println!("{}", i);
    }
}
```
- Example : 1 to 10 (inclusive)
```
fn main() {
    for i in 1..=10 {
        println!("{}", i);
    }
}
```