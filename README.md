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
apt-get install libgtk-3-dev
```

Then clone the repository and build the project using cargo:
```
git clone https://git.tpi.uni-jena.de/srenkhoff/muninn
cd muninn
cargo build --release
```
Cargo will place the executable in `target/release`.
You can then move it to wherever you like.

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
