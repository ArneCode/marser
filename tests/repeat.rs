//! AI assistance: this file was written with AI assistance. The maintainer reviewed it and did not find errors.
//!
//! Integration tests for [`marser::matcher::repeat`].

use marser::capture;
use marser::matcher::{
    many, one_or_more, optional, repeat,
};
use marser::parser::{token_parser, Parser};

fn parse_ok<'src, P: Parser<'src, &'src str>>(parser: P, src: &'src str) -> bool {
    parser.parse_str(src).is_ok()
}

#[test]
fn repeat_exactly_three() {
    let p = capture!((repeat('a', 3..=3), marser::matcher::end_of_input()) => ());
    assert!(parse_ok(p.clone(), "aaa"));
    assert!(!parse_ok(p.clone(), "aa"));
    assert!(!parse_ok(p, "aaaa"));
}

#[test]
fn repeat_half_open_range() {
    let p = capture!((repeat('a', 2..5), marser::matcher::end_of_input()) => ());
    assert!(parse_ok(p.clone(), "aa"));
    assert!(parse_ok(p.clone(), "aaa"));
    assert!(parse_ok(p.clone(), "aaaa"));
    assert!(!parse_ok(p.clone(), "a"));
    assert!(!parse_ok(p, "aaaaa"));
}

#[test]
fn repeat_unbounded_from_min() {
    let p = capture!((repeat('a', 2..), marser::matcher::end_of_input()) => ());
    assert!(parse_ok(p.clone(), "aa"));
    assert!(parse_ok(p.clone(), "aaaa"));
    assert!(!parse_ok(p, "a"));
}

#[test]
fn repeat_inclusive_range() {
    let p = capture!((repeat('a', 2..=4), marser::matcher::end_of_input()) => ());
    assert!(parse_ok(p.clone(), "aa"));
    assert!(parse_ok(p.clone(), "aaaa"));
    assert!(!parse_ok(p.clone(), "a"));
    assert!(!parse_ok(p, "aaaaa"));
}

#[test]
fn repeat_empty_range_always_fails() {
    let p = capture!(repeat('a', 2..2) => ());
    assert!(!parse_ok(p.clone(), ""));
    assert!(!parse_ok(p, "aa"));
}

#[test]
fn repeat_stops_on_no_progress() {
    let p = capture!((repeat((), 0..), marser::matcher::end_of_input()) => ());
    assert!(parse_ok(p, ""));
}

#[test]
fn repeat_capture_bounded() {
    let digit = token_parser(
        |c: &char| c.is_ascii_digit(),
        |c| c.to_digit(10).unwrap(),
    );
    let p = capture!(repeat(bind!(digit.clone(), *ds), 2..=3) => ds);
    let (ds, _) = p.parse_str("12").expect("two digits");
    assert_eq!(ds, vec![1, 2]);
    let (ds, _) = p.parse_str("123").expect("three digits");
    assert_eq!(ds, vec![1, 2, 3]);
    assert!(p.parse_str("1").is_err());
    assert!(p.parse_str("1234").is_err());
}

#[test]
fn many_still_accepts_unbounded_run() {
    let p = capture!((many('a'), marser::matcher::end_of_input()) => ());
    assert!(parse_ok(p.clone(), ""));
    assert!(parse_ok(p.clone(), "aaa"));
    assert!(!parse_ok(p, "b"));
}

#[test]
fn one_or_more_fails_on_empty() {
    let p = capture!((one_or_more('0'..='9'), marser::matcher::end_of_input()) => ());
    assert!(!parse_ok(p.clone(), ""));
    assert!(parse_ok(p, "123"));
}

#[test]
fn optional_unchanged() {
    let p = capture!((optional('-'), marser::matcher::end_of_input()) => ());
    assert!(parse_ok(p.clone(), ""));
    assert!(parse_ok(p.clone(), "-"));
    assert!(!parse_ok(p, "--"));
}
