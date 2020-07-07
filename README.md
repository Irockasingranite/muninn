# Installation

## Linux
To compile muninn yourself, install [rustup](rustup.rs).
You may also need to install gtk3 using your favourite package manager, e.g.:

Arch Linux:
```
pacman -S gtk3
```
Ubuntu:
```
apt install libgtk-3-dev
```

Then clone the repository and build + install the project using cargo:
```
git clone https://git.tpi.uni-jena.de/srenkhoff/muninn
cd muninn
cargo install --path .
```

This will place the executable in `~/.cargo/bin`, which rustup should have added to your `$PATH`.

For a manual install in a location of your choice, you can run
```
cargo build --release
```
instead.
Cargo will place the executable in `target/release`.
You can move or symlink it to wherever you like, e.g. `/usr/local/bin`.

## Mac OSX
(*Tested on OSX 10.15.4*)

Firstly, get [Homebrew](https://brew.sh), *the missing package manager for macOS*.

Install Rust via Homebrew by
```
brew install rust
```
Install gtk+3 via Homebrew by
```
brew install gtk+3
```

Then clone the repository by
```
git clone https://git.tpi.uni-jena.de/srenkhoff/muninn ~/muninn
```
and compile and install Muninn with
```
cd ~/muninn
cargo install --path .
```
This should create an executable in `~/.cargo/bin`.
