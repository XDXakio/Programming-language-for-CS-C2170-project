use std::collections::HashSet;

#[derive(Debug, Clone)]
/// A term of the untyped lambda calculus with booleans
pub enum Term {
    // Core lambda calculus
    Var(String),
    Abs {
        var: String,
        ty: crate::types::Type,
        body: Box<Term>,
    },
    App(Box<Term>, Box<Term>),

    // Boolean extension
    True,
    False,
    Ite {
        cond: Box<Term>,
        if_true: Box<Term>,
        if_false: Box<Term>,
    },

    // Natural number extension
    Zero,
    Succ(Box<Term>),
    /// Recursor for natural numbers. Takes in the scrutinee, and two branches: the zero case and the successor case.
    /// `rec : forall T, Nat -> T -> (Nat -> T -> T) -> T`
    ///
    /// If the scrutinee is `zero`, reduces to the zero case.
    /// If the scrutinee is `succ(n)`, reduces to the successor case applied to `n` and the "induction hypothesis" `rec n if_zero if_succ`.
    Rec {
        scrutinee: Box<Term>,
        if_zero: Box<Term>,
        if_succ: Box<Term>,
    },
}

/// Return a variable name which is not in `vars` and starts with `base`
fn fresh_var<'a>(vars: &'_ HashSet<&'a str>, mut base: String) -> String {
    while vars.contains(base.as_str()) {
        base.push('_');
    }
    base
}

use Term::*;

/// Encodes an integer to a natural number term
pub fn nat(mut n: u64) -> Term {
    let mut t = Zero;
    while n > 0 {
        t = Succ(Box::new(t));
        n -= 1;
    }
    t
}

impl Term {
    pub fn free_vars(&self) -> HashSet<&str> {
        fn go<'a>(t: &'a Term, out: &mut HashSet<&'a str>) {
            match t {
                Var(v) => { out.insert(v); }
                Abs { var, ty: _, body } => {
                    go(body, out);
                    out.remove(var.as_str());
                }
                App(l, r) => { go(l, out); go(r, out); }
                Ite { cond, if_true, if_false } => {
                    go(cond, out); go(if_true, out); go(if_false, out);
                }
                True | False | Zero => {}
                Succ(t) => go(t, out),
                Rec { scrutinee, if_zero, if_succ } => {
                    go(scrutinee, out); go(if_zero, out); go(if_succ, out);
                }
            }
        }

        let mut s = HashSet::new();
        go(self, &mut s);
        s
    }

    pub fn rename(&mut self, var: &str, new: &str) {
        match self {
            Var(v) => { if v == var { *v = new.to_string(); } }
            Abs { var: b, ty: _, body } => {
                if b == var { *b = new.to_string(); }
                body.rename(var, new);
            }
            App(l, r) => { l.rename(var, new); r.rename(var, new); }
            Ite { cond, if_true, if_false } => {
                cond.rename(var, new); if_true.rename(var, new); if_false.rename(var, new);
            }
            Succ(t) => t.rename(var, new),
            Rec { scrutinee, if_zero, if_succ } => {
                scrutinee.rename(var, new); if_zero.rename(var, new); if_succ.rename(var, new);
            }
            True | False | Zero => {}
        }
    }

    /// Performs capture-avoiding substitution, i.e. replaces every free occurrence of `var` with `value` in `self` such that no free variable gets bound by an abstraction in `self`.
    pub fn subst(self, var: &str, value: &Term) -> Term {
        match self {
            Var(v) => if v == var { value.clone() } else { Var(v) },
            App(l, r) => App(Box::new(l.subst(var, value)), Box::new(r.subst(var, value))),
            Abs { var: b, ty, body } => {
                if b == var {
                    Abs { var: b, ty, body } // binder shadows var
                } else {
                    let fv_value = value.free_vars();
                    if fv_value.contains(b.as_str()) {
                        let mut used: HashSet<&str> = body.free_vars();
                        used.extend(fv_value.iter().copied());
                        used.insert(b.as_str()); used.insert(var);
                        let fresh = fresh_var(&used, b.clone());
                        let mut body_owned = *body.clone();
                        body_owned.rename(b.as_str(), &fresh);
                        Abs {
                            var: fresh,
                            ty,
                            body: Box::new(body_owned.subst(var, value)),
                        }
                    } else {
                        Abs { var: b, ty, body: Box::new(body.subst(var, value)) }
                    }
                }
            }
            Ite { cond, if_true, if_false } => Ite {
                cond: Box::new(cond.subst(var, value)),
                if_true: Box::new(if_true.subst(var, value)),
                if_false: Box::new(if_false.subst(var, value)),
            },
            Succ(t) => Succ(Box::new(t.subst(var, value))),
            Rec { scrutinee, if_zero, if_succ } => Rec {
                scrutinee: Box::new(scrutinee.subst(var, value)),
                if_zero: Box::new(if_zero.subst(var, value)),
                if_succ: Box::new(if_succ.subst(var, value)),
            },
            True => True,
            False => False,
            Zero => Zero,
        }
    }
}
