[![Build Status](https://travis-ci.org/second-super-secret-squirrel-account/rust_tic_tac_toe.svg?branch=master)](https://travis-ci.org/second-super-secret-squirrel-account/rust_tic_tac_toe)

# README #
* Tic Tac Toe in Rust
* Version "0.1.0"
* Repo owner: Tyler.Decker@gmail.com

## Summary of set up ##
  In order to run this repo you must first have Cargo and the Ncurses Library installed.
  Mac osX users that have Xcode installed should already have the Ncurses Library by default.

---

###  To setup Cargo Rust and the Library for Ncurses use the following commands. ###
* Debian / Ubuntu

  `sudo apt-get install cargo`

  `sudo apt-get install libncurses-dev`

* Mac osX

  `curl -sf -L https://static.rust-lang.org/rustup.sh | sh`

---
### Rust and Cargo Versions that were used for this project ###
rustc 1.12.1 [rust github] (https://github.com/rust-lang/rust)

cargo 0.13.0-nightly [cargo github](https://github.com/rust-lang/cargo)

---

### Clone the project ####
`git clone https://github.com/second-super-secret-squirrel-account/rust_tic_tac_toe.git`

### To Run the Tests ###
`cargo test`

### To Run the Program ###
`cargo run`

---

### Rust Dependencies needed to run this project ###

Cargo will install this dependency for you, it is just listed here as a reference.

  * ncurses = "5.83.0" [ncurses github](https://github.com/second-super-secret-squirrel-account/ncurses-rs)
