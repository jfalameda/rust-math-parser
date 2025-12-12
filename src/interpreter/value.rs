use std::ops;

use crate::error;


// Integer values and float should be distinguished, also boolean properly
// handled.
#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Empty,
}

impl Value {
    /// Convert to a string `Value::String(...)` (keeps same semantics you had).
    pub fn to_string(&self) -> Value {
        match self {
            Value::Integer(i) => Value::String(i.to_string()),
            Value::Float(f) => Value::String(f.to_string()),
            Value::String(s) => Value::String(s.to_string()),
            Value::Empty => Value::String("".to_string()),
            Value::Boolean(b) => Value::String(b.to_string()),
        }
    }

    /// Try to coerce this value to a numeric Value::Float or Value::Integer (keeps integer if it was integer).
    ///
    /// - Integer -> Integer(i)
    /// - Float -> Float(f)
    /// - Boolean -> Integer(1) or Integer(0)
    /// - Empty -> Integer(0)
    /// - String -> parsed Float if contains '.' or exponent or fails -> error
    pub fn to_number(&self) -> Value {
        match self {
            Value::Integer(_) | Value::Float(_) => self.clone(),
            Value::Boolean(b) => {
                if *b { Value::Integer(1) } else { Value::Integer(0) }
            }
            Value::Empty => Value::Integer(0),
            Value::String(s) => {
                // Try integer parse first, then float
                if let Ok(i) = s.parse::<i64>() {
                    Value::Integer(i)
                } else if let Ok(f) = s.parse::<f64>() {
                    Value::Float(f)
                } else {
                    error(format!("Unable to convert string '{}' to number", s))
                }
            }
        }
    }

    /// Force-convert to Float (used when float math is required).
    pub fn to_f64(&self) -> f64 {
        match self.to_number() {
            Value::Integer(i) => i as f64,
            Value::Float(f) => f,
            other => error(format!("Expected numeric value, got {:?}", other)),
        }
    }

    pub fn to_bool(&self) -> bool {
        match self {
            Value::Boolean(b) => *b,

            // Integers: 0 = false, non-zero = true
            Value::Integer(i) => *i != 0,

            // Floats: 0.0 = false, everything else = true
            Value::Float(f) => *f != 0.0,

            // Empty = false
            Value::Empty => false,

            // Strings: trim whitespace, then:
            // "" or "0" or "false" (case-insensitive) → false
            // anything else → true
            Value::String(s) => {
                let t = s.trim().to_ascii_lowercase();
                !(t.is_empty() || t == "0" || t == "false")
            }
        }
    }

    pub fn eq_value(&self, other: &Value) -> Value {
        let result = match (self, other) {
            // same-type
            (Value::Integer(a), Value::Integer(b)) => a == b,
            (Value::Float(a),   Value::Float(b))   => a == b,
            (Value::String(a),  Value::String(b))  => a == b,
            (Value::Boolean(a), Value::Boolean(b)) => a == b,
            (Value::Empty,      Value::Empty)      => true,

            // cross numeric
            (Value::Integer(a), Value::Float(b)) => (*a as f64) == *b,
            (Value::Float(a),   Value::Integer(b)) => *a == (*b as f64),

            _ => false,
        };

        Value::Boolean(result)
    }

    pub fn neq_value(&self, other: &Value) -> Value {
        match self.eq_value(other) {
            Value::Boolean(b) => Value::Boolean(!b),
            _ => unreachable!(),
        }
    }

    /// Internal helper: unify numeric handling and preserve integer results when possible.
    ///
    /// Returns `Value::Integer(i64)` if both operands were integers and operation result fits in i64,
    /// otherwise `Value::Float`.
    fn numeric_binop<F_int, F_float>(left: Value, right: Value, int_op: F_int, float_op: F_float) -> Value
    where
        F_int: Fn(i64, i64) -> Option<i64>,
        F_float: Fn(f64, f64) -> f64,
    {
        let lnum = left.to_number();
        let rnum = right.to_number();

        match (lnum, rnum) {
            (Value::Integer(li), Value::Integer(ri)) => {
                // try integer operation (some ops may overflow or be undefined)
                if let Some(res_i) = int_op(li, ri) {
                    return Value::Integer(res_i);
                } else {
                    // fall back to float op if integer op can't represent the result
                    let res_f = float_op(li as f64, ri as f64);
                    return Value::Float(res_f);
                }
            }
            (lother, rother) => {
                // Coerce both to f64 and run float op
                let lf = match lother {
                    Value::Integer(i) => i as f64,
                    Value::Float(f) => f,
                    _ => unreachable!(), // to_number ensures numeric variants
                };
                let rf = match rother {
                    Value::Integer(i) => i as f64,
                    Value::Float(f) => f,
                    _ => unreachable!(),
                };
                Value::Float(float_op(lf, rf))
            }
        }
    }

    pub fn power(&self, right: Value) -> Value {
        let left_f = self.to_f64();
        let right_f = right.to_f64();
        Value::Float(left_f.powf(right_f))
    }
}

impl ops::Add<Value> for Value {
    type Output = Value;

    fn add(self, right: Value) -> Value {
        // If either side is a String (or converts to string?) the operation should be concatenation.
        // We follow the rule: if either VARIANT is String, do textual concatenation using `to_string()`.
        if matches!(self, Value::String(_)) || matches!(right, Value::String(_)) {
            let s_left = match self {
                Value::String(_) => self.to_string(), // returns Value::String(...)
                other => other.to_string(),
            };
            let s_right = match right {
                Value::String(_) => right.to_string(),
                other => other.to_string(),
            };

            // both to_string() returned Value::String
            if let (Value::String(ls), Value::String(rs)) = (s_left, s_right) {
                return Value::String(format!("{}{}", ls, rs));
            } else {
                // defensive fallback; shouldn't happen
                return error("String concatenation failed.".to_string());
            }
        }

        // Numeric addition, preserve integer result when both are integers and no overflow
        Value::numeric_binop(self, right,
            // integer op -> attempt checked_add and return Some if success else None
            |a, b| a.checked_add(b),
            // float op
            |a, b| a + b
        )
    }
}

impl ops::Sub<Value> for Value {
    type Output = Value;

    fn sub(self, right: Value) -> Value {
        Value::numeric_binop(self, right,
            |a, b| a.checked_sub(b),
            |a, b| a - b
        )
    }
}

impl ops::Mul<Value> for Value {
    type Output = Value;

    fn mul(self, right: Value) -> Value {
        Value::numeric_binop(self, right,
            |a, b| a.checked_mul(b),
            |a, b| a * b
        )
    }
}

impl ops::Div<Value> for Value {
    type Output = Value;

    fn div(self, right: Value) -> Value {
        // division always as float (to preserve fractional results and avoid divide-by-zero integer traps)
        let lf = self.to_f64();
        let rf = right.to_f64();

        // guard divide by zero
        if rf == 0.0 {
            error("Division by zero".to_string());
        }
        Value::Float(lf / rf)
    }
}

impl Eq for Value {}

/// Convert trait used in your code base. Keep implementations for types you used.
pub trait Convert: Sized {
    fn convert(v: Value) -> Option<Self>;
}

macro_rules! impl_convert {
    ($t:ty, $id:ident) => (
        impl Convert for $t {
            fn convert(v: Value) -> Option<$t> {
                match v {
                    Value::$id(x) => Some(x),
                    _ => None,
                }
            }
        }
    )
}

impl_convert!(f64, Float);
impl_convert!(String, String);
impl_convert!(i64, Integer);
impl_convert!(bool, Boolean);
