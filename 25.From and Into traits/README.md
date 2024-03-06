---
marp: true
---

# From<T>

- Definition
    ```
    pub trait From<T>: Sized {
        // Required method
        fn from(value: T) -> Self;
    }
    ```
- Allows us to convert a value from another type T

---

# Example

```
#[derive(Debug)]
enum TechCompanies {
    Apple,
    Google
}

#[derive(Debug)]
struct Company {
    name: TechCompanies
}

impl From<String> for Company {
    fn from(str_val: String) -> Self {
        if str_val == "Apple" || str_val == "apple" {
            return Company { name: TechCompanies::Apple }
        } else if str_val == "Google" || str_val == "google"  {
            return Company { name: TechCompanies::Google }
        } else {
            panic!("Invalid value, can't parse to Company")
        }
    }
}


fn main(){
    let s = String::from("google");
    let c = Company::from(s);
    println!("{:?}",c.name);
}
```

---

# Into

- `Into` is the reverse of `From`
- Definition:
    ```
    pub trait Into<T>: Sized {
        // Required method
        fn into(self) -> T;
    }
    ```

---

# Example

```
#[derive(Debug)]
enum TechCompanies {
    Apple,
    Google
}

#[derive(Debug)]
struct Company {
    name: TechCompanies
}

impl From<String> for Company {
    fn from(str_val: String) -> Self {
        if str_val == "Apple" || str_val == "apple" {
            return Company { name: TechCompanies::Apple }
        } else if str_val == "Google" || str_val == "google"  {
            return Company { name: TechCompanies::Google }
        } else {
            panic!("Invalid value, can't parse to Company")
        }
    }
}


fn main(){
    let s = String::from("google");
    let c: Company = s.into();
    println!("{:?}",c.name);
}
```

---

# Exercise

- Implement From<T> for the following struct where T is a tuple of 3 numbers:
    ```
    struct Pixel {
        r: u8,
        g: u8,
        b: u8
    }
    ```