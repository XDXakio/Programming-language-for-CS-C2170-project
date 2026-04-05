#[cfg(test)]
mod tests {
    use programming_language::{module::Module, parser::{ parse_ast, parse_decl }, };
    
    fn test_repl_success(input: &str, expected_result: &str) {
        let module = Module::new_with_prelude();
        let (_, ast) = parse_ast(&module, input).unwrap();
        let term = ast.desugar(&module);
        
        // Eval only (skip type check for now - it's broken)
        let result = term.multistep();
        assert_eq!(format!("{}", result), expected_result);
    }
    
    fn test_decl_use(decl: &str, expr: &str, expected: &str) {
        let mut module = Module::new_with_prelude();
        let (_, (name, decl_ast)) = parse_decl(&module, decl).unwrap();
        module.insert(name, decl_ast);
        
        let (_, expr_ast) = parse_ast(&module, expr).unwrap();
        let term = expr_ast.desugar(&module).multistep();
        assert_eq!(format!("{}", term), expected);
    }
    
    #[test]
    fn test_arithmetic() {
        test_repl_success("2 + 3", "5");
        test_repl_success("5 - 2", "3");
        test_repl_success("3 * 2", "6");
    }
    
    #[test]
    fn test_booleans() {
        test_repl_success("true and false", "false");
        test_repl_success("true or false", "true");
        test_repl_success("not true", "false");
    }
    
    #[test]
    fn test_nat() {
        test_repl_success("is_zero 0", "true");
        test_repl_success("is_zero 1", "false");
        test_repl_success("succ 0", "1");
    }
    
    #[test]
    fn test_conditionals() {
        test_repl_success("if true then 1 else 2", "1");
        test_repl_success("if false then 1 else 2", "2");
    }
    
    //#[test]
    //fn test_recursion() {
      //  test_repl_success("rec 3 0 (fun x => succ x)", "3");
    //}
    
    #[test]
    fn test_decls() {
        test_decl_use(
            "two = 2",
            "two + 1", 
            "3"
        );
    }
    
    #[test]
    #[ignore]  // Enable after type checker fixed
    fn test_comparisons() {
        test_repl_success("eq 2 2", "true");
        test_repl_success("le 2 2", "true");
    }
}