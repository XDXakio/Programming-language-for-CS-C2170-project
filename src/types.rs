use std::collections::HashMap;

use crate::term::Term;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Bool,
    Nat,
    Func(Box<Type>, Box<Type>),
}

pub type Context = HashMap<String, Type>;

#[derive(Debug, Clone)]
pub enum TypeError {
    UnboundVariable(String),
    Mismatch(Type, Type),
    ExpectedBool(Type),
    ExpectedNat(Type),
    ExpectedFunction(Type),
}

