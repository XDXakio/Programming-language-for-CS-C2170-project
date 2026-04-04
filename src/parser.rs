use nom::IResult;
use nom::Parser;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_while1};
use nom::character::complete::{digit1, line_ending, space0, space1};

use nom::combinator::eof;
use nom::combinator::{fail, map_res, opt, value};
use nom::error::{Error, ErrorKind};
use nom::multi::fold_many0;
use nom::sequence::{delimited, preceded, terminated};
use nom_language::precedence::{Assoc, Operation, binary_op, precedence, unary_op};

use crate::ast::AST;
use crate::module::Module;

use AST::*;

/// Returns true if a character is valid in a variable name.
/// Used by `parse_variable_name`.
pub fn variable_pred(c: char) -> bool {
    c.is_alphanumeric() || c == '_' || c == '\''
}

/// Parses a raw variable identifier (no keyword filtering).
pub fn parse_variable_name(input: &str) -> IResult<&str, &str> {
    take_while1(variable_pred).parse(input)
}

/// Returns true if the identifier is reserved and cannot be parsed as a variable.
pub fn is_reserved(name: &str) -> bool {
    matches!(
        name,
        "fun" | "if" | "then" | "else" | "true" | "false" | "and" | "or" | "not" | "S" | "Z"
    )
}

/// Wraps a parser to consume optional surrounding whitespace.
pub fn lex<'a, O, P>(parser: P) -> impl Parser<&'a str, Output = O, Error = Error<&'a str>>
where
    P: Parser<&'a str, Output = O, Error = Error<&'a str>>,
{
    delimited(space0, parser, space0)
}

/// Parses a variable or a module name reference.
pub fn parse_var<'m, 'i>(module: &'m Module, input: &'i str) -> IResult<&'i str, AST> {
    let (rest, name) = parse_variable_name.parse(input)?;
    if is_reserved(name) {
        Err(nom::Err::Error(Error::new(input, ErrorKind::Tag)))
    } else if module.get_term(name).is_some() {
        Ok((rest, Name(name.to_string())))
    } else {
        Ok((rest, Var(name.to_string())))
    }
}

/// Parses a lambda-bound variable name, rejecting module names.
pub fn parse_lambda_var<'m, 'i>(module: &'m Module, input: &'i str) -> IResult<&'i str, &'i str> {
    let (rest, name) = parse_variable_name.parse(input)?;
    if is_reserved(name) || module.contains(name) {
        Err(nom::Err::Error(Error::new(input, ErrorKind::Tag)))
    } else {
        Ok((rest, name))
    }
}

/// Parses boolean literals: `true` or `false`.
pub fn parse_bool(input: &str) -> IResult<&str, AST> {
    alt((value(True, tag("true")), value(False, tag("false")))).parse(input)
}

/// Parses a natural number literal into `AST::Nat`.
pub fn parse_nat(input: &str) -> IResult<&str, AST> {
    map_res(digit1, |s: &str| s.parse::<u64>().map(Nat)).parse(input)
}

/// Parses an if-then-else expression.
pub fn parse_ite<'m, 'i>(module: &'m Module, input: &'i str) -> IResult<&'i str, AST> {
    (
        lex(tag("if")),
        |i| parse_ast(module, i),
        lex(tag("then")),
        |i| parse_ast(module, i),
        lex(tag("else")),
        |i| parse_ast(module, i),
    )
        .map(|(_, cond, _, if_true, _, if_false)| Ite {
            cond: Box::new(cond),
            if_true: Box::new(if_true),
            if_false: Box::new(if_false),
        })
        .parse(input)
}

/// Parses left-associative application chains.
pub fn parse_app<'m, 'i>(module: &'m Module, input: &'i str) -> IResult<&'i str, AST> {
    let (rest, t1) = parse_atom(module, input)?;

    fold_many0(
        preceded(space1, move |i| parse_atom(module, i)),
        move || t1.clone(),
        |t1, t2| App(Box::new(t1), Box::new(t2)),
    )
    .parse(rest)
}

/// Parses a long-form lambda: `fun x, body`.
use crate::types::Type;

fn parse_type(input: &str) -> IResult<&str, Type> {
    alt((
        value(Type::Nat, tag("Nat")),
        value(Type::Bool, tag("Bool")),
    ))
    .parse(input)
}
/// Parses a short-form lambda: `x => body`.
pub fn parse_abs<'m, 'i>(module: &'m Module, input: &'i str) -> IResult<&'i str, AST> {
    let (input, var) = parse_lambda_var(module, input)?;
    
    // Optional type annotation (e.g., x: Nat)
    let (input, ty) = if let Ok((input, _)) = lex(tag(":")).parse(input) {
        let (input, ty) = parse_type(input)?;
        (input, ty)
    } else {
        (input, Type::Nat) // default type for untyped lambdas
    };

    // Require '=>'
    let (input, _) = lex(tag("=>")).parse(input)?;
    
    // Parse body
    let (input, body) = parse_ast(module, input)?;

    Ok((input, AST::Abs { var: var.to_string(), ty, body: Box::new(body) }))
}

