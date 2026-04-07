use programming_language::parser::parse_ast;
use programming_language::module::Module;
use programming_language::ast::AST;
use programming_language::term::Term;
use programming_language::types::Type;
use AST::*;

fn parse_str(input: &str) -> AST {
    let module = Module::new_with_prelude();
    let (rest, ast) = parse_ast(&module, input).expect("parse failed");
    assert!(rest.trim().is_empty(), "trailing input: '{}'", rest);
    ast
}

#[test]
fn test_simple_lambda() {
    let ast = parse_str("(fun x: Nat => succ(x)) 0");
    let term = ast.desugar(&Module::new_with_prelude());
    let whnf = Term::whnf(&term).multistep();

    match whnf {
        Term::Succ(inner) => {
            match &*inner {  // inner is Box<Term>, deref once
                Term::Zero => (), // test passes
                _ => panic!("Expected Zero inside Succ"),
            }
        }
        _ => panic!("Expected Succ"),
    }
}

#[test]
fn test_lambda_application() {
    let ast = parse_str("(fun x: Nat => succ(x)) 0");
    let term = ast.desugar(&Module::new_with_prelude());
    let whnf = Term::whnf(&term).multistep();
    // S(0) in Term form
    assert!(matches!(whnf, Term::Succ(inner) if matches!(*inner, Term::Zero)));
}

#[test]
fn test_nested_lambda_application() {
    let ast = parse_str("(fun x: Nat => fun y: Nat => x) 0 1");
    let term = ast.desugar(&Module::new_with_prelude());
    let whnf = Term::whnf(&term);
    // Should reduce to 0
    assert!(matches!(whnf, Term::Zero));
}

#[test]
fn test_lambda_type_annotation_parsing() {
    let ast = parse_str("fun y: Bool => y");
    if let Abs { var, ty, .. } = ast {
        assert_eq!(var, "y");
        assert_eq!(ty, Type::Bool);
    } else {
        panic!("Expected AST::Abs");
    }
}

#[test]
fn test_simple_typed_lambda() {
    let module = Module::new_with_prelude();
    let input = "fun x: Nat => x";
    let (_, ast) = parse_ast(&module, input).expect("Failed to parse");
    let term = ast.desugar(&module);
    let ty = programming_language::types::type_of(&term, &mut programming_language::types::empty_ctx()).expect("Type error");
    assert_eq!(ty, Type::Func(Box::new(Type::Nat), Box::new(Type::Nat)));
}

#[test]
fn test_lambda_application_2() {
    let module = Module::new_with_prelude();
    let input = "(fun x: Nat => succ x) 0";
    let (_, ast) = parse_ast(&module, input).expect("Failed to parse");
    let term = ast.desugar(&module);
    let whnf = Term::whnf(&term).multistep();
    // Expect Succ(Zero)
    match whnf {
        Term::Succ(inner) => assert!(matches!(*inner, Term::Zero)),
        _ => panic!("Expected Succ(Zero), got {:?}", whnf),
    }
}

#[test]
fn test_list_in_lambda() {
    let module = Module::new_with_prelude();
    let input = "(fun x: Nat => [x]) 0";
    let (_, ast) = parse_ast(&module, input).expect("Failed to parse");
    let term = ast.desugar(&module);
    let whnf = Term::whnf(&term);
    // Expect a singleton list: Cons(Zero, Nil(Some(Nat)))
    match whnf {
        Term::Cons(head, tail) => {
            assert!(matches!(*head, Term::Zero));
            match *tail {
                Term::Nil(Some(Type::Nat)) => {},
                _ => panic!("Expected typed Nil(Nat), got {:?}", tail),
            }
        },
        _ => panic!("Expected singleton list, got {:?}", whnf),
    }
}

#[test]
fn test_list_head_lambda() {
    let module = Module::new_with_prelude();
    let input = "(fun l: List(Nat) => head l) [succ 0, 0]";
    let (_, ast) = parse_ast(&module, input).expect("Failed to parse");
    let term = ast.desugar(&module);
    let whnf = Term::whnf(&term).multistep();
    // Expect first element: Succ(Zero)
    match whnf {
        Term::Succ(inner) => assert!(matches!(*inner, Term::Zero)),
        _ => panic!("Expected Succ(Zero), got {:?}", whnf),
    }
}