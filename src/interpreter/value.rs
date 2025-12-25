use std::{ops, rc::Rc};

use crate::error::error;

// Integer values and float should be distinguished, also boolean properly
// handled.
#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Integer(i64),
    Float(f64),
    String(Rc<str>),
    Boolean(bool),
    Empty,
}

impl Value {
    /// Convert to a string `Value::String(...)` (keeps same semantics you had).
    pub fn to_string(&self) -> Value {
        match self {
            Value::Integer(i) => Value::String(Rc::from(i.to_string())),
            Value::Float(f) => Value::String(Rc::from(f.to_string())),
            Value::Boolean(b) => Value::String(Rc::from(b.to_string())),
            Value::Empty => Value::String(Rc::from("")),
            Value::String(s) => Value::String(s.clone()), // cheap Rc clone
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
                if *b {
                    Value::Integer(1)
                } else {
                    Value::Integer(0)
                }
            }
            Value::Empty => Value::Integer(0),
            Value::String(s) => {
                // Try integer parse first, then float
                if let Ok(i) = s.parse::<i64>() {
                    Value::Integer(i)
                } else if let Ok(f) = s.parse::<f64>() {
                    Value::Float(f)
                } else {
                    error(format!("Unable to convert string '{}' to number", s).as_str())
                }
            }
        }
    }
    /// Force convert to integer
    pub fn to_i64(&self) -> i64 {
        match self.to_number() {
            Value::Integer(i) => i,
            Value::Float(f) => f as i64,
            other => error(format!("Expected numeric value, got {:?}", other).as_str()),
        }
    }

    /// Force-convert to Float (used when float math is required).
    pub fn to_f64(&self) -> f64 {
        match self.to_number() {
            Value::Integer(i) => i as f64,
            Value::Float(f) => f,
            other => error(format!("Expected numeric value, got {:?}", other).as_str()),
        }
    }

    pub fn into_rc(self) -> Rc<Value> {
        Rc::new(self)
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

    pub fn and_value(&self, other: &Value) -> Value {
        Value::Boolean(self.to_bool() && other.to_bool())
    }

    pub fn or_value(&self, other: &Value) -> Value {
        Value::Boolean(self.to_bool() || other.to_bool())
    }

    pub fn eq_value(&self, other: &Value) -> Value {
        let result = match (self, other) {
            // same-type
            (Value::Integer(a), Value::Integer(b)) => a == b,
            (Value::Float(a), Value::Float(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Boolean(a), Value::Boolean(b)) => a == b,
            (Value::Empty, Value::Empty) => true,

            // cross numeric
            (Value::Integer(a), Value::Float(b)) => (*a as f64) == *b,
            (Value::Float(a), Value::Integer(b)) => *a == (*b as f64),

            _ => false,
        };

        Value::Boolean(result)
    }

    pub fn gt_value(&self, other: &Value) -> Value {
        let result = match (self, other) {
            // same-type
            (Value::Integer(a), Value::Integer(b)) => a > b,
            (Value::Float(a), Value::Float(b)) => a > b,
            (Value::String(a), Value::String(b)) => a > b,
            (Value::Boolean(a), Value::Boolean(b)) => a > b,
            (Value::Empty, Value::Empty) => false,

            // cross numeric
            (Value::Integer(a), Value::Float(b)) => (*a as f64) > *b,
            (Value::Float(a), Value::Integer(b)) => *a > (*b as f64),

            _ => false,
        };

        Value::Boolean(result)
    }

    pub fn gte_value(&self, other: &Value) -> Value {
        let result = match (self, other) {
            // same-type
            (Value::Integer(a), Value::Integer(b)) => a >= b,
            (Value::Float(a), Value::Float(b)) => a >= b,
            (Value::String(a), Value::String(b)) => a >= b,
            (Value::Boolean(a), Value::Boolean(b)) => a >= b,
            (Value::Empty, Value::Empty) => false,

            // cross numeric
            (Value::Integer(a), Value::Float(b)) => (*a as f64) >= *b,
            (Value::Float(a), Value::Integer(b)) => *a >= (*b as f64),

            _ => false,
        };

        Value::Boolean(result)
    }

    pub fn lt_value(&self, other: &Value) -> Value {
        let result = match (self, other) {
            // same-type
            (Value::Integer(a), Value::Integer(b)) => a < b,
            (Value::Float(a), Value::Float(b)) => a < b,
            (Value::String(a), Value::String(b)) => a < b,
            (Value::Boolean(a), Value::Boolean(b)) => a < b,
            (Value::Empty, Value::Empty) => false,

            // cross numeric
            (Value::Integer(a), Value::Float(b)) => (*a as f64) < *b,
            (Value::Float(a), Value::Integer(b)) => *a < (*b as f64),

            _ => false,
        };

        Value::Boolean(result)
    }

    pub fn lte_value(&self, other: &Value) -> Value {
        let result = match (self, other) {
            // same-type
            (Value::Integer(a), Value::Integer(b)) => a <= b,
            (Value::Float(a), Value::Float(b)) => a <= b,
            (Value::String(a), Value::String(b)) => a <= b,
            (Value::Boolean(a), Value::Boolean(b)) => a <= b,
            (Value::Empty, Value::Empty) => false,

            // cross numeric
            (Value::Integer(a), Value::Float(b)) => (*a as f64) <= *b,
            (Value::Float(a), Value::Integer(b)) => *a <= (*b as f64),

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
    fn numeric_binop<FInt, FFloat>(
        left: &Value,
        right: &Value,
        int_op: FInt,
        float_op: FFloat,
    ) -> Value
    where
        FInt: Fn(i64, i64) -> Option<i64>,
        FFloat: Fn(f64, f64) -> f64,
    {
        let lnum = left.to_number();
        let rnum = right.to_number();

        match (lnum, rnum) {
            (Value::Integer(li), Value::Integer(ri)) => {
                if let Some(res_i) = int_op(li, ri) {
                    Value::Integer(res_i)
                } else {
                    let res_f = float_op(li as f64, ri as f64);
                    Value::Float(res_f)
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

    pub fn add_value(&self, right: &Value) -> Value {
        if matches!(self, Value::String(_)) || matches!(right, Value::String(_)) {
            let left_str = self.to_string();
            let right_str = right.to_string();

            if let (Value::String(ls), Value::String(rs)) = (left_str, right_str) {
                let mut buf = String::with_capacity(ls.len() + rs.len());
                buf.push_str(&ls);
                buf.push_str(&rs);
                return Value::String(Rc::from(buf));
            }
        }

        Value::numeric_binop(self, right, |a, b| a.checked_add(b), |a, b| a + b)
    }

    pub fn sub_value(&self, right: &Value) -> Value {
        Value::numeric_binop(self, right, |a, b| a.checked_sub(b), |a, b| a - b)
    }

    pub fn mul_value(&self, right: &Value) -> Value {
        Value::numeric_binop(self, right, |a, b| a.checked_mul(b), |a, b| a * b)
    }

    pub fn div_value(&self, right: &Value) -> Value {
        let lf = self.to_f64();
        let rf = right.to_f64();

        if rf == 0.0 {
            error("Division by zero");
        }
        Value::Float(lf / rf)
    }

    pub fn power(&self, right: &Value) -> Value {
        let left_f = self.to_f64();
        let right_f = right.to_f64();
        Value::Float(left_f.powf(right_f))
    }
}

impl ops::Add<Value> for Value {
    type Output = Value;

    fn add(self, right: Value) -> Value {
        self.add_value(&right)
    }
}

impl ops::Sub<Value> for Value {
    type Output = Value;

    fn sub(self, right: Value) -> Value {
        self.sub_value(&right)
    }
}

impl ops::Mul<Value> for Value {
    type Output = Value;

    fn mul(self, right: Value) -> Value {
        self.mul_value(&right)
    }
}

impl ops::Div<Value> for Value {
    type Output = Value;

    fn div(self, right: Value) -> Value {
        self.div_value(&right)
    }
}

impl Eq for Value {}

#[cfg(test)]
mod tests {
    use super::Value;
    use std::rc::Rc;

    #[test]
    fn and_uses_truthiness_rules() {
        let true_bool = Value::Boolean(true);
        let false_bool = Value::Boolean(false);
        let numeric_true = Value::Integer(10);
        let numeric_false = Value::Integer(0);

        assert_eq!(
            true_bool.and_value(&numeric_true),
            Value::Boolean(true)
        );
        assert_eq!(
            true_bool.and_value(&false_bool),
            Value::Boolean(false)
        );
        assert_eq!(
            numeric_false.and_value(&numeric_true),
            Value::Boolean(false)
        );
    }

    #[test]
    fn or_uses_truthiness_rules() {
        let empty = Value::Empty;
        let string_truthy = Value::String(Rc::from("yep"));
        let string_false = Value::String(Rc::from("false"));

        assert_eq!(empty.or_value(&string_truthy), Value::Boolean(true));
        assert_eq!(string_false.or_value(&empty), Value::Boolean(false));
    }
}
