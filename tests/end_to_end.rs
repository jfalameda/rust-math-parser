mod harness;

use harness::{reset_assertions, take_assertions, AssertionRecord};
use parser::{
    interpreter::{ControlFlow, Interpreter, runtime_errors::RuntimeError},
    lexer, parser as ast_parser,
};

fn run_source(source: &str) -> (Result<(), RuntimeError>, Vec<AssertionRecord>) {
    reset_assertions();

    let mut token_parser = lexer::TokenParser::new(source.to_string());
    let tokens = token_parser.parse().expect("lexer should succeed");

    let mut parser = ast_parser::Parser::new(tokens);
    let ast = parser.parse().expect("parser should succeed");

    let mut interpreter = Interpreter::new();
    let result = interpreter.run(Some(ast.as_ref()));
    let assertions = take_assertions();

    (result, assertions)
}

#[allow(dead_code)]
fn run_script(path: &str) -> (Result<(), RuntimeError>, Vec<AssertionRecord>) {
    let source = std::fs::read_to_string(path).expect("script should be readable");
    run_source(&source)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn expect_assertions(source: &str, expected_messages: &[&str]) {
        let (result, assertions) = run_source(source);
        if let Err(err) = result {
            panic!("runtime error: {:?}", err);
        }

        assert_eq!(
            assertions.len(),
            expected_messages.len(),
            "unexpected number of assertions recorded: {:?}",
            assertions
                .iter()
                .map(|record| record.message.clone())
                .collect::<Vec<_>>()
        );

        for (record, expected) in assertions.iter().zip(expected_messages.iter()) {
            assert_eq!(record.message, *expected, "assertion message mismatch");
            assert!(record.passed, "assertion '{}' did not pass", record.message);
        }
    }

    #[test]
    fn assert_harness_records_results() {
        let source = r#"
        let total = 1 + 2 + 3;
        assert("sum should equal 6", total == 6);
        "#;

        expect_assertions(source, &["sum should equal 6"]);
    }

    #[test]
    fn executes_literals_and_operators() {
        let source = r#"
        // Arithmetic and literal coverage
        let int_value = 42;
        let float_value = 3.5;
        let negative_literal = -10;
        let negative_of_var = -int_value;
        let bool_true = true;
        let bool_false = false;
        let string_value = "hello";
        let complex = (int_value + float_value) * 2 - 4 / 2;
        let exponent_chain = 2 ^ 3 ^ 2;
        let parentheses_result = (1 + 2) * (3 + 4);
        let string_concat = "hello " + "world";
        let string_number_concat = "value: " + int_value;
        let string_bool_concat = "bool: " + bool_true;
        let eq_check = int_value == 42;
        let neq_check = int_value != 0;
        let gt_check = int_value > 10;
        let gte_check = int_value >= 42;
        let lt_check = float_value < 4;
        let lte_check = float_value <= 3.5;
        let comparison_chain = (int_value > 10) == true;
        let not_check = !bool_false;
        assert("integer literal equality", int_value == 42);
        assert("float literal equality", float_value == 3.5);
        assert("unary minus literal", negative_literal == -10);
        assert("unary minus identifier", negative_of_var == -42);
        assert("boolean literal truthiness", bool_true);
        assert("string literal equality", string_value == "hello");
        assert("string concatenation", string_concat == "hello world");
        assert("string + number", string_number_concat == "value: 42");
        assert("string + bool", string_bool_concat == "bool: true");
        assert("parentheses grouping", parentheses_result == 21);
        assert("complex mixed arithmetic", complex == 89);
        assert("exponent precedence", exponent_chain == 512);
        assert("equality operator", eq_check);
        assert("inequality operator", neq_check);
        assert("greater than operator", gt_check);
        assert("greater than or equal operator", gte_check);
        assert("less than operator", lt_check);
        assert("less than or equal operator", lte_check);
        assert("comparison chain boolean", comparison_chain);
        assert("logical not operator", not_check);
        "#;

        expect_assertions(
            source,
            &[
                "integer literal equality",
                "float literal equality",
                "unary minus literal",
                "unary minus identifier",
                "boolean literal truthiness",
                "string literal equality",
                "string concatenation",
                "string + number",
                "string + bool",
                "parentheses grouping",
                "complex mixed arithmetic",
                "exponent precedence",
                "equality operator",
                "inequality operator",
                "greater than operator",
                "greater than or equal operator",
                "less than operator",
                "less than or equal operator",
                "comparison chain boolean",
                "logical not operator",
            ],
        );
    }

    #[test]
    fn executes_logical_operators() {
        let source = r#"
        func fail_and() {
            assert("right operand of && evaluated unexpectedly", false);
            return true;
        }

        func fail_or() {
            assert("right operand of || evaluated unexpectedly", false);
            return false;
        }

        func record_and() {
            assert("&& evaluates right operand when left true", true);
            return true;
        }

        func record_or() {
            assert("|| evaluates right operand when left false", true);
            return true;
        }

        let and_short = false && fail_and();
        let and_truthy = true && record_and();
        let or_short = true || fail_or();
        let or_truthy = false || record_or();
        assert("&& short-circuits false left", !and_short);
        assert("&& produces truthy when both true", and_truthy);
        assert("|| short-circuits true left", or_short);
        assert("|| produces truthy when right true", or_truthy);
        "#;

        expect_assertions(
            source,
            &[
                "&& evaluates right operand when left true",
                "|| evaluates right operand when left false",
                "&& short-circuits false left",
                "&& produces truthy when both true",
                "|| short-circuits true left",
                "|| produces truthy when right true",
            ],
        );
    }

    #[test]
    fn executes_conditionals_and_scopes() {
        let source = r#"
        let outer = "outer";
        if (outer == "outer") {
            let outer = "inner";
            assert("if block runs", outer == "inner");
        }
        assert("outer scope preserved", outer == "outer");
        if (false) {
            assert("unreachable branch not executed", false);
        } else {
            assert("else block executes", true);
        }
        if (true) assert("single statement if executes", true);
        if (false) assert("single statement else-if unreachable", false); else assert("single statement else executes", true);
        let nested_flag = false;
        if (false) {
            assert("nested outer false branch skipped", false);
        } else {
            if (nested_flag == false) {
                assert("nested else-if executes branch", true);
            } else {
                assert("nested else-if fallback skipped", false);
            }
        }
        "#;

        expect_assertions(
            source,
            &[
                "if block runs",
                "outer scope preserved",
                "else block executes",
                "single statement if executes",
                "single statement else executes",
                "nested else-if executes branch",
            ],
        );
    }

    #[test]
    fn executes_functions_and_returns() {
        let source = r#"
        func double(n) {
            return n * 2;
        }

        func conditional_sum(a, b) {
            let sum = a + b;
            if (sum > 10) {
                return sum;
            } else {
                return sum + 10;
            }
        }

        func uses_shadowing(value) {
            let value = value + 1;
            return value;
        }

        func return_breaks_flow() {
            return true;
            assert("Statement after return not executed", false);
        }

        let doubled = double(21);
        let high_sum = conditional_sum(6, 7);
        let low_sum = conditional_sum(2, 3);
        let shadowed = uses_shadowing(4);
        return_breaks_flow();
        assert("double returns value", doubled == 42);
        assert("conditional sum returns branch result", high_sum == 13);
        assert("conditional sum returns else result", low_sum == 15);
        assert("function arguments shadow correctly", shadowed == 5);

        func invoke_nested(x) {
            func increment(y) {
                return y + 1;
            }
            let inc = increment(x);
            let dbl = double(x);
            return inc + dbl;
        }

        let nested_result = invoke_nested(3);
        assert("nested function call works", nested_result == 10);
        "#;

        expect_assertions(
            source,
            &[
                "double returns value",
                "conditional sum returns branch result",
                "conditional sum returns else result",
                "function arguments shadow correctly",
                "nested function call works",
            ],
        );
    }

    #[test]
    fn executes_builtins_and_coercions() {
        let source = r#"
        let sin_zero = sin(0.0);
        let cos_zero = cos(0);
        let concatenated = str_concat("The ", "answer ", "is ", 42);
        let coerced_string_sum = "Number: " + 5;
        let bool_concat = "Bool: " + false;
        let to_number_int = to_number("123");
        let to_number_float = to_number("3.25");
        let print_value = print("Hello", " ", "World");
        let println_value = println("Hello line");
        let print_coerced = str_concat("prefix", print_value);
        let println_coerced = str_concat("prefix", println_value);
        assert("sin(0) equals 0", sin_zero == 0);
        assert("cos(0) equals 1", cos_zero == 1);
        assert("str_concat concatenates all arguments", concatenated == "The answer is 42");
        assert("string plus number coerces number", coerced_string_sum == "Number: 5");
        assert("string plus bool coerces bool", bool_concat == "Bool: false");
        assert("to_number parses integer", to_number_int == 123);
        assert("to_number parses float", to_number_float == 3.25);
        assert("print returns falsy empty value", !print_value);
        assert("println returns falsy empty value", !println_value);
        assert("print coerces empty value to string", print_coerced == "prefix");
        assert("println coerces empty value to string", println_coerced == "prefix");
        "#;

        expect_assertions(
            source,
            &[
                "sin(0) equals 0",
                "cos(0) equals 1",
                "str_concat concatenates all arguments",
                "string plus number coerces number",
                "string plus bool coerces bool",
                "to_number parses integer",
                "to_number parses float",
                "print returns falsy empty value",
                "println returns falsy empty value",
                "print coerces empty value to string",
                "println coerces empty value to string",
            ],
        );
    }
}
