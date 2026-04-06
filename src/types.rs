use std::collections::HashMap;
use std::hash::Hash;

use crate::term::Term;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    Bool,
    Nat,
    Func(Box<Type>, Box<Type>),
}

pub type Context = HashMap<String, Type>;

#[derive(Debug, Clone)]
pub enum TypeError {
    UnboundVariable(String),

    Mismatch {
        expected: Type,
        found: Type,
        context: &'static str,
    },

    ExpectedBool {
        found: Type,
        context: &'static str,
    },

    ExpectedNat {
        found: Type,
        context: &'static str,
    },

    ExpectedFunction {
        found: Type,
        context: &'static str,
    },
}

use Type::*;
use TypeError::*;

pub fn type_of(term: &Term, ctx: &mut Context) -> Result<Type, TypeError> {
    match term {
        Term::Var(x) => {
            ctx.get(x)
                .cloned()
                .ok_or(UnboundVariable(x.clone()))
        }

        Term::Abs { var, ty, body } => {
            let old = ctx.insert(var.clone(), ty.clone());
            let body_type = type_of(body, ctx)?;
            if let Some(old_ty) = old {
                ctx.insert(var.clone(), old_ty);
            } else {
                ctx.remove(var);
            }
            Ok(Type::Func(Box::new(ty.clone()), Box::new(body_type)))
        }

        Term::App(t1,t2 ) => {
            let  t1_type = type_of(t1, ctx)?;
            let  t2_type = type_of(t2, ctx)?;

            match t1_type {
                Func(param,ret ) => {
                    if *param == t2_type {
                        Ok(*ret)
                    } else {
                        Err(Mismatch {
                            expected: *param,
                            found: t2_type,
                            context: "function application",
                        })
                    }
                }
                other => Err(ExpectedFunction {
                                    found: other,
                                    context: "function application",
                                })
            }
        }

        Term::True | Term::False => Ok(Bool),

        Term::Ite { cond, if_true, if_false } => {
            let cond_type = type_of(cond, ctx)?;
            if cond_type != Bool {
                return Err(ExpectedBool {
                    found: cond_type,
                    context: "if condition",
                });
            }

            let t1 = type_of(if_true, ctx)?;
            let t2 = type_of(if_false, ctx)?;

            if t1 == t2 {
                Ok(t1)
            } else {
                Err(Mismatch {
                    expected: t1,
                    found: t2,
                    context: "if branches",
                })
            }
        }

        Term::Zero => Ok(Nat),

        Term::Succ(t) => {
            let inner = type_of(t, ctx)?;
            if inner == Nat {
                Ok(Nat)
            } else {
                Err(ExpectedNat {
                    found: inner,
                    context: "succ",
                })
            }
        }

        Term::Rec { scrutinee, if_zero, if_succ } => {
            let s_type = type_of(scrutinee, ctx)?;
            if s_type != Nat {
                return Err(ExpectedNat {
                    found: s_type,
                    context: "succ",
                })
            }

            let z_type = type_of(if_zero, ctx)?;
            let s_case_type = type_of(if_succ, ctx)?;

            match s_case_type {
                Func(nat_ty, rest) if *nat_ty == Nat => {
                    match *rest {
                        Func(t_ty, result_ty) if *t_ty == z_type => {
                            Ok(*result_ty)
                        }
                        other => Err(ExpectedFunction {
                            found: other,
                            context: "rec successor case",
                        }),
                    }
                }
                other => Err(ExpectedFunction {
                    found: other,
                    context: "rec successor case",
                }),
            }
        }

        Term::Add(t1, t2)
        | Term::Sub(t1, t2)
        | Term::Mul(t1, t2) => {
            let t1_ty = type_of(t1, ctx)?;
            let t2_ty = type_of(t2, ctx)?;

            if t1_ty == Type::Nat && t2_ty == Type::Nat {
                Ok(Type::Nat)
            } else {
                Err(ExpectedNat {
                    found: if t1_ty != Nat { t1_ty } else { t2_ty },
                    context: "arithmetic operation",
                })
            }
        }
    }
}