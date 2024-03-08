# rcc

## Running the Compiler

Make sure you have Rust installed.

```bash
cargo run test.c # generates test.s
gcc test.s -o test.out
./test.out
```

## C Language Features

Current features:
- Function definitions (with int return type, no parameters)
- Multiple statements in a function (you have to use a compound statement with curly braces)
- Return statements with no expression or an integer literal
