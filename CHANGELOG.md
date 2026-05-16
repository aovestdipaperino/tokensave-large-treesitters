# Changelog

All notable changes to this crate are documented in this file.

The format is loosely based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- AWK grammar (vendored from `Beaglefoot/tree-sitter-awk` @ v0.7.2,
  registered as `awk`). No published Rust bindings exist, so the C
  sources are compiled in via `build.rs`.

Large tier now ships 48 languages.

## [0.5.0]

### Added
- Lean 4 grammar via `arborium-lean` (registered as `lean`).
- Quint grammar (vendored from `zdavison/tree-sitter-quint`, registered as `quint`).
- Kotlin grammar via `arborium-kotlin` (registered as `kotlin`, replacing the medium-tier binding).
- TOML grammar via `tree-sitter-toml-ng` (registered as `toml`).

### Changed
- Switched the vendored Markdown grammar to
  `tree-sitter-grammars/tree-sitter-markdown` (block + inline parsers).

### Fixed
- Bound the Markdown external scanner state size to prevent a serialize
  buffer overflow on pathological inputs.

## [0.4.0]

### Added
- R, SQL (via `tree-sitter-sequel`), Julia, Haskell, OCaml, Clojure (via
  `tree-sitter-clojure-orchard`), Erlang, Elixir, and F# grammars.

## Earlier releases

See `git log` for release history prior to 0.4.0.
