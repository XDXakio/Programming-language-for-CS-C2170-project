#[macro_export]

macro_rules! t_ty {
    (Nat) => {
        crate::types::Type::Nat
    };
    ($t1:tt -> $($t2:tt)+) => {
        crate::types::Type::Arrow(
            Box::new(t_ty!($t1)),
            Box::new(t_ty!($($t2)+)),
        )
    };
}

#[macro_export]
/// Build lambda terms using a token-tree muncher.
macro_rules! t {
    // Shorthand lambda
    ($var:ident : $($ty:tt)+ => $($body:tt)+) => {
        crate::term::Term::Abs {
            var: stringify!($var).to_string(),
            ty: t_ty!($($ty)+),
            body: Box::new(t!($($body)+)),
        }
    };

    // Embed existing Term expression
    (!( $term:expr )) => { $term };
    (! $term:ident) => { $term };

    // Application muncher: parse atoms left-to-right
    (@app [$($stack:expr),*] ! ( $term:expr ) $($rest:tt)*) => {
        t!(@app [$($stack,)* $term] $($rest)*)
    };
    (@app [$($stack:expr),*] ! $term:ident $($rest:tt)*) => {
        t!(@app [$($stack,)* $term] $($rest)*)
    };
    (@app [$($stack:expr),*] ( $($inner:tt)+ ) $($rest:tt)*) => {
        t!(@app [$($stack,)* t!($($inner)+)] $($rest)*)
    };
    (@app [$($stack:expr),*] $var:ident $($rest:tt)*) => {
        t!(@app [$($stack,)* crate::term::Term::Var(stringify!($var).to_string())] $($rest)*)
    };

    // Reduce stack into left-associative applications
    (@app [$single:expr]) => { $single };
    (@app [$head:expr, $next:expr $(, $tail:expr)*]) => {
        t!(@app [
            crate::term::Term::App(Box::new($head), Box::new($next))
            $(, $tail)*
        ])
    };

    ($($t:tt)+) => {
        t!(@app [] $($t)+)
    };
}
