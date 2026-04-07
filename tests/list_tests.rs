use programming_language::module::Module;
use programming_language::term::Term;
use programming_language::types::Type;
use Term::*;

#[test]
fn test_is_empty() {
    let empty: Term = Nil(Some(Type::Nat));
    let non_empty: Term = Cons(Box::new(Zero), Box::new(Nil(Some(Type::Nat))));

    // is_empty should return Some(True/False)
    assert_eq!(empty.is_empty_eval(), Some(True));
    assert_eq!(non_empty.is_empty_eval(), Some(False));
}

#[test]
fn test_tail() {
    let list: Term = Cons(
        Box::new(Succ(Box::new(Zero))),
        Box::new(Cons(
            Box::new(Succ(Box::new(Succ(Box::new(Zero))))),
            Box::new(Nil(Some(Type::Nat))),
        )),
    );

    let expected_tail: Term = Cons(
        Box::new(Succ(Box::new(Succ(Box::new(Zero))))),
        Box::new(Nil(Some(Type::Nat))),
    );

    // tail returns Option<Term>, compare with expected
    assert_eq!(list.tail_eval(), Some(expected_tail));
}

#[test]
fn test_head() {
    let list: Term = Cons(Box::new(Zero), Box::new(Nil(Some(Type::Nat))));
    assert_eq!(list.head_eval(), Some(Zero));
}

#[test]
fn test_collect_list() {
    let list: Term = Cons(
        Box::new(Zero),
        Box::new(Cons(Box::new(Succ(Box::new(Zero))), Box::new(Nil(Some(Type::Nat))))),
    );

    let elems = list.collect_list().unwrap();
    assert_eq!(elems.len(), 2);
    assert_eq!(elems[0], &Zero);
    assert_eq!(elems[1], &Succ(Box::new(Zero)));
}

#[test]
fn test_complex_list() {
    let m = Module::new_with_prelude();

    // [succ zero, zero]
    let list = Cons(
        Box::new(App(Box::new(m.get_term("succ").unwrap()), Box::new(Zero))),
        Box::new(Cons(Box::new(Zero), Box::new(Nil(Some(Type::Nat))))),
    );

    let head = list.head_eval();
    let tail = list.tail_eval();
    let tail_head = tail.as_ref().unwrap().head_eval();

    assert_eq!(head, Some(App(Box::new(m.get_term("succ").unwrap()), Box::new(Zero))));
    assert_eq!(tail_head, Some(Zero));
    assert_eq!(list.is_empty_eval(), Some(False));
}