---
marp: true
---

- Scopes allow us to group executable statements
- demarcated by using curly braces `{` `}`
- variables are destroyed when a scope ends. Example:

    ```
    fn test(){
        let a = 4;
        println!("{}", a);
    }

    fn main() {
        // println!("{}", a);
        test();
    }
    ```

---

# Scope within a function ?

```
fn test(){
    {
        let b = 5;
        println!("{}", b);
    }
    println!("{}", b); //b is not found in this scope
}

fn main() {
    test();
}
```

---

# Shadowing

- Shadowing allows a variable to be re-declared

```
fn test(){
    let x = 1;
    println!("{}", x);
    {
        let x = 2*x;
        println!("{}", x);
    }
    println!("{}", x);
}

fn main() {
    test();
}
```