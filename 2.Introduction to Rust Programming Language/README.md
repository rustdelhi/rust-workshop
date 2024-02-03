---
marp: true
---

# So, What is Rust ?
- A Programming language in this context.
- We are not talking about the compound studied in Chemistry ( Fe₂O₃ ).

---

# What can Rust produce ?
- Rust compiler produces Native Binaries for your Computer/target hardware.
- A virtual machine is not required to run your executable/app.

---

# What’s Different about rust ?

- It enforces memory-safety.
- That is, you are unlikely to encounter Buffer overflows, Dangling pointers, Seg Faults, Memory leaks etc.
- Hence, Rust allows devs to write SAFE code.
- Safe code is Important to prevent Production bugs. It's always better to catch bugs during development instead after a Production deployment.
- Rust helps you create Reliable software

---

# Origin of Rust

- Rust started as Mozilla engineer Graydon Hoare’s personal project in 2006. Almost ~18 years ago.
- Rust is named after a [fungus](https://www.reddit.com/r/rust/comments/27jvdt/internet_archaeology_the_definitive_endall_source/) that is robust, distributed, and parallel.
- The following is quoted from an interview with [Graydon](https://www.infoq.com/news/2012/08/Interview-Rust/)
    ```
    A lot of obvious good ideas, known and loved in other languages, 
    haven’t made it into widely-used systems languages, or are deployed in languages 
    that have very poor 
    (unsafe, concurrency-hostile) memory models. 
    There were a lot of good competitors in the late 70s and early 80s in that space, 
    and I wanted to revive some of their ideas and give them another go, 
    on the theory that circumstances have changed: 
    the internet is highly concurrent 
    and highly security-conscious, 
    so the design-tradeoffs that always favor C and C++ (for example) have been shifting.
    ```

---

# What can you build with Rust ?

- Command Line Interfaces
- Network Services 
    (Ex:HTTP(s) Services, WebSocket APIs etc)
- Apps for Embedded Systems 
    (Ex:for IoT Devices)
- Target WebAssembly 
    (Ex: apps in browsers, but requires JS)

---

- Desktop Applications (Ex: using Tauri)
- Operating Systems (Ex: see Redox)
- Game Engines, VR Experiences
- Web3 Apps
- Web4, Web5 ………, Web99 apps ? (not sure)
- Probably a lot more !

---

# Contemporary Developer Tools

Rust ecosystem showers you with modern tools and make your life easier, Examples:

- Cargo: Dependency Manager and build tool
- Rustfmt: Formatting tool ensures consistent coding style
- Rust Language Server : IDE Integrations for code completion, inline error messages

---

# Projects using Rust

- [Servo](https://github.com/servo/servo) : Browser Engine by Mozilla Research
- [Android](https://security.googleblog.com/2021/04/rust-in-android-platform.html) and [Windows](https://www.theregister.com/2023/04/27/microsoft_windows_rust/) both have some parts written in Rust
- [Bottlerocket](https://aws.amazon.com/bottlerocket/) : An operating system designed for hosting containers, written in Rust, built by AWS
- [Redox](https://www.redox-os.org/) : a Unix-like Operating System written in Rust
- [Discord](https://discord.com/blog/why-discord-is-switching-from-go-to-rust) : Rust has helped optimized discord backend
- [Ulvetanna](https://www.ulvetanna.io/) : FPGA clusters tailor-made for Zero Knowledge Proof computation at scale, to support L1 and L2 blockchain protocols
- [And many more ...](https://github.com/search?q=language%3ARust&type=repositories&s=stars&o=desc)