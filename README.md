# Brainfuck
Rust implementation of [Brainfuck](https://en.wikipedia.org/wiki/Brainfuck)

## Command args

### --data

Set the size of the data array.  This defaults to 30,000 bytes.

### --help

Prints the help message:


```sh
Rust implementation of Brainfuck

Usage: brainfuck [OPTIONS] [file]

Arguments:
  [file]

Options:
  -d, --data <data>  [default: 10]
  -h, --help         Print help
  -V, --version      Print version
```

# Example
````sh
cargo run -- -d 20 examples/hello_world.txt
````
