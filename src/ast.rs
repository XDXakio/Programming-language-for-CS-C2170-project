use crate::{
    module::Module,
    t,
    term::{Term, nat}, types::Type,
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

pub fn and() -> Term {
    Term::Abs {
        var: "b1".to_string(),
        ty: Type::Bool,
        body: Box::new(Term::Abs {
            var: "b2".to_string(),
            ty: Type::Bool,
            body: Box::new(Term::Ite {
                cond: Box::new(Term::Var("b1".to_string())),
                if_true: Box::new(Term::Var("b2".to_string())),
                if_false: Box::new(Term::False),
            }),
        }),
    }
}

pub fn not() -> Term {
    Term::Abs {
        var: "a".to_string(),
        ty: Type::Bool,
        body: Box::new(Term::Ite {
            cond: Box::new(Term::Var("a".to_string())),
            if_true: Box::new(Term::False),
            if_false: Box::new(Term::True),
        }),
    }
}

pub fn or() -> Term {
    Term::Abs {
        var: "b1".to_string(),
        ty: Type::Bool,
        body: Box::new(Term::Abs {
            var: "b2".to_string(),
            ty: Type::Bool,
            body: Box::new(Term::Ite {
                cond: Box::new(Term::Var("b1".to_string())),
                if_true: Box::new(Term::True),
                if_false: Box::new(Term::Var("b2".to_string())),
            }),
        }),
    }
}

/// The predecessor function for natural numbers
pub fn pred() -> Term {
    Term::Abs {
        var: "n".to_string(),
        ty: Type::Nat,
        body: Box::new(Term::Rec {
            scrutinee: Box::new(Term::Var("n".to_string())),
            if_zero: Box::new(Term::Zero),
            if_succ: Box::new(Term::Abs {
                var: "pred".to_string(),
                ty: Type::Nat,
                body: Box::new(Term::Abs {
                    var: "ih".to_string(),
                    ty: Type::Nat,
                    body: Box::new(Term::Var("pred".to_string())),
                }),
            }),
        }),
    }
}

pub fn plus() -> Term {
    let n = Term::Var("n".to_string());
    let m = Term::Var("m".to_string());

    Term::Abs {
        var: "n".to_string(),
        ty: Type::Nat,
        body: Box::new(Term::Abs {
            var: "m".to_string(),
            ty: Type::Nat,
            body: Box::new(Term::Rec {
                scrutinee: Box::new(n),
                if_zero: Box::new(m),
                if_succ: Box::new(Term::Abs {
                    var: "pred".to_string(),
                    ty: Type::Nat,
                    body: Box::new(Term::Abs {
                        var: "ih".to_string(),
                        ty: Type::Nat,
                        body: Box::new(Term::Succ(Box::new(Term::Var("ih".to_string())))),
                    }),
                }),
            }),
        }),
    }
}

pub fn mult() -> Term {
    Term::Abs {
        var: "n".to_string(),
        ty: Type::Nat,
        body: Box::new(Term::Abs {
            var: "m".to_string(),
            ty: Type::Nat,
            body: Box::new(Term::Rec {
                scrutinee: Box::new(Term::Var("n".to_string())),
                if_zero: Box::new(Term::Zero),
                if_succ: Box::new(Term::Abs {
                    var: "pred".to_string(),
                    ty: Type::Nat,
                    body: Box::new(Term::Abs {
                        var: "ih".to_string(),
                        ty: Type::Nat,
                        body: Box::new(Term::App(
                            Box::new(Term::App(Box::new(plus()), Box::new(Term::Var("m".to_string())))),
                            Box::new(Term::Var("ih".to_string())),
                        )),
                    }),
                }),
            }),
        }),
    }
}

pub fn minus() -> Term {
    Term::Abs {
        var: "n".to_string(),
        ty: Type::Nat,
        body: Box::new(Term::Abs {
            var: "m".to_string(),
            ty: Type::Nat,
            body: Box::new(Term::Rec {
                scrutinee: Box::new(Term::Var("m".to_string())),
                if_zero: Box::new(Term::Var("n".to_string())),
                if_succ: Box::new(Term::Abs {
                    var: "pred".to_string(),
                    ty: Type::Nat,
                    body: Box::new(Term::Abs {
                        var: "ih".to_string(),
                        ty: Type::Nat,
                        body: Box::new(Term::Var("ih".to_string())),
                    }),
                }),
            }),
        }),
    }
}

pub fn is_zero() -> Term {
    Term::Abs {
        var: "n".to_string(),
        ty: Type::Nat,
        body: Box::new(Term::Rec {
            scrutinee: Box::new(Term::Var("n".to_string())),
            if_zero: Box::new(Term::True),
            if_succ: Box::new(Term::Abs {
                var: "pred".to_string(),
                ty: Type::Nat,
                body: Box::new(Term::Abs {
                    var: "ih".to_string(),
                    ty: Type::Nat,
                    body: Box::new(Term::False),
                }),
            }),
        }),
    }
}

pub fn eq() -> Term {
    Term::Abs {
        var: "n".to_string(),
        ty: Type::Nat,
        body: Box::new(Term::Abs {
            var: "m".to_string(),
            ty: Type::Nat,
            body: Box::new(Term::App(
                Box::new(Term::App(Box::new(and()), 
                    Box::new(Term::App(Box::new(is_zero()), Box::new(Term::App(Box::new(minus()), Box::new(Term::Var("n".to_string())))))))),
                Box::new(Term::App(Box::new(is_zero()), Box::new(Term::App(Box::new(minus()), Box::new(Term::Var("m".to_string()))))))
            )),
        }),
    }
}

pub fn neq() -> Term {
    Term::Abs {
        var: "n".to_string(),
        ty: Type::Nat,
        body: Box::new(Term::Abs {
            var: "m".to_string(),
            ty: Type::Nat,
            body: Box::new(Term::App(
                Box::new(not()),
                Box::new(Term::App(Box::new(eq()), Box::new(Term::Var("n".to_string())))),
            )),
        }),
    }
}

pub fn le() -> Term {
    Term::Abs {
        var: "n".to_string(),
        ty: Type::Nat,
        body: Box::new(Term::Abs {
            var: "m".to_string(),
            ty: Type::Nat,
            body: Box::new(Term::App(
                Box::new(is_zero()),
                Box::new(Term::App(Box::new(minus()), Box::new(Term::Var("n".to_string())))),
            )),
        }),
    }
}

pub fn lt() -> Term {
    // Note: must use Succ rather than pred as 0 is not < 0
    Term::Abs {
        var: "n".to_string(),
        ty: Type::Nat,
        body: Box::new(Term::Abs {
            var: "m".to_string(),
            ty: Type::Nat,
            body: Box::new(Term::App(
                Box::new(le()),
                Box::new(Term::App(
                    Box::new(Term::Succ(Box::new(Term::Var("n".to_string())))),
                    Box::new(Term::Var("m".to_string())),
                )),
            )),
        }),
    }
}

pub fn ge() -> Term {
    Term::Abs {
        var: "n".to_string(),
        ty: Type::Nat,
        body: Box::new(Term::Abs {
            var: "m".to_string(),
            ty: Type::Nat,
            body: Box::new(Term::App(Box::new(le()), Box::new(Term::Var("m".to_string())))),
        }),
    }
}

pub fn gt() -> Term {
    Term::Abs {
        var: "n".to_string(),
        ty: Type::Nat,
        body: Box::new(Term::Abs {
            var: "m".to_string(),
            ty: Type::Nat,
            body: Box::new(Term::App(Box::new(lt()), Box::new(Term::Var("m".to_string())))),
        }),
    }
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
            And(t1, t2) => t!(!(and()) !(d(*t1)) !(d(*t2))),
            Or(t1, t2) => t!(!(or()) !(d(*t1)) !(d(*t2))),
            Not(t) => t!(!(not()) !(d(*t))),
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
            Add(t1, t2) => t!(!(plus()) !(d(*t1)) !(d(*t2))),
            Sub(t1, t2) => t!(!(minus()) !(d(*t1)) !(d(*t2))),
            Mul(t1, t2) => t!(!(mult()) !(d(*t1)) !(d(*t2))),
            Le(t1, t2) => t!(!(le()) !(d(*t1)) !(d(*t2))),
            Eq(t1, t2) => t!(!(eq()) !(d(*t1)) !(d(*t2))),
            Lt(t1, t2) => t!(!(lt()) !(d(*t1)) !(d(*t2))),
            Neq(t1, t2) => t!(!(neq()) !(d(*t1)) !(d(*t2))),
            Ge(t1, t2) => t!(!(ge()) !(d(*t1)) !(d(*t2))),
            Gt(t1, t2) => t!(!(gt()) !(d(*t1)) !(d(*t2))),
            Name(name) => env.get_term(&name).expect("env to contain name"),
        }
    }
}
