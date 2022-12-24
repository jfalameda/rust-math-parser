# A simple math expressions parser written in rust

A math parser implemented using top-down operator precedence parsing.

## How it works:

1. The input is sent to the lexer for lexical analysis
2. The returned tokens are then passed to the parser. The parser creates an AST tree that can be evaluated to retrieve the result.

## How to use it

```rust
let program = String::from("-(2-3)*4+3^5+4+(-3+4^4)+5+6+2-1");
let mut token_parser = lexer::TokenParser::new(program);
let tokens = token_parser.parse();
let mut parser = parser::Parser::new(tokens);
let result = parser.evaluate();
```

## TODO
- Prevent malformed floating numbers
- Refactor for clarity
- Write tests
- Accept input from console
- Exponential binary operations should be right-hand associative