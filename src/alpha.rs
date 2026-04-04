use crate::term::Term;

use Term::*;

pub fn eq_var_alpha<'l, 'r>(l_ctx: &[&'l str], a: &'l str, r_ctx: &[&'r str], b: &'r str) -> bool {
    match (l_ctx.split_last(), r_ctx.split_last()) {
        (None, None) => a == b,
        (Some((x, l_rest)), Some((y, r_rest))) => {
            if *x == a && *y == b {
                true
            } else if *x != a && *y != b {
                eq_var_alpha(l_rest, a, r_rest, b)
            } else {
                false
            }
        }
        _ => false,
    }
}

pub fn eq_alpha<'l, 'r>(
    mut l_ctx: Vec<&'l str>,
    lhs: &'l Term,
    mut r_ctx: Vec<&'r str>,
    rhs: &'r Term,
) -> bool {
    match (lhs, rhs) {
        (Var(a), Var(b)) => eq_var_alpha(&l_ctx, a, &r_ctx, b),

        (App(l1, l2), App(r1, r2)) => {
            eq_alpha(l_ctx.clone(), l1, r_ctx.clone(), r1)
                && eq_alpha(l_ctx, l2, r_ctx, r2)
        }

        (Abs { var: la, ty: lty, body: lbody },
        Abs { var: ra, ty: rty, body: rbody }) => {
            if lty != rty {
                return false;
            }

            l_ctx.push(la.as_str());
            r_ctx.push(ra.as_str());
            eq_alpha(l_ctx, lbody, r_ctx, rbody)
        }

        (True, True) => true,
        (False, False) => true,

        (
        Ite { cond: lc, if_true: lt, if_false: lf },
        Ite { cond: rc, if_true: rt, if_false: rf },
        ) => {
            eq_alpha(l_ctx.clone(), lc, r_ctx.clone(), rc)
                && eq_alpha(l_ctx.clone(), lt, r_ctx.clone(), rt)
                && eq_alpha(l_ctx, lf, r_ctx, rf)
        }

        (Zero, Zero) => true,

        (Succ(t1), Succ(t2)) => eq_alpha(l_ctx, t1, r_ctx, t2),

        (Rec { scrutinee: s1, if_zero: z1, if_succ: f1 },
         Rec { scrutinee: s2, if_zero: z2, if_succ: f2 }) => {
            eq_alpha(l_ctx.clone(), s1, r_ctx.clone(), s2)
                && eq_alpha(l_ctx.clone(), z1, r_ctx.clone(), z2)
                && eq_alpha(l_ctx, f1, r_ctx, f2)
        }

        _ => false,
    }
}

impl PartialEq for Term {
    fn eq(&self, other: &Self) -> bool {
        eq_alpha(vec![], self, vec![], other)
    }
}
