# About
A very humble attempt to make a [Touhou](https://en.wikipedia.org/wiki/Touhou_Project) inspired game using Rust and the [macroquad](https://macroquad.rs/) engine.

## Features
At the moment, the "game" features the typical things you'd see in a schmup. A player that has bombs (one given as of now, no way to get more, gameplay is too short), life, points, level and power (invisible value that increases level). 

It's got enemies that fire different patterns, collision between bullets, enemies, and the player. Items drop from enemies. More item types are planned but as of now, point items drop from all enemies that aren't bosses and raise point value based on player y value. 

A total of 2 power items spawn after the boss, the one on the left functions as an F (full power). It's there purely because there'd be no other way to show the levels and them affecting player fire.

The bomb destroys bullets but does not damage enemies at all yet. I'd probably implement that when there's actually a consistent way to get bombs (I'd need to add more to the sequence).

Indicators are shown on the left that display player info (levels, bombs, lives). Also a horribly placed points indicator at the top (forgot to move it). 

There are 2 patterns as of now, a fan and a circle pattern. First 2 enemies make circles, the fast one after those does a fan, and the boss makes a whopping 1 fan attack and then moves lol.


## Dev Process
Most of the dev for this game was handling object oriented programming stuff, which would help in the long run, but "long run" only applies if I put more time into the game. Most of what's left is assets (it looks horribly ugly), more stuff going on in the sequence and more patterns. 

The project isn't much, but learning Rust was a genuinely fun experience. While I have maybe like 0.1% of the knowledge required to be called an actual rustacean, a lot of Rust's cool features showed up in the process. You should use Rust :3

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
Ignore the 4 prints made when the boss appears lol.

# LLM Use
Zero AI generated code in this repo, but an LLM was used to explain some Rust concepts to me, as I have never had real experience with Rust beyond `rustlings` (genuinely amazing exercises). Once I started to get the hang of macroquad, I was able to do things on my own. Also had help with some math, but all of that was done in desmos, not tracked obviously. When I was happy with stuff in desmos, I put the equation in code according to my own knowledge and the extensive tools Rust has.

# License
See [license](./LICENSE)

**Not affiliated with or endorsed by *Team Shanghai Alice* or the *Touhou Project***

# References
- [Macroquad docs](https://macroquad.rs/docs/)
- Sleeping the thread for a constant FPS, https://github.com/not-fl3/macroquad/issues/749
- [Rust docs](https://doc.rust-lang.org/stable/)
- [Touhou tiling](https://imgur.com/a/3EuwQ6t)
- [Desmos graphing](https://www.desmos.com/calculator)
