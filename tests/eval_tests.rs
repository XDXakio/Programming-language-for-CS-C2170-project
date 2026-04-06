use programming_language::ast::AST;
use programming_language::module::Module;
use programming_language::term::Term;

fn eval(ast: AST) -> Term {
    let module = Module::new_with_prelude();
    ast.desugar(&module).multistep()
}

#[test]
fn test_add_simple() {
    let result = eval(AST::Add(
        Box::new(AST::Nat(1)),
        Box::new(AST::Nat(2)),
    ));
    assert_eq!(result.to_string(), "3");
}

#[test]
fn test_add_not_working_yet() {
    let result = eval(AST::Add(
        Box::new(AST::Nat(2)),
        Box::new(AST::Nat(3)),
    ));
    // This will likely FAIL before you fix arithmetic
    assert_eq!(result.to_string(), "5");
}

#[test]
fn test_sub_simple() {
    let result = eval(AST::Sub(
        Box::new(AST::Nat(5)),
        Box::new(AST::Nat(3)),
    ));
    assert_eq!(result.to_string(), "2");
}

#[test]
fn test_mul_simple() {
    let result = eval(AST::Mul(
        Box::new(AST::Nat(2)),
        Box::new(AST::Nat(3)),
    ));
    assert_eq!(result.to_string(), "6");
}

#[test]
fn test_eq_true() {
    let result = eval(AST::Eq(
        Box::new(AST::Nat(2)),
        Box::new(AST::Nat(2)),
    ));
    assert_eq!(result.to_string(), "true");
}

#[test]
fn test_eq_false() {
    let result = eval(AST::Eq(
        Box::new(AST::Nat(2)),
        Box::new(AST::Nat(3)),
    ));
    assert_eq!(result.to_string(), "false");
}

#[test]
fn test_neq_true() {
    let result = eval(AST::Neq(
        Box::new(AST::Nat(2)),
        Box::new(AST::Nat(3)),
    ));
    assert_eq!(result.to_string(), "true");
}

#[test]
fn test_neq_false() {
    let result = eval(AST::Neq(
        Box::new(AST::Nat(2)),
        Box::new(AST::Nat(2)),
    ));
    assert_eq!(result.to_string(), "false");
}

#[test]
fn test_le() {
    let result = eval(AST::Le(
        Box::new(AST::Nat(2)),
        Box::new(AST::Nat(3)),
    ));
    assert_eq!(result.to_string(), "true");
}

#[test]
fn test_lt() {
    let result = eval(AST::Lt(
        Box::new(AST::Nat(2)),
        Box::new(AST::Nat(3)),
    ));
    assert_eq!(result.to_string(), "true");
}

#[test]
fn test_ge() {
    let result = eval(AST::Ge(
        Box::new(AST::Nat(3)),
        Box::new(AST::Nat(2)),
    ));
    assert_eq!(result.to_string(), "true");
}

#[test]
fn test_gt() {
    let result = eval(AST::Gt(
        Box::new(AST::Nat(3)),
        Box::new(AST::Nat(2)),
    ));
    assert_eq!(result.to_string(), "true");
}

#[test]
fn eval_pair_no_reduce() {
    let term = Term::Pair(
        Box::new(Term::Zero),
        Box::new(Term::True),
    );

    let result = term.clone().multistep();
    assert_eq!(result, term);
}

#[test]
fn eval_fst() {
    let term = Term::Fst(Box::new(
        Term::Pair(Box::new(Term::Zero), Box::new(Term::True))
    ));

    let result = term.multistep();
    assert_eq!(result, Term::Zero);
}

#[test]
fn eval_snd() {
    let term = Term::Snd(Box::new(
        Term::Pair(Box::new(Term::Zero), Box::new(Term::True))
    ));

    let result = term.multistep();
    assert_eq!(result, Term::True);
}

#[test]
fn eval_nested_pair() {
    let term = Term::Fst(Box::new(
        Term::Pair(
            Box::new(Term::Pair(Box::new(Term::Zero), Box::new(Term::True))),
            Box::new(Term::True),
        )
    ));

    let result = term.multistep();

    assert_eq!(
        result,
        Term::Pair(Box::new(Term::Zero), Box::new(Term::True))
    );
}