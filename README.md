## Installing
If, for some reason, you want to try this "game", the release contains binary for a windows `.exe` and Linux binary. I apologize, I main Linux and have not checked if the `.exe` works (for the latest release, I have tried it on an earlier version, so I am fairly sure it'd work). I also do not know anything about binaries on MacOS. 

### Building From Source
If you really want to build from source, you'll want to make sure you have the Rust toolchain installed (you can find it [here](https://rustup.rs/)). 
Next, clone this repo, navigate to its root and then run `cargo build --release`. A `target/release` will be created with the binary that you can then run.
> [!NOTE]
> Plain `rustc` will not work lol


# References
- [Macroquad docs](https://macroquad.rs/docs/)
- Sleeping the thread for a constant FPS, https://github.com/not-fl3/macroquad/issues/749
- [Rust docs](https://doc.rust-lang.org/stable/)
- [Touhou tiling](https://imgur.com/a/3EuwQ6t)
- [Desmos graphing](https://www.desmos.com/calculator)
