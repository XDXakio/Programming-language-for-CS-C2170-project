use crate::{
    module::Module,
    t,
    term::{Term, nat},
    types::Type,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AST {
    // Core lambda calculus
    Var(String),
    Abs {
        var: String,
        ty: Type,
        body: Box<AST>,
    },
    App(Box<AST>, Box<AST>),

    // Boolean fragment
    True,
    False,
    Ite {
        cond: Box<AST>,
        if_true: Box<AST>,
        if_false: Box<AST>,
    },

    // Natural number fragment
    Zero,
    Succ(Box<AST>),
    Rec {
        scrutinee: Box<AST>,
        if_zero: Box<AST>,
        if_succ: Box<AST>,
    },

    // Surface syntax sugar
    /// A name in the surrounding module
    Name(String),
    And(Box<AST>, Box<AST>),
    Or(Box<AST>, Box<AST>),
    Not(Box<AST>),
    Nat(u64),
    Add(Box<AST>, Box<AST>),
    Sub(Box<AST>, Box<AST>),
    Mul(Box<AST>, Box<AST>),
    Eq(Box<AST>, Box<AST>),
    Neq(Box<AST>, Box<AST>),
    Le(Box<AST>, Box<AST>),
    Lt(Box<AST>, Box<AST>),
    Ge(Box<AST>, Box<AST>),
    Gt(Box<AST>, Box<AST>),
    Pair(Box<AST>, Box<AST>),
    Fst(Box<AST>),
    Snd(Box<AST>),
    Nil,
    TypedNil(Type),
    Cons(Box<AST>, Box<AST>),
    List(Vec<AST>),
    Head(Box<AST>),
    Tail(Box<AST>),
    IsEmpty(Box<AST>),
}

use AST::*;

/// Attempts to decode a natural number term to an integer
pub fn decode_nat(mut t: &Term) -> Option<u64> {
    let mut n = 0;
    while let Term::Succ(t1) = t {
        t = &**t1;
        n += 1;
    }
    if let Term::Zero = t { Some(n) } else { None }
}

pub fn and() -> AST {
    AST::Abs {
        var: "b1".to_string(),
        ty: Type::Bool,
        body: Box::new(AST::Abs {
            var: "b2".to_string(),
            ty: Type::Bool,
            body: Box::new(AST::Ite {
                cond: Box::new(AST::Var("b1".to_string())),
                if_true: Box::new(AST::Var("b2".to_string())),
                if_false: Box::new(AST::False),
            }),
        }),
    }
}

pub fn not() -> AST {
    AST::Abs {
        var: "a".to_string(),
        ty: Type::Bool,
        body: Box::new(AST::Ite {
            cond: Box::new(AST::Var("a".to_string())),
            if_true: Box::new(AST::False),
            if_false: Box::new(AST::True),
        }),
    }
}

pub fn or() -> AST {
    AST::Abs {
        var: "b1".to_string(),
        ty: Type::Bool,
        body: Box::new(AST::Abs {
            var: "b2".to_string(),
            ty: Type::Bool,
            body: Box::new(AST::Ite {
                cond: Box::new(AST::Var("b1".to_string())),
                if_true: Box::new(AST::True),
                if_false: Box::new(AST::Var("b2".to_string())),
            }),
        }),
    }
}

/// The predecessor function for natural numbers
pub fn pred() -> AST {
    AST::Abs {
        var: "n".to_string(),
        ty: Type::Nat,
        body: Box::new(AST::Rec {
            scrutinee: Box::new(AST::Var("n".to_string())),
            if_zero: Box::new(AST::Zero),
            if_succ: Box::new(AST::Abs {
                var: "pred".to_string(),
                ty: Type::Nat,
                body: Box::new(AST::Abs {
                    var: "ih".to_string(),
                    ty: Type::Nat,
                    body: Box::new(AST::Var("pred".to_string())),
                }),
            }),
        }),
    }
}

pub fn is_zero() -> AST {
    Abs {
        var: "n".to_string(),
        ty: Type::Nat, 
        body: Box::new(Rec {
            scrutinee: Box::new(Var("n".to_string())),
            if_zero: Box::new(True),
            if_succ: Box::new(Abs {           // Nat -> Bool -> Bool
                var: "pred".to_string(),
                ty: Type::Nat,
                body: Box::new(Abs {
                    var: "ih".to_string(),
                    ty: Type::Bool,           // FIXED: Bool, not Nat!
                    body: Box::new(False),
                }),
            }),
        }),
    }
}

pub fn zero() -> AST {
    Zero
}

pub fn eq() -> AST {
    Abs {
        var: "n".to_string(),
        ty: Type::Nat,
        body: Box::new(Abs {
            var: "m".to_string(),
            ty: Type::Nat,
            body: Box::new(And(                    // And(t1, t2) - TWO args!
                Box::new(App(                      // is_zero(n - m)
                    Box::new(is_zero()),
                    Box::new(Sub(
                        Box::new(Var("n".to_string())),
                        Box::new(Var("m".to_string())),
                    )),
                )),
                Box::new(App(                      // is_zero(m - n)  
                    Box::new(is_zero()),
                    Box::new(Sub(
                        Box::new(Var("m".to_string())),
                        Box::new(Var("n".to_string())),
                    )),
                )),
            )),
        }),
    }
}

pub fn neq() -> AST {
    AST::Abs {
        var: "n".to_string(),
        ty: Type::Nat,
        body: Box::new(AST::Abs {
            var: "m".to_string(),
            ty: Type::Nat,
            body: Box::new(AST::App(
                Box::new(not()),
                Box::new(AST::App(
                    Box::new(AST::App(Box::new(eq()), Box::new(AST::Var("n".to_string())))),
                    Box::new(AST::Var("m".to_string())),
                )),
            )),
        }),
    }
}

pub fn le() -> AST {
    AST::Abs {
        var: "n".to_string(),
        ty: Type::Nat,
        body: Box::new(AST::Abs {
            var: "m".to_string(),
            ty: Type::Nat,
            body: Box::new(AST::App(
                Box::new(is_zero()),
                Box::new(AST::Sub(
                    Box::new(AST::Var("n".to_string())),
                    Box::new(AST::Var("m".to_string())),
                )),
            )),
        }),
    }
}

pub fn lt() -> AST {
    AST::Abs {
        var: "n".to_string(),
        ty: Type::Nat,
        body: Box::new(AST::Abs {
            var: "m".to_string(),
            ty: Type::Nat,
            body: Box::new(AST::App(
                Box::new(AST::App(
                    Box::new(le()),
                    Box::new(AST::Succ(Box::new(AST::Var("n".to_string())))),
                )),
                Box::new(AST::Var("m".to_string())),
            )),
        }),
    }
}

pub fn ge() -> AST {
    AST::Abs {
        var: "n".to_string(),
        ty: Type::Nat,
        body: Box::new(AST::Abs {
            var: "m".to_string(),
            ty: Type::Nat,
            body: Box::new(AST::App(
                Box::new(AST::App(Box::new(le()), Box::new(AST::Var("m".to_string())))),
                Box::new(AST::Var("n".to_string())),
            )),
        }),
    }
}

pub fn gt() -> AST {
    AST::Abs {
        var: "n".to_string(),
        ty: Type::Nat,
        body: Box::new(AST::Abs {
            var: "m".to_string(),
            ty: Type::Nat,
            body: Box::new(AST::App(
                Box::new(AST::App(Box::new(lt()), Box::new(AST::Var("m".to_string())))),
                Box::new(AST::Var("n".to_string())),
            )),
        }),
    }
}

fn app1(f: Term, a: Term) -> Term {
    Term::App(Box::new(f), Box::new(a))
}

fn app2(f: Term, a: Term, b: Term) -> Term {
    app1(app1(f, a), b)
}

impl AST {
    pub fn desugar(self, env: &Module) -> Term {
        let d = |s: AST| s.desugar(env);

        match self {
            Var(x) => Term::Var(x),
            Abs { var,ty, body } => Term::Abs {
                var,
                ty,
                body: Box::new(d(*body)),
            },
            App(t1, t2) => t!(!(d(*t1)) !(d(*t2))),
            True => Term::True,
            False => Term::False,
            And(t1, t2) => {
                let f = env.get_term("and").expect("and in env");
                app2(f, d(*t1), d(*t2))
            }

            Or(t1, t2) => {
                let f = env.get_term("or").expect("or in env");
                app2(f, d(*t1), d(*t2))
            }

            Not(t) => {
                let f = env.get_term("not").expect("not in env");
                app1(f, d(*t))
            }
            Ite {
                cond,
                if_true,
                if_false,
            } => Term::Ite {
                cond: Box::new(d(*cond)),
                if_true: Box::new(d(*if_true)),
                if_false: Box::new(d(*if_false)),
            },
            Zero => Term::Zero,
            Succ(ast) => Term::Succ(Box::new(d(*ast))),
            Rec {
                scrutinee,
                if_zero,
                if_succ,
            } => Term::Rec {
                scrutinee: Box::new(d(*scrutinee)),
                if_zero: Box::new(d(*if_zero)),
                if_succ: Box::new(d(*if_succ)),
            },
            Nat(n) => nat(n),
            Add(t1, t2) => Term::Add(Box::new(d(*t1)), Box::new(d(*t2))),
            Sub(t1, t2) => Term::Sub(Box::new(d(*t1)), Box::new(d(*t2))),
            Mul(t1, t2) => Term::Mul(Box::new(d(*t1)), Box::new(d(*t2))),
            Eq(t1, t2) => {
                let f = env.get_term("eq").expect("eq in env");
                app2(f, d(*t1), d(*t2))
            }

            Neq(t1, t2) => {
                let f = env.get_term("neq").expect("neq in env");
                app2(f, d(*t1), d(*t2))
            }

            Le(t1, t2) => {
                let f = env.get_term("le").expect("le in env");
                app2(f, d(*t1), d(*t2))
            }

            Lt(t1, t2) => {
                let f = env.get_term("lt").expect("lt in env");
                app2(f, d(*t1), d(*t2))
            }

            Ge(t1, t2) => {
                let f = env.get_term("ge").expect("ge in env");
                app2(f, d(*t1), d(*t2))
            }

            Gt(t1, t2) => {
                let f = env.get_term("gt").expect("gt in env");
                app2(f, d(*t1), d(*t2))
            }
            Name(name) => {
                // CRITICAL: Direct lookup, NO recursion!
                env.get_term(&name)
                    .unwrap_or_else(|| Term::Var(name))
            },
            Pair(t1, t2) => Term::Pair(
                Box::new(d(*t1)),
                Box::new(d(*t2)),
            ),
            Fst(t) => Term::Fst(Box::new(d(*t))),
            Snd(t) => Term::Snd(Box::new(d(*t))),
            Nil => Term::Nil(None),
            TypedNil(ty) => Term::Nil(Some(ty)),

            Cons(h, t) => {
                Term::Cons(
                    Box::new(h.desugar(env)),
                    Box::new(t.desugar(env)),
                )
            }

            List(elements) => {
                elements.iter().rev().fold(Term::Nil(None), |tail, head| {
                    let head_term = head.clone().desugar(env);

                    // attach type hint to Nil if it’s currently None
                    let tail = match tail {
                        Term::Nil(None) => {
                            let mut ctx = crate::types::empty_ctx();
                            let ty = crate::types::type_of(&head_term, &mut ctx).ok();
                            Term::Nil(ty)
                        }
                        other => other,
                    };

                    Term::Cons(Box::new(head_term), Box::new(tail))
                })
            }

            Head(t) => Term::Head(Box::new(d(*t))),
            Tail(t) => Term::Tail(Box::new(d(*t))),
            IsEmpty(t) => Term::IsEmpty(Box::new(d(*t))),
        }
    }
}
