# Rust Fire

Rust version of [Fabien Sanglard's](https://github.com/fabiensanglard/DoomFirePSX) Doom fire effect.

![Alt text](rustfire.png?raw=true "Doom fire effect")

## Build instructions
------------------

[minifb](https://github.com/emoon/rust_minifb) provides a frame buffer and has a few dependencies:

```
sudo apt install libxkbcommon-dev libwayland-cursor0 libwayland-dev
```

Build and run (tested with Rust 1.73.0 on Debian 12):

```
cargo build
cargo run
```