use nom::{Parser, combinator::all_consuming};

use programming_language::{
    ast::AST,
    module::Module,
    parser::parse_ast,
};
use AST::*;

#[test]
fn test_parse_conditional() {
    // `Ite(Lt(2, 5), And(Gt(5, 2), True), False)`
    let expected = Ite {
        cond: Box::new(Lt(Box::new(Nat(2)), Box::new(Nat(5)))),
        if_true: Box::new(And(
            Box::new(Gt(Box::new(Nat(5)), Box::new(Nat(2)))),
            Box::new(True),
        )),
        if_false: Box::new(False),
    };
    let module = Module::new_with_prelude();
    assert_eq!(
        all_consuming(|i| parse_ast(&module, i))
            .parse("if 2 < 5 then (5 > 2) and true else false"),
        Ok(("", expected))
    );
}

#[test]
fn test_parse_ast() {
    use programming_language::types::Type;
    // `Sub(Add(1, Mul(App(b, c), 5)), App(App(App(Abs(x, x), d), e), f))`
    let expected = Sub(
        Box::new(Add(
            Box::new(Nat(1)),
            Box::new(Mul(
                Box::new(App(
                    Box::new(Var("b".to_string())),
                    Box::new(Var("c".to_string())),
                )),
                Box::new(Nat(5)),
            )),
        )),
        Box::new(App(
            Box::new(App(
                Box::new(App(
                    Box::new(Abs {
                        var: "x".to_string(),
                        ty: Type::Nat,
                        body: Box::new(Var("x".to_string())),
                    }),
                    Box::new(Var("d".to_string())),
                )),
                Box::new(Var("e".to_string())),
            )),
            Box::new(Var("f".to_string())),
        )),
    );
    let module = Module::new_with_prelude();
    assert_eq!(
        all_consuming(|i| parse_ast(&module, i)).parse("1 + b c * 5 - (fun x => x) d e f"),
        Ok(("", expected))
    );
}

#[test]
fn test_parse_not_and_or_precedence() {
    let expected = Or(
        Box::new(Not(Box::new(True))),
        Box::new(And(Box::new(False), Box::new(True))),
    );
    let module = Module::new_with_prelude();
    assert_eq!(
        all_consuming(|i| parse_ast(&module, i)).parse("not true or false and true"),
        Ok(("", expected))
    );
}

#[test]
fn test_parse_arithmetic_precedence() {
    let expected = Eq(
        Box::new(Add(
            Box::new(Nat(1)),
            Box::new(Mul(Box::new(Nat(2)), Box::new(Nat(3)))),
        )),
        Box::new(Nat(7)),
    );
    let module = Module::new_with_prelude();
    assert_eq!(
        all_consuming(|i| parse_ast(&module, i)).parse("1 + 2 * 3 == 7"),
        Ok(("", expected))
    );
}

#[test]
fn test_parse_parens_override_precedence() {
    let expected = Mul(
        Box::new(Add(Box::new(Nat(1)), Box::new(Nat(2)))),
        Box::new(Nat(3)),
    );
    let module = Module::new_with_prelude();
    assert_eq!(
        all_consuming(|i| parse_ast(&module, i)).parse("(1 + 2) * 3"),
        Ok(("", expected))
    );
}

#[test]
fn test_parse_lambda_forms() {
    use programming_language::ast::{AST::*};
    use programming_language::types::Type;
    use programming_language::module::Module;
    use programming_language::parser::parse_ast;
    use nom::combinator::all_consuming;

    // Expected AST for "fun x => x" or "x => x"
    let expected = Abs {
        var: "x".to_string(),
        ty: Type::Nat, // or whatever default type your parser assigns
        body: Box::new(Var("x".to_string())),
    };

    let module = Module::new_with_prelude();

    // Parse typed lambda (fun x => x)
    assert_eq!(
        all_consuming(|i| parse_ast(&module, i)).parse("fun x => x"),
        Ok(("", expected.clone()))
    );

    // Parse shorthand lambda (x => x)
    assert_eq!(
        all_consuming(|i| parse_ast(&module, i)).parse("x => x"),
        Ok(("", expected))
    );
}

#[test]
fn test_parse_primitives_partial_application() {
    let expected = App(Box::new(Name("succ".to_string())), Box::new(Nat(1)));
    let module = Module::new_with_prelude();
    assert_eq!(
        all_consuming(|i| parse_ast(&module, i)).parse("succ 1"),
        Ok(("", expected))
    );

    let expected_rec = App(Box::new(Name("rec".to_string())), Box::new(Nat(0)));
    assert_eq!(
        all_consuming(|i| parse_ast(&module, i)).parse("rec 0"),
        Ok(("", expected_rec))
    );
}

#[test]
fn test_parse_comparisons() {
    let expected = And(
        Box::new(Le(Box::new(Nat(1)), Box::new(Nat(2)))),
        Box::new(Neq(Box::new(Nat(2)), Box::new(Nat(3)))),
    );
    let module = Module::new_with_prelude();
    assert_eq!(
        all_consuming(|i| parse_ast(&module, i)).parse("1 <= 2 and 2 != 3"),
        Ok(("", expected))
    );
}

#[test]
fn test_reject_module_names_as_lambda_vars() {
    let mut module = Module::new_with_prelude();
    module.insert("a".to_string(), Nat(1));
    assert!(
        all_consuming(|i| parse_ast(&module, i))
            .parse("a => a")
            .is_err()
    );
}

/*#[test]
fn test_parse_module() {
    let (_, m) = all_consuming(parse_module)
        .parse(
            "a = x => x
b = y => 1
c = a b",
        )
        .unwrap();
    assert_eq!(m.get_term("a"), Some(t!(x => x)));
    assert_eq!(m.get_term("c"), Some(t!((x => x) (y => !(nat(1))))));
    assert_eq!(m.get_term("d"), None);
}
*/