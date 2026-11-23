## Installing
If, for some reason, you want to try this "game", the release contains binary for a windows `.exe` and Linux binary. I apologize, I main Linux and have not checked if the `.exe` works (for the latest release, I have tried it on an earlier version, so I am fairly sure it'd work). I also do not know anything about binaries on MacOS. 

### Building From Source
If you really want to build from source, you'll want to make sure you have the Rust toolchain installed (you can find it [here](https://rustup.rs/)). 
Next, clone this repo, navigate to its root and then run `cargo build --release`. A `target/release` will be created with the binary that you can then run.
> [!NOTE]
> Plain `rustc` will not work lol

## Gameplay
Super simple, not much going on. Shares keybinds with Touhou,
  - Z, shoot
  - X, bomb (red indicator on right)
  - Arrow keys, movement
  - Left shift, slow

# LLM Use
Zero AI generated code in this repo, but an LLM was used to explain some Rust concepts to me, as I have never had real experience with Rust beyond `rustlings` (genuinely amazing exercises). Once I started to get the hang of macroquad, I was able to do things on my own. Also had help with some math, but all of that was done in desmos, not tracked obviously. When I was happy with stuff in desmos, I put the equation in code according to my own knowledge and the extensive tools Rust has.

# License
This project is covered under (wait not it's not hold on)

# References
- [Macroquad docs](https://macroquad.rs/docs/)
- Sleeping the thread for a constant FPS, https://github.com/not-fl3/macroquad/issues/749
- [Rust docs](https://doc.rust-lang.org/stable/)
- [Touhou tiling](https://imgur.com/a/3EuwQ6t)
- [Desmos graphing](https://www.desmos.com/calculator)
