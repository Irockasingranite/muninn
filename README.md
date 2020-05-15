# Installation

# Mac OSX
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
cargo run
```
and it should plot the data from `test.dat`.
