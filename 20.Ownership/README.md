---
marp: true
---

# Understanding Ownership

- Programs usually manage memory in the following ways:
    - The programmer must explicitly allocate and free the memory
    - A Garbage Collector regularly looks for no-longer-used memory
- Rust has it's own rules to manage memory:
    - Each value in Rust has an owner
    - There can only be one owner at a time
    - When the owner goes out of scope, the value will be dropped

---

# Borrow checker example 1

```
struct Square { side: u8 }

fn main() {
   let sq = Square { side: 5 };
   let sq2 = sq;
   println!("{}", sq.side);
}
```

---

# Borrow checker example 2

```
struct Square { side: u8 }

fn print_area(sq: Square){
	println!("Area {:?}", sq.side * sq.side);
}

fn main() {
   let sq = Square { side: 5 };
   print_area(sq); //Value moved here
   println!("Side of square is {}", sq.side);
}

```
- Error : `error[E0382]: borrow of moved value: sq`

---

# References

```
struct Square { side: u8 }

fn print_area(sq: &Square){
	println!("Area {:?}", sq.side * sq.side);
}

fn main() {
   let sq = Square { side: 5 };
   print_area(&sq); //Reference given to fn
   println!("Side of square is {}", sq.side);
}
```

---

# Borrowing rules

- The scope of a borrow must be within the scope of the owner
- One or more references (&T) to a resource are allowed
- There can only be exactly one mutable reference (&mut T)
- Modifying a value using a reference is only possible if using a mutable reference.

---

# Ownership exceptions

- All primitive types that implement the Copy trait are not moved like one would assume

---

# Borrow checker example 3

```
fn main(){
    let mut v = vec![1, 2, 3];

    for i in &v {
        println!("{}", i);
        v.push(34);
    }
}
```

- Error: `cannot borrow `v` as mutable because it is also borrowed as immutable`

---

# Borrow checker example 4

```
fn main(){
    let y: &i32;
    
    {
        let x = 5;
        y = &x;
    }
    
    println!("{}", y);

}
```

- Error: `error[E0597]: `x` does not live long enough`