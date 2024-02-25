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