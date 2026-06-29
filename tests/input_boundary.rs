//! AI assistance: this file was written with AI assistance. The maintainer reviewed it and did not find errors.
//!
//! Integration tests for [`marser::matcher::start_of_input`] and [`marser::matcher::end_of_input`].

use marser::capture;
use marser::matcher::{end_of_input, start_of_input};
use marser::parser::Parser;

#[test]
fn end_of_input_succeeds_on_empty() {
    let p = capture!(end_of_input() => ());
    let ((), errs) = p.parse_str("").expect("parse");
    assert!(errs.is_empty());
}

#[test]
fn end_of_input_fails_with_trailing() {
    let p = capture!(end_of_input() => ());
    assert!(p.parse_str("x").is_err());
}

#[test]
fn end_of_input_after_consumed() {
    let p = capture!(('a', end_of_input()) => ());
    let ((), errs) = p.parse_str("a").expect("parse");
    assert!(errs.is_empty());
    assert!(p.parse_str("ab").is_err());
}

#[test]
fn start_of_input_at_beginning() {
    let p = capture!(start_of_input() => ());
    let ((), errs) = p.parse_str("").expect("parse empty");
    assert!(errs.is_empty());

    let with_char = capture!((start_of_input(), 'x', end_of_input()) => ());
    let ((), errs) = with_char.parse_str("x").expect("parse non-empty");
    assert!(errs.is_empty());
}

#[test]
fn start_of_input_fails_after_advance() {
    let p = capture!(('a', start_of_input()) => ());
    assert!(p.parse_str("a").is_err());
}

#[test]
fn start_of_input_with_subslice() {
    let src = "prefix:suffix";
    let p = capture!((start_of_input(), "suffix") => ());
    let ((), errs) = p.parse_whole_input(&src[7..]).expect("parse subslice");
    assert!(errs.is_empty());

    let p_after = capture!(('s', start_of_input()) => ());
    assert!(p_after.parse_whole_input(&src[7..]).is_err());
}
