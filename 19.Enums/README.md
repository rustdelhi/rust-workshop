---
marp: true
---

# `enum` keyword

- `enum` stands for `enumeration`
- Enums allow you to define a type by enumerating its possible variants

---

# `enum` example

```
#[derive(Debug)]
enum Pixel {
    Red,
    Green,
    Blue
}

fn main(){
    let first_pixel = Pixel::Red;
    let second_pixel = Pixel::Green;
    let third_pixel = Pixel::Blue;
    
    println!("First Pixel is {:?}", first_pixel);
    println!("Second Pixel is {:?}", second_pixel);
    println!("Third Pixel is {:?}", third_pixel);
}
```
---

# `#[derive(Debug)]`

```
#[derive(Debug)]
enum Pixel {
    Red,
    Green,
    Blue
}
```
- Here we are implementing the `Debug` trait which helps in displaying the enum values in println. 
- The `derive` attribute (i.e metadata associated with the type) helps us to provide standard implementations for some traits.

---

# `enum` and `match` 

- `match` is designed to ensure all possible patterns are considered in the arms.
- For an `enum` the patterns are all the `variants` of the `enum`

---
# `enum` and `match` example

```
#[derive(Debug)]
enum Pixel {
    Red,
    Green,
    Blue
}

fn main(){
    let test_var = Pixel::Green;
    match test_var {
        Pixel::Red => println!("Red pixel"),
        Pixel::Green => println!("Green pixel"),
        Pixel::Blue => println!("Blue pixel"),
    }
}
```

- If any of these match `arms` are missing, we get the `non-exhaustive patterns` error.

---

# `enum` with data

```
#[derive(Debug)]
enum AppleSmartPhone {
    IPhone11,
    IPhone12,
    IPhone13,
    IPhone14,
    IPhone15
}

#[derive(Debug)]
enum AppleComputer {
    MacMini,
    Macbook,
    MacbookPro,
    MacbookAir
}

#[derive(Debug)]
enum AppleProduct {
    SmartPhone(AppleSmartPhone),
    Computer(AppleComputer)
}

fn main(){
    let purchased = AppleProduct::SmartPhone(AppleSmartPhone::IPhone15);
    println!("I bought a {:?}", purchased);

}
```

---

# `enum` + multiple `data` example 2

```
#[derive(Debug)]
enum ShoeColor {
    Black,
    White,
    Blue
}

#[derive(Debug)]
enum ShoeSize {
    UK6,
    UK7,
    UK8,
    UK9,
    UK10,
    UK11
}

#[derive(Debug)]
enum FootWear {
    Sneaker { color: ShoeColor, size: ShoeSize  },
    SportShoe { color: ShoeColor, size: ShoeSize  },
    Sandal { color: ShoeColor, size: ShoeSize  }
}

fn main(){
    let purchased = FootWear::SportShoe { color: ShoeColor::White, size: ShoeSize::UK10 };
    println!("I bought a {:?}", purchased);
}
```

---
# Exercise

- Prepare a rust program to create an enum `RainbowColors` and demonstrate use of `match` keyword with it.