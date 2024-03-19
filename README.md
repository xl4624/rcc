# rcc

## Running the Compiler

Make sure you have Rust installed.

```bash
cargo run test.c # generates test.s
gcc test.s -o test.o
./test.o # echo $? to see the return value
```

## C Language Features

Current features:
- Function definitions and calls (with int or void return type but no parameters)
- Multiple statements in a function (you have to use a compound statement with curly braces)
- Return statements with no expression or an integer literal or a function call
- Checking for mismatched types in function declarations
