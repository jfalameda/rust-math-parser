# A simple math expressions parser written in rust

A math parser implemented using top-down operator precedence parsing.

## How it works:

1. The input is sent to the lexer for lexical analysis
2. The returned tokens are then passed to the parser. The parser creates an AST tree that can be evaluated to retrieve the result.

## How to use it

```rust
# cargo run program.rmp
```

## Syntax example

```javascript
// Asign result to variable a
let a = (2-3)*4+3^5+4+(-3+4^4)+5+6+2-1;
let b = sin(10/5+a);
let c = cos(30);

// Print variable a
println(a);
println(b + c);

```

## TODO
- Refactor for clarity
- Write tests
- Exponential binary operations should be right-hand associative
- Check for undefined variables
