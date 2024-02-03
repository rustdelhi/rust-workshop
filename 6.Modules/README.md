---
marp: true
---

# Package > Crates > Modules

- Package contains a Cargo.toml
- A package can have multiple crates
- Crates can be a binary or a library
- A package can contain as many binary crates, but at most only one library crate
- Cargo follows these conventions
	- `src/main.rs` is considered the root of bin crate
	- `src/lib.rs` is considered the root of lib crate
- Root crates name is same as the package name
---

# Multiple Binaries from 1 Package

- A package can have multiple binary crates by placing files in the `src/bin` directory. 
- Each file will be a separate binary crate.

---

# Modules

- The compiler starts with the crate root (ex: `src/main.rs`)
- It looks for module declerations. Ex:
	```
	mod shop;
	```
- The compiler then searches for these modules in the following paths:
	- Inline, within the same file
	- In the file `src/shop.rs`
	- In the file `src/shop/mod.rs`