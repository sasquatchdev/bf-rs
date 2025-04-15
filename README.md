# sasquatchdev/bf-rs
A brainf*ck compiler written in rust.

## Table of Contents

- [Introduction](#introduction)
- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)

Additional Brainf*ck resources:

- [Spec](https://esolangs.org/wiki/Brainfuck)
- [Implemented Syntax](SYNTAX.md)

## Introduction

This project features a cli tool for interpreting and compiling an
input file in the ["brainfuck"](https://en.wikipedia.org/wiki/Brainfuck) (esoteric) programming language.

## Features

- Live-Interpretation in Rust
- Compilation to Python
- Compilation to Assembly (AT&T syntax)
- Pre-Compiled `hello_world.bf` file (`.py` and `.asm`) in `./res`
- _(planned)_ Compilation to javascript / typescript

## Installation

To install this, make sure you have [Rust](https://www.rust-lang.org/) and [Git](https://git-scm.com/downloads) installed, and clone this repository.

```bash
git clone https://github.com/sasquatchdev/bf-rs.git && cd ./bf-rs
```

## Usage

To use [bf-rs](#), make sure you followed the previous steps. Then, run

```bash
cargo run --quiet -- -c input.bf output.asm assembly # (for compilation to assembly)
```

or

```bash
cargo run --quiet -- -c input.bf output.py python # (for compilation to python)
```

or

```bash
cargo run --quiet -- -i input.bf # (for interpretation)
```

Alternatively, you can also choose to compile it first. To do so, run

```bash
cargo build --release --quiet && cd ./target/release
```

Next you can use the syntax specified above (`-i` or `-c`) to use [bf-rs](#).

```bash
./bf-rs -i input.bf # (for interpretation)
```
