use crate::{term::Term};

use Term::*;

impl Term {
    pub fn whnf(t: &Term) -> Term {
        match t {
            App(l, r) => {
                let l_whnf = Term::whnf(l);
                match l_whnf {
                    Abs { var, ty: _, body } => body.subst(&var, &*r.clone()),
                    other => App(Box::new(other), r.clone()),
                }
            }
            Var(v) => Var(v.clone()),
            Abs { var, ty, body } => Abs { var: var.clone(), ty: ty.clone(), body: body.clone() },
            Ite { cond, if_true, if_false } => {
                let c_whnf = Term::whnf(cond);
                match c_whnf {
                    True => *if_true.clone(),
                    False => *if_false.clone(),
                    other => Term::Ite {
                        cond: Box::new(other),
                        if_true: if_true.clone(),
                        if_false: if_false.clone(),
                    }
                }
            }
            True => True,
            False => False,
            Zero => Zero,
            Add(a, b) => Add(a.clone(), b.clone()),
            Sub(a, b) => Sub(a.clone(), b.clone()),
            Mul(a, b) => Mul(a.clone(), b.clone()),
            Succ(t1) => Succ(t1.clone()),
            Rec { scrutinee, if_zero, if_succ } => Rec {
                scrutinee: scrutinee.clone(),
                if_zero: if_zero.clone(),
                if_succ: if_succ.clone(),
            },
            Pair(t1, t2) => Pair(t1.clone(), t2.clone()),
            Fst(t) => {
                let t_whnf = Term::whnf(t);
                match t_whnf {
                    Pair(v1, _) => *v1,
                    other => Fst(Box::new(other)),
                }
            }
            Snd(t) => {
                let t_whnf = Term::whnf(t);
                match t_whnf {
                    Pair(_, v2) => *v2,
                    other => Snd(Box::new(other)),
                }
            }
            Nil(t) => Nil(t.clone()),
            Cons(h, t1) => Cons(h.clone(), t1.clone()),
            Head(t) => {
                let t_whnf = Term::whnf(t);
                match t_whnf {
                    Cons(h, _) => *h,
                    other => Head(Box::new(other)),
                }
            }

            Tail(t) => {
                let t_whnf = Term::whnf(t);
                match t_whnf {
                    Cons(_, tail) => *tail,
                    other => Tail(Box::new(other)),
                }
            }

            IsEmpty(t) => {
                let t_whnf = Term::whnf(t);
                match t_whnf {
                    Nil(_) => True,
                    Cons(_, _) => False,
                    other => IsEmpty(Box::new(other)),
                }
            }
        }
    }

