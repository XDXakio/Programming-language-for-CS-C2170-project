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
        // Input with spaces and newlines
        let input = "a = 1\nb = a";

        // Parse the module
        let (_, module) = parse_module(input).expect("Failed to parse module");

        // Get the AST for 'b' and desugar into a Term
        let ast_b = module.get_term_ast("b").expect("Module should contain 'b'");
        let term_b = ast_b.clone().desugar(&module);

        // Evaluate the term fully
        let result = term_b.multistep();

        // Expect that 'b' evaluates to 1
        assert_eq!(result, programming_language::term::nat(1));
    }