pub fn parse_lambda<'m, 'i>(module: &'m Module, input: &'i str) -> IResult<&'i str, AST> {
    let (input, _) = opt(lex(tag("fun"))).parse(input)?; // optional 'fun'
    parse_abs(module, input)
}

/// Parses a parenthesized expression.
pub fn parse_paren<'m, 'i>(module: &'m Module, input: &'i str) -> IResult<&'i str, AST> {
    delimited(tag("("), lex(|i| parse_ast(module, i)), tag(")")).parse(input)
}

/// Parses an atomic expression (no infix operators).
pub fn parse_atom<'m, 'i>(module: &'m Module, input: &'i str) -> IResult<&'i str, AST> {
    alt((
        |i| parse_paren(module, i),
        |i| parse_ite(module, i),
        parse_bool,
        |i| parse_lambda(module, i),
        parse_nat,
        |i| parse_var(module, i),
    ))
    .parse(input)
}

/// Parses a full expression with infix precedence.
pub fn parse_ast<'m, 'i>(module: &'m Module, input: &'i str) -> IResult<&'i str, AST> {
    precedence(
        unary_op(1, lex(tag("not"))),
        fail(),
        alt((
            binary_op(2, Assoc::Left, lex(tag("*"))),
            binary_op(3, Assoc::Left, lex(tag("+"))),
            binary_op(3, Assoc::Left, lex(tag("-"))),
            binary_op(4, Assoc::Left, lex(tag("=="))),
            binary_op(4, Assoc::Left, lex(tag("!="))),
            binary_op(4, Assoc::Left, lex(tag("<="))),
            binary_op(4, Assoc::Left, lex(tag("<"))),
            binary_op(4, Assoc::Left, lex(tag(">="))),
            binary_op(4, Assoc::Left, lex(tag(">"))),
            binary_op(5, Assoc::Left, lex(tag("and"))),
            binary_op(6, Assoc::Left, lex(tag("or"))),
        )),
        lex(move |i| parse_app(module, i)),
        |op: Operation<&str, (), &str, AST>| {
            use nom_language::precedence::Operation::*;
            match op {
                Prefix("not", o) => Ok(Not(Box::new(o))),
                Binary(lhs, "*", rhs) => Ok(Mul(Box::new(lhs), Box::new(rhs))),
                Binary(lhs, "+", rhs) => Ok(Add(Box::new(lhs), Box::new(rhs))),
                Binary(lhs, "-", rhs) => Ok(Sub(Box::new(lhs), Box::new(rhs))),
                Binary(lhs, "==", rhs) => Ok(Eq(Box::new(lhs), Box::new(rhs))),
                Binary(lhs, "!=", rhs) => Ok(Neq(Box::new(lhs), Box::new(rhs))),
                Binary(lhs, "<", rhs) => Ok(Lt(Box::new(lhs), Box::new(rhs))),
                Binary(lhs, "<=", rhs) => Ok(Le(Box::new(lhs), Box::new(rhs))),
                Binary(lhs, ">", rhs) => Ok(Gt(Box::new(lhs), Box::new(rhs))),
                Binary(lhs, ">=", rhs) => Ok(Ge(Box::new(lhs), Box::new(rhs))),
                Binary(lhs, "and", rhs) => Ok(And(Box::new(lhs), Box::new(rhs))),
                Binary(lhs, "or", rhs) => Ok(Or(Box::new(lhs), Box::new(rhs))),
                _ => Err("invalid operator"),
            }
        },
    )(input)
}

/// Parses a single module declaration: `name = expr`.
pub fn parse_decl<'m, 'i>(module: &'m Module, input: &'i str) -> IResult<&'i str, (String, AST)> {
    let (rest, name) = parse_variable_name.parse(input)?;
    if is_reserved(name) {
        return Err(nom::Err::Error(Error::new(input, ErrorKind::Tag)));
    }
    let (rest, (_, ast)) = (lex(tag("=")), |i| parse_ast(module, i)).parse(rest)?;
    Ok((rest, (name.to_string(), ast)))
}

/// Parses a single declaration line (one declaration per line).
pub fn parse_decl_line<'m, 'i>(
    module: &'m Module,
    input: &'i str,
) -> IResult<&'i str, (String, AST)> {
    terminated(
        |i| parse_decl(module, i),
        opt(preceded(space0, line_ending)),
    )
    .parse(input)
}

/// Parses a module.
/// Declarations are available from top to bottom as `AST::Name`.
/// Uses the module as state so earlier declarations can be referenced as names in the AST.
pub fn parse_module(mut input: &str) -> IResult<&str, Module> {
    let mut module = Module::new_with_prelude();
    while let Ok((rest, (name, decl))) = parse_decl_line(&module, input) {
        module.insert(name, decl);
        input = rest;
    }
    let (rest, _) = eof(input)?;
    Ok((rest, module))
}
