---
marp: true
---

# Functions

- Functions are defined using `fn` keyword
- Example :
    ```
    fn main() {
        println!("Hello, world!");
    }
    ```
- In Rust we use snake case for function and variable names
    ```
    fn my_function(){
        let my_var = 2;
        println!("{}", my_var);
    }
    ```
- Rust doesn’t care where you define your functions, 
    only that they’re defined somewhere in the same scope
---

# Function signature, parameters and arguments
```
fn is_smaller(a: u32, b:u32) -> bool {
    if a < b { return true; } else { return false; }
}

fn main() {
    println!("{}", is_smaller(1,2));
}
```
- Here `fn is_smaller(a: u32, b:u32) -> bool` is the function signature
- `a` and `b` are the `parameters` of the function
- In function signatures, you must declare the type of each parameter. 
- The values passed to the `parameters` are called the `arguments`
- Here `bool` is the return type. Notice the use of `->`.
---

# Function body

- The body of a function is composed of `statements` and an optional `expression` at the end.
- `Statements` are instructions that perform some action and do not return a value.
- `Expressions` evaluate to a resultant value.
    ```
    fn main() {
        let y = {
            let x = 3;
            x + 1
        };

        println!("The value of y is: {y}");
    }
    ```