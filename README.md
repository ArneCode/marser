# marser

`marser` is a parser-combinator library for writing **PEG-style grammars in Rust** with a focus on useful errors, error recovery and good performance.

Why build yet another parser-combinator library? You can read about it [here](https://blog.arnedebo.com/posts/a-grammar-first-approach-to-parser-combinators/).

It supports: 
- **Zero-copy parsing** for faster parsers
- **Multiple input formats** - use `&str` and `&[u8]` / `&[T]` or implement the `Input` trait yourself.
- **Packrat-style caching** - just wrap your parsers in `.memoized()` to cache results at each position.
- **Simple debugging** of your parsers using a custom TUI
- **no_std**

## Quickstart

To add this library to your Rust project run:
```bash
cargo add marser
```

This library has a couple of optional features. You can find them [below](#cargo-features).

## Example
This example parses dice notation like `2d6` into a struct:

```rust
use marser::capture;
use marser::matcher::one_or_more;
use marser::parser::Parser;

// the struct we want to parse into
#[derive(Debug, PartialEq)]
struct Roll {
    count: u32,
    sides: u32,
}

// A parser that can parse a number
fn number<'src>() -> impl Parser<'src, &'src str, Output = u32> + Clone {
    // capture defines a parser. It consists of a matcher (the part before `=>`) 
    // and a Rust expression that builds the output value (the part after `=>`).
    capture!(
        bind_slice!( // bind_slice! stores the matched part of the input inside a variable
            one_or_more('0'..='9'), // matches any sequence of digits
            number_slice as &'src str // the matched digits are available as `number_slice` of type `&'src str`
        )
        => // we can then define how to build the output value from the bound variables
            number_slice // we use the captured slice of digits
                .parse() // and parse it into a u32
                .expect("matched only digits")
    )
}

// A parser that can parse a roll like `2d6`
fn roll<'src>() -> impl Parser<'src, &'src str, Output = Roll> + Clone {
    // we again define a parser with capture!, this time for the whole roll
    capture!(
        ( // we define a sequence by putting multiple matchers in a tuple
          // they are matched one after another
            bind!(number(), count), // first we expect a number. We use bind! to store its value in `count`
            'd', // then we expect the literal character 'd'
            bind!(number(), sides) // then we expect another number, which we store in `sides`
        )
        => // finally we define how to build the output value from the bound variables
            Roll { count, sides }
    )
}

fn main() {
    // we can then use this parser we defined to parse a string
    let (roll, _errors) = roll().parse_str("2d6").unwrap();
    assert_eq!(roll, Roll { count: 2, sides: 6 });
}
```
Runnable examples live under [`examples/`](https://github.com/ArneCode/marser/tree/main/examples) (see also [below](#examples-in-this-repository)).

## Learn more

- [Guide](https://docs.rs/marser/latest/marser/guide/index.html) on docs.rs
- [grammar-to-marser](https://grammar-to-marser.arnedebo.com/) - Input a PEG/PEST grammar and get out a working marser parser
- [API documentation](https://docs.rs/marser)
- [crates.io](https://crates.io/crates/marser)
- [Design rationale: A Grammar-First Approach to Parser Combinators in Rust](https://blog.arnedebo.com/posts/a-grammar-first-approach-to-parser-combinators/)

## Cargo features

| Feature                 | When you need it                                                                                                                                                                                                                                                      |
| ----------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **`std`** *(default)*   | Standard-library integration: `ParserError::eprint` / `write`, trace-to-file helpers, and other I/O. Disable with `default-features = false` for embedded or other **`no_std` + `alloc`** targets.                                                                    |
| **`annotate-snippets`** | Rich terminal diagnostics via [annotate-snippets](https://docs.rs/annotate-snippets). Works on `no_std` builds for string rendering; `eprint` / `write` still need **`std`**.                                                                                         |
| **`parser-trace`**      | **Experimental:** record parser traces to replay them in the trace viewer TUI (requires **`std`**). See the [tracing guide](https://docs.rs/marser/latest/marser/guide/tracing_and_debugging/index.html) and [`marser-trace-viewer/`](https://github.com/ArneCode/marser/tree/main/marser-trace-viewer). |


## Requirements

- **Rust 1.88 or later** 

## Examples in this repository

Examples need the **`annotate-snippets`** feature for rendering of errors

| Example                                                  | What it shows                                                                                                  |
| -------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------- |
| [`examples/json/`](https://github.com/ArneCode/marser/tree/main/examples/json)                       | A JSON parser with error recovery and custom error messages.                                                   |
| [`examples/mini_language.rs`](https://github.com/ArneCode/marser/blob/main/examples/mini_language.rs) | Small language: statements, operator precedence, functions etc. with error recovery and custom error messages. |

Run JSON from a git clone:

```bash
cargo run --example json --features annotate-snippets -- tests/data/json1.json
```

### Error output sample

Input:

```json
{
    "foo": 123,
    "bar": [1, ,2 ,3
}
```

Example diagnostic, rendered using **`annotate-snippets`**:

![Example parse error for invalid JSON Screenshot](https://raw.githubusercontent.com/ArneCode/marser/feaabd44f8d684642c820363f5387fbccdea3f03/img/image.png)

This parser can also still produce a recovered output:

```json
{
    "foo": 123,
    "bar": [
        1,
        2,
        3
    ]
}
```

The json example also has tracing support, so parsing can be stepped through in the trace viewer. See screenshot below. Left side is the rust source code for the parser, right side is the file being parsed.
<img width="1126" height="531" alt="Trace viewer stepping through the JSON parser" src="https://raw.githubusercontent.com/ArneCode/marser/main/img/trace-viewer.png" />

## Performance compared to other libraries:

Below is a comparison of the speed of different libraries for parsing json, including marser. I used json because there are already parsers using different libraries written for it

<img src="https://raw.githubusercontent.com/ArneCode/marser/feaabd44f8d684642c820363f5387fbccdea3f03/img/chart.png" width="50%">

Code for other libraries taken from [parse-rosetta](https://github.com/rosetta-rs/parse-rosetta-rs). Read more [here](https://github.com/ArneCode/json-parser-compare).

The difference in speed between the marser implementation with error recovery and diagnostics ("marser") and the implementation without error recovery and diagnostics ("marser-bare") is quite small because marser works in two modes. First the parser is run without error recovery logic. If the parser encounters an error, it is restarted with error recovery included. This makes it so that the performance cost of including error recovery and diagnostics is only very little. 

## Early release

**Early release:** `marser` is my first published Rust library. Feedback on the API, error messages, and docs is welcome — [open an issue](https://github.com/ArneCode/marser/issues/new)

## License

This project is licensed under the [MIT License](https://github.com/ArneCode/marser/blob/main/LICENSE).

## AI assistance

Parts of this repository were drafted or expanded with AI tools (guide, library docs, tests, macros, trace crates, examples, and parts of this README). The maintainer reviewed this material. If you spot a mistake, please [open an issue](https://github.com/ArneCode/marser/issues/new).
