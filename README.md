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

    // Print variable ax
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

## Language syntax

### Comments
- Single-line comments start with `//` and run until the end of the line.

### Literals
- Integers (`42`), floats (`3.14`), booleans (`true`, `false`) and double quoted strings (`"hello"`).
- Numeric literals support unary negation (e.g. `-5`) and exponentiation via `^`.

### Variables and scope
- Declare variables with `let name = expression;`. Declarations must include an initializer.
- Variables are scoped to the surrounding block delimited by `{ ... }`.

### Statements and semicolons
- Expression statements, assignments and variable declarations must end with `;`.
- `if` and `func` introduce blocks; branches may use either braces or a single statement (which still requires a semicolon).

### Operators
| Category        | Operators                    | Notes |
|-----------------|------------------------------|-------|
| Arithmetic      | `+`, `-`, `*`, `/`, `^`      | `^` is right-associative; `/` performs floating-point division when needed. |
| Comparison      | `==`, `!=`, `>`, `>=`, `<`, `<=` | Yield boolean results. |
| Unary           | `-`, `!`                     | `-` negates numbers; `!` negates booleans. |
| Boolean         | `&&`, `\|\|`                   | Short-circuit evaluation using the runtime's truthiness rules. |

- If either operand of `+` is a string, the result is string concatenation.

### Functions
- Define a function with `func name(arg1, arg2) { ... }`.
- Use `return expression;` inside functions to produce a value. Returning outside a function raises a runtime error.
- Call user-defined or built-in functions with `name(arg1, arg2);`.

### Conditionals
- `if (condition) { ... } else { ... }` evaluates the condition is true; an `else` block is optional.
- Boolean rules follow the runtime: `0`, `0.0`, empty strings, the string literals `"0"`/`"false"`, and `false` behave as false; everything else is true.
- Single statement branches can omit braces but still require terminating semicolons.

### Built-in functions
- **`print(...)` / `println(...)`** — write values to stdout (with or without a newline).
- **`readln(...)`** — print an optional prompt and return the entered line as a string.
- **`sin(value)` / `cos(value)`** — trigonometric functions that coerce arguments to numbers.
- **`str_concat(...)`** — concatenate multiple values as strings.
- **`to_number(value)`** — convert strings or other values into numeric types when possible.

## TODO
- Write tests
- Implement mechanism on the interpreter to check for mandatory function arguments (consider semantic analysis)
- Make functions first class citizens
- Check for undefined variables (also semantic analysis)
- Implement assignment without declaration
- Improve syntax errors
- Value should return results and produce runtime errors.
- Find a better way to handle unary tokens and parsing. Right now binary and unary are mixed as Operators.
- Implement simple garbage collector
- Implement simple structures as objects and arrays
- Implement for and while
- Built-in functions should be able to throw errors
- Implement reserved words
- Implement objects