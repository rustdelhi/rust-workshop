---
marp: true
---

# Exercise 1

- Build a rust binary package which prints the following pattern for 50 lines :

    ```
    *
    **
    ***
    ****
    *****
    ```

---

# Exercise 2

- Build a rust binary package which prints the maximum and minimum of a u32 array containing 10 numbers.

    ```
    let arr:[u32;10] = [4,8,9,7,6,5,2,3,1,0];
    ```

---

# Exercise 3

- A Program which loops over an array and prints only even numbers

---

# Solution 1

```
fn main() {
    let num_of_lines = 3;
    for i in 1..=num_of_lines {
        for _ in 1..=i {
            print!("*");
        }
        println!("");
    }
}
```

---

# Solution 2


```
fn main() {
    let arr:[u32;10] = [4,8,9,7,6,5,2,3,1,0];
    let mut min = arr[0];
    let mut max = arr[0];
    for ele in arr {
        if ele < min {
            min = ele
        }
        if ele > max {
            max = ele
        }
    }
    println!("Min: {}", min);
    println!("Max: {}", max);
}
```