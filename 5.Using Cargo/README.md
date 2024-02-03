---
marp: true
---

# Cargo

- Cargo is the Rust package manager. 
- Cargo helps 
    - download your Rust package’s dependencies
    - compile your source code or libraries into packages called `Crates`
    - upload them to crates.io, the Rust community’s package registry.
- External dependencies are specified in a `Cargo.toml` file
- To create a new package using cargo, use the following:
    ```
    cargo new hello_world --bin
    ```
---
# External Dependencies

- External packages/crates can be found at `https://crates.io`
- Let's add one by using `cargo add emoji` in the `hello_world` directory
- Now we modify `hello_world/src/main.rs` to use the new package
    ```
    println!("{}", emoji::food_and_drink::food_fruit::LEMON.glyph);
    println!("{}", emoji::food_and_drink::food_fruit::WATERMELON.glyph);
    ```
- Build the program using `cargo build`
- Run the program using `cargo run`