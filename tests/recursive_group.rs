//! AI assistance: this file was written with AI assistance. The maintainer reviewed it and did not find errors.
//!
//! Mutually recursive parser groups.

use marser::{
    capture,
    matcher::MatcherCombinator,
    one_of::one_of,
    parser::{Deferred, Parser, recursive2},
};

fn mutually_recursive<'src>() -> (
    Deferred<'src, 'src, &'src str, usize>,
    Deferred<'src, 'src, &'src str, bool>,
) {
    recursive2(|number, flag| {
        let recursive_number =
            capture!(('n', bind!(flag.clone(), flag_value)) => if flag_value { 1 } else { 2 });
        let terminal_number = '0'.to(0usize);

        let recursive_flag =
            capture!(('f', bind!(number.clone(), number_value)) => number_value == 0);
        let terminal_flag = 't'.to(true);

        (
            one_of((recursive_number, terminal_number)),
            one_of((recursive_flag, terminal_flag)),
        )
    })
}

#[test]
fn recursive2_supports_mutually_recursive_parsers() {
    let (number, flag) = mutually_recursive();

    let (number_out, number_errors) = number.parse_str("nf0").expect("parse number");
    assert!(number_errors.is_empty());
    assert_eq!(number_out, 1);

    let (flag_out, flag_errors) = flag.parse_str("fnf0").expect("parse flag");
    assert!(flag_errors.is_empty());
    assert!(!flag_out);
}
