---
marp: true
---

# Match

- `match` enables comparing a value against a series of patterns and then execute code based on which pattern matches.
- Example
    ```
    fn main() {
        let number = 2;
        
        match number {
            1 => println!("Value is 1"),
            2 => println!("Value is 2"),
            3 => println!("Value is 3"),
            _ => println!("Value is something else"),
        }
        
    }
    ```

---

- A pattern consists of some combination of the following:
    - Variables
    - Literals
    - Destructured arrays, enums, structs, or tuples
    - Wildcard / Placeholders (Ex: `_`)
- In `if`, the condition needs to evaluate to a Boolean value, but here it can be of Any type
-  The patterns in the arms of a `match` must cover all possibilities.
- The `_` placeholder catches all values in `match`

---

# `if let`

- In some cases the exhaustive checking of a match is not desired
And only a specific pattern needs to be matched, rest of them are to be ignored/handled together.
    ```
    fn main() {
        let tup = (1,2,3);
        if let (1,2,c) = tup {
            println!("c={}", c);
        } else {
            println!("Ignored values")
        }
    }
    ```
- Note: `c` variable is bound to the third value in the tuple
- Other patterns are simply ignored, and handled by else
- `if let` is more readable as there is no indentation