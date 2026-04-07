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