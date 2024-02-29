---
marp: true
---

# Lifetime

```
struct Person{
   name: String
}
struct Company{
   name: String,
   ceo: &Person
}
fn main() {
   let boss = Person { name: String::from("Elon") };
   let tesla = Company { name: String::from("Tesla"), ceo: &boss };
}
```
---

# Fixing with lifetime

```
struct Person{
   name: String
}
struct Company<'c_lifetime>{
   name: String,
   ceo: &'c_lifetime Person
}
fn main() {
   let boss = Person { name: String::from("Elon") };
   let tesla = Company { name: String::from("Tesla"), ceo: &boss };
}
```

- The lifetime of `Company` and `ceo` are now the same
- A `Company` can no longer point to ceo that goes out of scope

---

# 'static lifetime

- A reference can be kept alive throughout the lifetime of the program by using `'static`

   ```
   fn main() {
      let s: &'static str = "hello world";
      println!("{}", s);
   }
   ```

- By default a string literal has a 'static lifetime. But String does not.

---

# Dangling Reference example

The following code tries to create a dangling reference :

```
struct Person<'a> {
    name: &'a String
}

fn main() {
    let mut s = Person { name: &String::from("First") };
    println!("{}", s.name);
    {
        let john = String::from("John");
        s.name = &john;
    }
    println!("{}", s.name);
}
```

- Output: `error[E0597]: john does not live long enough`

---