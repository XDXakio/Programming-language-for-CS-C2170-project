use programming_language::{ parser::{ parse_ast, parse_module },
    module::Module,
    term::nat,
    types::{type_of, Context, Type}, };

#[test]
fn test_full_pipeline() {
    let module = Module::new_with_prelude();

    let (_, ast) = parse_ast(&module, "1 + 2").unwrap();
    let term = ast.desugar(&module);

    let mut ctx = Context::new();
    let ty = type_of(&term, &mut ctx).unwrap();

    assert_eq!(ty, Type::Nat);

    let result = term.multistep();
    assert_eq!(result, nat(3));
}

#[test]
fn test_type_and_eval_pipeline() {
    let module = Module::new_with_prelude();

    let (_, ast) = parse_ast(&module, "1 + 2").unwrap();
    let term = ast.desugar(&module);

    let mut ctx = Context::new();
    let ty = type_of(&term, &mut ctx).unwrap();

    assert_eq!(ty, Type::Nat);

    let result = term.multistep();
    assert_eq!(result, programming_language::term::nat(3));
}

#[test]
fn test_module_declarations() {
    let input = "
    a = 1
    b = a
    ";

    let (_, module) = parse_module(input).unwrap();

    let term = module.get_term("b").unwrap();
    let result = term.multistep();

    assert_eq!(result, programming_language::term::nat(1));
}