    pub fn pair1(&self) -> Option<Self> {
        match self {
            Pair(t1, t2) => {
                if let Some(t1_step) = t1.step() {
                    Some(Pair(Box::new(t1_step), t2.clone()))
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    pub fn pair2(&self) -> Option<Self> {
        match self {
            Pair(t1, t2) => {
                if t1.step().is_none() {
                    t2.step().map(|t2_step| Pair(t1.clone(), Box::new(t2_step)))
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    pub fn fst(&self) -> Option<Self> {
        match self {
            Fst(t) => {
                if let Some(t_step) = t.step() {
                    return Some(Fst(Box::new(t_step)));
                }
                if let Pair(v1, _) = &**t {
                    return Some(*v1.clone());
                }
                None
            }
            _ => None,
        }
    }

    pub fn snd(&self) -> Option<Self> {
        match self {
            Snd(t) => {
                if let Some(t_step) = t.step() {
                    return Some(Snd(Box::new(t_step)));
                }
                if let Pair(_, v2) = &**t {
                    return Some(*v2.clone());
                }
                None
            }
            _ => None,
        }
    }

    pub fn head(&self) -> Option<Self> {
        match self {
            Term::Head(t) => {
                if let Some(t_step) = t.step() {
                    return Some(Term::Head(Box::new(t_step)));
                }

                match &**t {
                    Term::Cons(h, _) => Some(*h.clone()),
                    _ => None,
                }
            }
            _ => None,
        }
    }

    pub fn tail(&self) -> Option<Self> {
        match self {
            Term::Tail(t) => {
                // Step inside first
                if let Some(t_step) = t.step() {
                    return Some(Term::Tail(Box::new(t_step)));
                }

                // Apply rule
                match &**t {
                    Term::Cons(_, tail) => Some(*tail.clone()),
                    _ => None,
                }
            }
            _ => None,
        }
    }

    pub fn is_empty(&self) -> Option<Self> {
        match self {
            Term::IsEmpty(t) => {
                // Step inside first
                if let Some(t_step) = t.step() {
                    return Some(Term::IsEmpty(Box::new(t_step)));
                }

                // Apply rules
                match &**t {
                    Term::Nil(_) => Some(Term::True),
                    Term::Cons(_, _) => Some(Term::False),
                    _ => None,
                }
            }
            _ => None,
        }
    }

    /// Applies the `AppAbs` rule returning None if it doesn't apply.
    pub fn app_abs(&self) -> Option<Self> {
        match self {
            App(f, a) => {
                let f_whnf = Term::whnf(&*f);
                if let Abs { var, ty: _, body } = f_whnf {
                    Some(body.subst(&var, &*a.clone()))
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// Applies the `App1` rule returning None if it doesn't apply.
    pub fn app1(&self) -> Option<Self> {
        match self {
            App(f, a) => {
                if let Some(f_step) = f.step() {
                    Some(App(Box::new(f_step), a.clone()))
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// Applies the `App2` rule returning None if it doesn't apply.
    pub fn app2(&self) -> Option<Self> {
        match self {
            App(f, a) => {
                if f.step().is_none() {
                    a.step().map(|a_step| App(f.clone(), Box::new(a_step)))
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// Applies the `Abs` rule returning None if it doesn't apply.
    pub fn abs(&self) -> Option<Self> {
        match self {
            Abs { var, ty, body } => {
                body.step().map(|body_step| Abs { var: var.clone(), ty: ty.clone(), body: Box::new(body_step) })
            }
            _ => None,
        }
    }

    /// Applies the `Ite1` rule returning None if it doesn't apply.
    pub fn ite1(&self) -> Option<Self> {
        match self {
            Ite { cond, if_true, if_false } => {
                cond.step().map(|c_step| Ite { cond: Box::new(c_step), if_true: if_true.clone(), if_false: if_false.clone() })
            }
            _ => None,
        }
    }

    /// Applies `IteTrue` or `IteFalse` returning None if neither applies.
    pub fn ite(&self) -> Option<Self> {
        match self {
            Ite { cond, if_true, if_false } => {
                match Term::whnf(&*cond) {
                    True => Some(*if_true.clone()),
                    False => Some(*if_false.clone()),
                    _ => None,
                }
            }
            _ => None,
        }
    }

    /// Applies the `Succ1` rule returning None if it doesn't apply.
    pub fn succ1(&self) -> Option<Self> {
        match self { Succ(t) => t.step().map(|s| Succ(Box::new(s))), _ => None }
    }

    /// Applies the `Rec` rule returning None if it doesn't apply.
    pub fn rec(&self) -> Option<Self> {
        if let Rec { scrutinee, if_zero, if_succ } = self {
            if let Some(step) = scrutinee.step() {
                return Some(Rec {
                    scrutinee: Box::new(step),
                    if_zero: if_zero.clone(),
                    if_succ: if_succ.clone(),
                });
            }

            match Term::whnf(scrutinee) {
                Zero => Some(*if_zero.clone()),
                Succ(n) => Some(App(
                    Box::new(App(if_succ.clone(), n.clone())),
                    Box::new(Rec { scrutinee: n, if_zero: if_zero.clone(), if_succ: if_succ.clone() }),
                )),
                _ => None,
            }
        } else {
            None
        }
    }

    /// Does a beta-reduction step returning None if no reduction rule applies.
    /// Note: `AppAbs`, `Ite` and `Rec` and `App1` should come before the other rules.
    pub fn step(&self) -> Option<Self> {
        self.app_abs()
            .or_else(|| self.app1())
            .or_else(|| self.app2())
            .or_else(|| self.ite())
            .or_else(|| self.ite1())
            .or_else(|| self.rec())
            .or_else(|| self.fst())
            .or_else(|| self.snd())
            .or_else(|| self.head())
            .or_else(|| self.tail())
            .or_else(|| self.abs())
            .or_else(|| self.is_empty())
            .or_else(|| self.arith())
            .or_else(|| self.succ1())
            .or_else(|| self.abs())
    }

    /// Does any number of beta-reduction steps.
    /// Returns the final term for which no reduction could be made.
    pub fn multistep(mut self) -> Self {
        while let Some(next) = self.step() {
            self = next;
        }
        self
    }

    /// Compares if two normalizing terms are beta-equivalent.
    pub fn beta_eq(&self, other: &Self) -> bool {
        let n1 = self.clone().multistep();
        let n2 = other.clone().multistep();
        n1 == n2
    }
}
