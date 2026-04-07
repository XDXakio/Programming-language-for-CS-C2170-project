use programming_language::types::{self, Context, Type, TypeError, type_of};
use programming_language::term::Term;
use programming_language::ast::AST;
use programming_language::module::Module;

fn typecheck(ast: AST) -> Result<Type, String> {
    let module = Module::new_with_prelude();
    let term = ast.desugar(&module);
    let mut ctx = Context::new();

    type_of(&term, &mut ctx).map_err(|e| format!("{:?}", e))
}

#[test]
fn test_bool_type() {
    let term = Term::True;
    let mut ctx = Context::new();

    let ty = type_of(&term, &mut ctx).unwrap();
    assert_eq!(ty, Type::Bool);
}

#[test]
fn test_add_type_ok() {
    let ty = typecheck(AST::Add(
        Box::new(AST::Nat(1)),
        Box::new(AST::Nat(2)),
    )).unwrap();

    assert_eq!(ty, Type::Nat);
}

#[test]
fn test_add_type_error() {
    let result = typecheck(AST::Add(
        Box::new(AST::Nat(1)),
        Box::new(AST::True),
    ));

    assert!(result.is_err());
}

#[test]
fn test_mul_type_error() {
    let result = typecheck(AST::Mul(
        Box::new(AST::False),
        Box::new(AST::Nat(2)),
    ));

    assert!(result.is_err());
}

#[test]
fn test_eq_type_error() {
    let result = typecheck(AST::Eq(
        Box::new(AST::True),
        Box::new(AST::Nat(1)),
    ));

    assert!(result.is_err());
}

#[test]
fn pair_basic_type() {
    let mut ctx = Context::new();

    let term = Term::Pair(
        Box::new(Term::Zero),
        Box::new(Term::True),
    );

    let ty = type_of(&term, &mut ctx).unwrap();
    assert_eq!(ty.to_string(), "(Nat, Bool)");
}

#[test]
fn fst_type() {
    let mut ctx = Context::new();

    let term = Term::Fst(Box::new(
        Term::Pair(Box::new(Term::Zero), Box::new(Term::True))
    ));

    let ty = type_of(&term, &mut ctx).unwrap();
    assert_eq!(ty, Type::Nat);
}

#[test]
fn snd_type() {
    let mut ctx = Context::new();

    let term = Term::Snd(Box::new(
        Term::Pair(Box::new(Term::Zero), Box::new(Term::True))
    ));

    let ty = type_of(&term, &mut ctx).unwrap();
    assert_eq!(ty, Type::Bool);
}

#[test]
fn fst_type_error() {
    let mut ctx = Context::new();

    let term = Term::Fst(Box::new(Term::Zero));

    assert!(type_of(&term, &mut ctx).is_err());
}

#[test]
fn test_empty_list_type() {
    let term = Term::Nil(Some(Type::Nat)); // now carries type
    let mut ctx = types::empty_ctx();
    let ty = type_of(&term, &mut ctx).unwrap();
    assert_eq!(ty, Type::List(Box::new(Type::Nat)));
}

#[test]
fn test_single_element_list() {
    let term = Term::Cons(Box::new(Term::Zero), Box::new(Term::Nil(Some(Type::Nat))));
    let mut ctx = types::empty_ctx();
    let ty = type_of(&term, &mut ctx).unwrap();
    assert_eq!(ty, Type::List(Box::new(Type::Nat)));
}

#[test]
fn test_multiple_element_list() {
    let term = Term::Cons(
        Box::new(Term::Zero),
        Box::new(Term::Cons(
            Box::new(Term::Succ(Box::new(Term::Zero))),
            Box::new(Term::Nil(Some(Type::Nat))),
        )),
    );
    let mut ctx = types::empty_ctx();
    let ty = type_of(&term, &mut ctx).unwrap();
    assert_eq!(ty, Type::List(Box::new(Type::Nat)));
}

#[test]
fn test_nested_list() {
    let inner_list = Term::Cons(Box::new(Term::Zero), Box::new(Term::Nil(Some(Type::Nat))));
    let term = Term::Cons(Box::new(inner_list), Box::new(Term::Nil(Some(Type::List(Box::new(Type::Nat))))));
    let mut ctx = types::empty_ctx();
    let ty = type_of(&term, &mut ctx).unwrap();
    assert_eq!(ty, Type::List(Box::new(Type::List(Box::new(Type::Nat)))));
}

#[test]
fn test_list_type_mismatch() {
    let term = Term::Cons(
        Box::new(Term::Zero),
        Box::new(Term::Cons(Box::new(Term::True), Box::new(Term::Nil(Some(Type::Nat))))),
    );
    let mut ctx = types::empty_ctx();
    let err = type_of(&term, &mut ctx).unwrap_err();
    match err {
        TypeError::Mismatch { expected, found, .. } => {
            assert_eq!(expected, Type::Nat);
            assert_eq!(found, Type::Bool);
        }
        _ => panic!("Expected type mismatch error"),
    }
}