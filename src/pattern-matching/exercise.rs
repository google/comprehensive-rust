// Copyright 2023 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![allow(dead_code)]
// ANCHOR: solution
// ANCHOR: Operation
/// An operation to perform on two subexpressions.
#[derive(Debug)]
enum Operation {
    /// Add two Int values, or concatenate two String values.
    Add,
    /// Subtract two Int values.
    Sub,
}
// ANCHOR_END: Operation

// ANCHOR: Value
/// A value, the result of evaluating expression.
#[derive(Debug, PartialEq, Eq)]
enum Value {
    /// An integer value.
    Int(i64),
    /// A string value.
    String(String),
}
// ANCHOR_END: Value

// ANCHOR: Expression
/// A numerical expression, in tree form.
#[derive(Debug)]
enum Expression {
    /// An operation on two subexpressions.
    Op {
        op: Operation,
        left: Box<Expression>,
        right: Box<Expression>,
    },

    /// A literal value.
    Value(Value),
}
// ANCHOR_END: Expression

// ANCHOR: eval
fn eval(e: Expression) -> Result<Value, String> {
    // ANCHOR_END: eval
    match e {
        Expression::Op { op, left, right } => {
            let left = match eval(*left) {
                Ok(v) => v,
                e @ Err(_) => return e,
            };
            let right = match eval(*right) {
                Ok(v) => v,
                e @ Err(_) => return e,
            };
            Ok(match (op, left, right) {
                (Operation::Add, Value::Int(l), Value::Int(r)) => Value::Int(l + r),
                (Operation::Add, Value::String(l), Value::String(r)) => {
                    Value::String(l + &r)
                }
                (Operation::Sub, Value::Int(l), Value::Int(r)) => Value::Int(l - r),
                _ => return Err(String::from("invalid operand types")),
            })
        }
        Expression::Value(v) => Ok(v),
    }
}

// ANCHOR: tests
#[test]
fn test_value() {
    assert_eq!(eval(Expression::Value(Value::Int(19))), Ok(Value::Int(19)));
}

#[test]
fn test_sum() {
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Add,
            left: Box::new(Expression::Value(Value::Int(10))),
            right: Box::new(Expression::Value(Value::Int(20))),
        }),
        Ok(Value::Int(30))
    );
}

#[test]
fn test_string_sum() {
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Add,
            left: Box::new(Expression::Value(Value::String(String::from("foo")))),
            right: Box::new(Expression::Value(Value::String(String::from("bar")))),
        }),
        Ok(Value::String(String::from("foobar")))
    );
}

#[test]
fn test_recursion() {
    let term1 = Expression::Op {
        op: Operation::Add,
        left: Box::new(Expression::Value(Value::Int(10))),
        right: Box::new(Expression::Value(Value::Int(9))),
    };
    let term2 = Expression::Op {
        op: Operation::Add,
        left: Box::new(Expression::Op {
            op: Operation::Sub,
            left: Box::new(Expression::Value(Value::Int(3))),
            right: Box::new(Expression::Value(Value::Int(4))),
        }),
        right: Box::new(Expression::Value(Value::Int(5))),
    };
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Add,
            left: Box::new(term1),
            right: Box::new(term2),
        }),
        Ok(Value::Int(23))
    );
}

#[test]
fn test_incompatible_types() {
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Add,
            left: Box::new(Expression::Value(Value::Int(99))),
            right: Box::new(Expression::Value(Value::String(String::from("foo")))),
        }),
        Err(String::from("invalid operand types"))
    );
}
// ANCHOR_END: tests

fn main() {
    let expr = Expression::Op {
        op: Operation::Sub,
        left: Box::new(Expression::Value(Value::Int(20))),
        right: Box::new(Expression::Value(Value::Int(10))),
    };
    println!("expr: {:?}", expr);
    println!("result: {:?}", eval(expr));
}
