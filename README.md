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

Then clone the repository and build the project using cargo:
```
git clone https://git.tpi.uni-jena.de/srenkhoff/muninn
cd muninn
cargo build --release
```
Cargo will place the executable in `target/release`.

To add muninn to your `$PATH` for easy access, either manually move or symlink it to e.g. `/usr/local/bin`, or use
```
cargo install --path .
```
to have cargo take care of it (by default it places the executable in `~/.cargo/bin`, which rustup added to your `$PATH`).

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
and compile Muninn with
```
cd ~/muninn
cargo build --release
```
This should create an executable in target/release.
