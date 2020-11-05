# SiO<sub>2</sub> #

![demo](img/demo.gif)

A simple powder toy clone written in rust with [bevy](https://bevyengine.org/).

## Controls ##

- `1` - static concrete
- `2` - sand
- `3` - water
- `0` - eraser
- `+` - increase brush size
- `-` - decrease brush size

## Building ##

Install and configure nightly rust:

```
rustup toolchain install nightly
cd path/to/sio2
rustup override set nightly
```

Install LLD/ZLD:

```
# Ubuntu:
sudo apt-get install lld

# Windows:
cargo install -f cargo-binutils
rustup component add llvm-tools-preview

# Mac OS:
brew install michaeleisel/zld/zld
```

Build and run SiO2:

```
cd path/to/sio2
cargo run --release
```