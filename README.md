# A simple interpreter written in rust

A parser implemented using top-down operator precedence parsing.

## How it works:

1. The input is sent to the lexer for lexical analysis
2. The returned tokens are then passed to the parser. The parser creates an AST tree that can be evaluated.

## How to use it

```
# cargo run program.rmp
```

## Syntax example

```js
// Asign result to variable a
let main_expr  = (2-3)*4+3^5+4+(-3+4^4)+5+6+2-1;
let sin_result = sin(10/5+(main_expr+2));
let cos_result = cos(30);

let value = to_number(readln("Insert value: "));

if (value) {
    // Scope variable
    let value = "Scope value";

    // Print variable a
    println("Result main: ", main_expr);

    // Concatenation using + symbol
    println("Result sin(x): " + sin_result);

    println(str_concat("This ", "is ", "an ", "example"));
} else {
    println("Empty value");
}

// Function definition
func multiply(a, b) {
    return a * b;
}

// Calling user defined function
println("Multiplication: " + multiply(1, 2));

```

## TODO
- Write tests
- Implement mechanism on the interpreter to check for mandatory function arguments
- Check for undefined variables
- Improve syntax errors
- Add runtime errors
- Find a better way to handle unary tokens and parsing. Right now binary and unary are mixed as Operators.
- Prevent allocations with .clone()
- Implement returns
- Implement simple garbage collector
- Implement simple structures as objects and arrays
- Implement for and while