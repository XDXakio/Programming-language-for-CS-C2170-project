#[macro_export]
/// Build lambda terms using a token-tree muncher.
macro_rules! t {
    // Abstraction has the lowest precedence and must appear at top level
    (fun $var:ident => $($body:tt)+) => {
        crate::term::Term::Abs {
            var: stringify!($var).to_string(),
            body: Box::new(t!($($body)+)),
        }
    };
    // Shorthand lambda
    ($var:ident => $($body:tt)+) => {
        crate::term::Term::Abs {
            var: stringify!($var).to_string(),
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
