# What is this?
A small Rust program that plays a "Mary Stuart" card patience and
counts the success rate.

# What is this "Mary Stuart" patience?
Ah, you just lay out a 36-card deck in a row and start
going from left ro right doing the following:
if a card is surrounded with two other cards
and those two cards have the same suit or value,
you pick up the central card and put it on top of the left one.

The goal is to have only two stacks of cards left.

# Running this
## Prerequisites:
0. Git
1. Rust (with cargo and a nightly toolchain installed)

## Running
0. Clone the repo
1.
```shell
cargo +nightly run --release
```
