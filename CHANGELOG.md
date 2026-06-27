# Changelog

**AI assistance:** This document was drafted with AI assistance. The maintainer reviewed it. If anything looks wrong, please [open an issue](https://github.com/ArneCode/marser/issues/new).

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- `matcher::start_of_input()` and `matcher::end_of_input()` zero-width matchers for input-boundary checks in grammars; `end_of_input()` is equivalent to `negative_lookahead(AnyToken)`.

## [0.2.1] - 2026-06-18

### Fixed
- linked to wrong image for trace-viewer. Fixed it.

## [0.2.0] - 2026-06-18

### Changed

- README image URLs are pinned to a fixed commit so screenshots stay stable on docs.rs and crates.io; repository links in the README use absolute GitHub URLs so they resolve on docs.rs as well as on GitHub and crates.io. The embedded guide, trace-crate READMEs, and example-path references use the same absolute URL patterns (docs.rs-first for Cargo features and tracing docs).

### Added

- **`no_std` support** with the `alloc` crate (heap-backed `Vec`, `String`, and related types). Use `default-features = false` on the `marser` dependency for embedded or other targets without `std`.
- Cargo feature **`std`** (enabled by default) gates standard-library helpers such as `ParserError::eprint` / `write` and trace-to-file APIs.
- `parser::recursive2` through `parser::recursive12` build mutually recursive parser groups while keeping deferred parser initialization internal.

### Removed

- The `profile_json_parse` binary and `scripts/flamegraph-json-parse.sh` helper (use `cargo bench --bench json_parse` with `[profile.profiling]` or `profile.samply` for sampling instead).

### Changed

- Internal maps and sets use **`hashbrown`** so the same code path works with and without `std`.
- **`parser-trace`** now requires **`std`** (file I/O and trace viewer workflows stay std-only).

## [0.1.4] - 2026-05-31

### Added

- Criterion and profiling fixtures `twitter.json` and `citm_catalog.json` (simdjson-data) alongside `canada.json`.

### Changed

- Removed `ParserContext::is_in_error_recovery`; the whole-input parse driver now selects the error-recovery pass via internal `Mode` (`Emit<false, false>` vs `Emit<true, false>`). Custom parse drivers that relied on that field should pass the appropriate mode to the crate-private parse entry point instead.
- `cargo bench` uses `[profile.bench]` tuned for throughput (`lto`, `codegen-units = 1`, no debug info). Sampling profilers and flamegraphs should use `[profile.profiling]` (`debug = true`, LTO off); `profile.samply` inherits `profiling`.
- Guide AI-assistance notices on docs.rs use rustdoc’s `.warning` callout styling so they read correctly in dark and Ayu themes (replacing fixed light-theme colors).
- The guide and `capture!` docs now document **E0283** on disconnected or unused `let` rules (for example after commenting out the only `one_of` / `.trace()` use of a branch).

## [0.1.3] - 2026-05-23

### Changed
- improved error recovery of json example
- changed docs for trace viewer

### Fixed
- label messed up error recognition if inner parser returned error

## [0.1.2] - 2026-05-20

### Added
- README files under `examples/`, `examples/json/`, and `examples/mini_language/` with run commands and layout notes.

### Changed
- README: links (docs.rs, crates.io, LICENSE, CONTRIBUTING), trimmed AI/tracing sections, example table points at `examples/*/README.md`, general improvements.

### Removed
- The `mini_script` example and its unused fixtures under `tests/data/mini_script/`.

## [0.1.1] - 2026-05-19

### Changed

- docs.rs rustdoc includes scraped code from the `json`, `mini_language`, and `mini_script` examples on relevant API pages (`doc-scrape-examples`).

### Fixed

- docs.rs builds again: `doc_auto_cfg` (removed in Rust 1.92) is replaced with `doc_cfg` under the `docsrs` cfg.

## [0.1.0] - 2026-05-19

### Changed

- Minimum supported Rust version is **1.88** (stable let chains in `if`/`while`).
- CI runs a nightly `cargo doc` with `--cfg docsrs` and the same features as docs.rs so removed or renamed rustdoc features are caught before publish.

## [0.1.0] - 2026-05-19

### Added

- Initial public release of `marser`, `marser_macros`, `marser-trace-schema`, and `marser-trace-viewer`.
- PEG-style parser combinators with `capture!`, matcher-level backtracking, and error recovery.
- Optional `annotate-snippets` feature for terminal diagnostics (`ParserError::eprint`, etc.).
- Optional `parser-trace` feature and experimental trace schema / viewer crates.
- Optional `embed-guide` feature for long-form rustdoc chapters (`marser::guide`).
- Integration tests, JSON example grammar, and Criterion benchmarks in the repository.

### Notes

- Trace file formats and trace-related APIs may change in future releases; pin versions and read release notes when upgrading.
- Macro expansion details are not a stability guarantee; use `capture!` and documented helpers as the public API.

[0.2.1]: https://github.com/ArneCode/marser/releases/tag/0.2.1
[0.2.0]: https://github.com/ArneCode/marser/releases/tag/0.2.0
[0.1.4]: https://github.com/ArneCode/marser/releases/tag/0.1.4
[0.1.3]: https://github.com/ArneCode/marser/releases/tag/0.1.3
[0.1.2]: https://github.com/ArneCode/marser/releases/tag/0.1.2
[0.1.1]: https://github.com/ArneCode/marser/releases/tag/0.1.1
[0.1.0]: https://github.com/ArneCode/marser/releases/tag/0.1.0