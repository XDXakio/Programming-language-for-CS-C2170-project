use crate::{ast::AST, term::Term, types::Type};

use AST::*;

#[derive(Debug, Clone, Default)]
/// A module containing declarations
pub struct Module { asts: Vec<(String, AST)>, terms: std::collections::HashMap<String, Term>, }

fn succ() -> AST {
    Abs {
        var: "n".to_string(),
        ty: Type::Nat,
        body: Box::new(Succ(Box::new(Var("n".to_string())))),
    }
}

fn rec() -> AST {
    Abs {
        var: "n".to_string(),
        ty: Type::Nat,
        body: Box::new(Abs {
            var: "z".to_string(),
            ty: Type::Nat,
            body: Box::new(Abs {
                var: "s".to_string(),
                ty: Type::Func(
                    Box::new(Type::Nat),
                    Box::new(Type::Func(Box::new(Type::Nat), Box::new(Type::Nat))),
                ),
                body: Box::new(Rec {
                    scrutinee: Box::new(Var("n".to_string())),
                    if_zero: Box::new(Var("z".to_string())),
                    if_succ: Box::new(App(
                        Box::new(Var("s".to_string())),
                        Box::new(Var("z".to_string())),
                    )),
                }),
            }),
        }),
    }
}

impl Module {
    pub fn get_term_ast(&self, name: &str) -> Option<&AST> {
        self.asts.iter().rfind(|(n, _)| n == name).map(|(_, ast)| ast)
    }

    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, name: String, ast: AST) {
        self.asts.push((name, ast));
    }

    pub fn iter(&self) -> impl Iterator<Item = &(String, AST)> {
        self.asts.iter()
    }

    pub fn new_with_prelude() -> Self {
        use crate::ast;
        let mut m = Self::new();
        
        // List ALL prelude ASTs
        let prelude_asts = [
            ("succ", succ()),
            ("rec", rec()),
            ("pred", ast::pred()),
            ("is_zero", ast::is_zero()),
            ("and", ast::and()),
            ("or", ast::or()),
            ("not", ast::not()),
            ("eq", ast::eq()),
            ("neq", ast::neq()),
            ("lt", ast::lt()),
            ("le", ast::le()),
            ("gt", ast::gt()),
            ("ge", ast::ge()),
        ];
        
        // Convert ALL to Term ONCE at startup (safe, empty module)
        for (name, ast) in prelude_asts {
            let term = ast.clone().desugar(&m);  // No recursion risk - empty module
            m.terms.insert(name.to_string(), term);
            m.asts.push((name.to_string(), ast));
        }
        
        m
    }

    pub fn contains(&self, name: &str) -> bool {
        self.asts.iter().rfind(|(n, _ast)| n == name).is_some()
    }

    pub fn get(&self, name: &str) -> Option<&AST> {
        self.asts
            .iter()
            .rfind(|(n, _ast)| n == name)
            .map(|(_n, ast)| ast)
    }

    pub fn get_term(&self, name: &str) -> Option<Term> {
        self.terms.get(name)
            .cloned()
            .or_else(|| {
                self.get(name)
                    .map(|ast| ast.clone().desugar(self))
            })
    }
}
