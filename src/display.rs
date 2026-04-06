use std::fmt::Display;
use std::fmt;

use crate::{ast::decode_nat, t, term::Term, types::{ Type, TypeError },};
use Term::*;

impl Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Nat => write!(f, "Nat"),
            Type::Bool => write!(f, "Bool"),
            Type::Func(t1, t2) => write!(f, "({} -> {})", t1, t2),
            Type::Pair(t1, t2) => write!(f, "({}, {})", t1, t2),
        }
    }
}

impl fmt::Display for TypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TypeError::UnboundVariable(x) => {
                write!(f, "Unbound variable: {}", x)
            }

            TypeError::Mismatch { expected, found, context } => {
                write!(
                    f,
                    "Type mismatch in {}: expected {:?}, found {:?}",
                    context, expected, found
                )
            }

            TypeError::ExpectedBool { found, context } => {
                write!(
                    f,
                    "Expected Bool in {}, but found {:?}",
                    context, found
                )
            }

            TypeError::ExpectedNat { found, context } => {
                write!(
                    f,
                    "Expected Nat in {}, but found {:?}",
                    context, found
                )
            }

            TypeError::ExpectedFunction { found, context } => {
                write!(
                    f,
                    "Expected function in {}, but found {:?}",
                    context, found
                )
            }

            TypeError::ExpectedPair { found, context } => {
                write!(
                    f,
                    "Expected pair in {}, but found {}",
                    context, found
                )
            }
        }
    }
}

impl Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Attempt to decode as nat
        if let Some(n) = decode_nat(self) {
            return write!(f, "{n}");
        }
        match self {
            Var(x) => write!(f, "{x}"),
            Abs { var, ty, body } => {
                write!(f, "({var}: {ty} => {body})")
            }
            App(t1, t2) => {
                if matches!(**t1, Abs { .. }) {
                    write!(f, "({t1}) ")?;
                } else {
                    write!(f, "{t1} ")?;
                }

                if matches!(**t2, Abs { .. } | App(_, _) | Rec { .. }) {
                    write!(f, "({t2})")
                } else {
                    write!(f, "{t2}")
                }
            }
            True => write!(f, "true"),
            False => write!(f, "false"),
            Ite {
                cond,
                if_true,
                if_false,
            } => {
                write!(f, "if {cond} then {if_true} else {if_false}")
            }
            Zero => write!(f, "Z"),
            Succ(t) => write!(f, "S({t})"),
            Rec {
                scrutinee,
                if_zero,
                if_succ,
            } => {
                let aux = t!(rec !(*scrutinee.clone()) !(*if_zero.clone()) !(*if_succ.clone()));
                write!(f, "{aux}")
            }
            Add(a, b) => write!(f, "{a} + {b}"),
            Sub(a, b) => write!(f, "{a} - {b}"),
            Mul(a, b) => write!(f, "{a} * {b}"),
            Pair(t1, t2) => write!(f, "({t1}, {t2})"),

            Fst(t) => {
                if matches!(**t, App(_, _) | Abs { .. }) {
                    write!(f, "fst ({t})")
                } else {
                    write!(f, "fst {t}")
                }
            }

            Snd(t) => {
                if matches!(**t, App(_, _) | Abs { .. }) {
                    write!(f, "snd ({t})")
                } else {
                    write!(f, "snd {t}")
                }
            }
        }
    }
}
