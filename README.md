# A simple math expressions parser written in rust

A math parser implemented using top-down operator precedence parsing.

## How it works:

1. The input is sent to the lexer for lexical analysis
2. The returned tokens are then passed to the parser. The parser creates an AST tree that can be evaluated to retrieve the result.

## How to use it

```
# cargo run program.rmp
```

## Syntax example

```js
// Asign result to variable a
let a = (2-3)*4+3^5+4+(-3+4^4)+5+6+2-1;

let b = sin(10/5+(a+2));
let c = cos(30);

// Print variable a
println("Result a: ", a);

// Print variable b
println(b + c);

println(str_concat("This ", "is ", "an ", "example"));

```

## TODO
- Write tests
- Exponential binary operations should be right-hand associative
- Check for undefined variables
- Add line number to errors
- Improve syntax errors
- Use peekabe iterable [https://doc.rust-lang.org/stable/std/iter/struct.Peekable.html](https://doc.rust-lang.org/stable/std/iter/struct.Peekable.html)
