//! All tree-sitter grammars for tokensave.
//!
//! Tier: **large** — 47 languages (includes all medium-tier languages).
//!
//! Additional languages: Zig, Nix, Protobuf, Perl, Fortran, Pascal,
//! PowerShell, VB.NET, Objective-C, Batch, COBOL, MSBASIC2, GW-BASIC, QBasic,
//! GLSL, Markdown, R, SQL, Julia, Haskell, OCaml, Clojure, Erlang, Elixir,
//! F#, Lean 4, Quint, Kotlin, TOML

pub use tokensave_medium_treesitters;
pub use tree_sitter;

pub mod languages {
    pub use arborium_kotlin;
    pub use arborium_lean;
    pub use tokensave_medium_treesitters::languages::*;
    pub use tree_sitter_batch;
    pub use tree_sitter_clojure_orchard;
    pub use tree_sitter_elixir;
    pub use tree_sitter_erlang;
    pub use tree_sitter_fortran;
    pub use tree_sitter_fsharp;
    pub use tree_sitter_glsl;
    pub use tree_sitter_gwbasic;
    pub use tree_sitter_haskell;
    pub use tree_sitter_julia;
    pub use tree_sitter_msbasic2;
    pub use tree_sitter_nix;
    pub use tree_sitter_objc;
    pub use tree_sitter_ocaml;
    pub use tree_sitter_pascal;
    pub use tree_sitter_perl;
    pub use tree_sitter_powershell;
    pub use tree_sitter_qbasic;
    pub use tree_sitter_r;
    pub use tree_sitter_sequel;
    pub use tree_sitter_toml_ng;
    pub use tree_sitter_vb_dotnet;
    pub use tree_sitter_zig;
}

/// Vendored tree-sitter-protobuf grammar (compiled from C source via build.rs).
pub mod protobuf {
    unsafe extern "C" {
        fn tree_sitter_protobuf() -> *const ();
    }

    pub const LANGUAGE: tree_sitter_language::LanguageFn =
        unsafe { tree_sitter_language::LanguageFn::from_raw(tree_sitter_protobuf) };
}

/// Vendored tree-sitter-cobol grammar (compiled from C source via build.rs).
pub mod cobol {
    unsafe extern "C" {
        fn tree_sitter_COBOL() -> *const ();
    }

    pub const LANGUAGE: tree_sitter_language::LanguageFn =
        unsafe { tree_sitter_language::LanguageFn::from_raw(tree_sitter_COBOL) };
}

/// Vendored tree-sitter-markdown grammar (compiled from C/C++ source via build.rs).
pub mod markdown {
    unsafe extern "C" {
        fn tree_sitter_markdown() -> *const ();
    }

    pub const LANGUAGE: tree_sitter_language::LanguageFn =
        unsafe { tree_sitter_language::LanguageFn::from_raw(tree_sitter_markdown) };
}

/// tree-sitter-dockerfile exports a `language()` function, not a `LanguageFn`
/// constant. Reference the C symbol directly so we get a `LanguageFn`.
pub mod dockerfile {
    unsafe extern "C" {
        fn tree_sitter_dockerfile() -> *const ();
    }

    pub const LANGUAGE: tree_sitter_language::LanguageFn =
        unsafe { tree_sitter_language::LanguageFn::from_raw(tree_sitter_dockerfile) };
}

/// Vendored tree-sitter-quint grammar (compiled from C source via build.rs).
/// Source: zdavison/tree-sitter-quint @ 5155d17 — no Rust bindings published.
pub mod quint {
    unsafe extern "C" {
        fn tree_sitter_quint() -> *const ();
    }

    pub const LANGUAGE: tree_sitter_language::LanguageFn =
        unsafe { tree_sitter_language::LanguageFn::from_raw(tree_sitter_quint) };
}

/// `arborium-lean` exposes a `language()` fn (returning `tree_sitter::Language`)
/// rather than a `LanguageFn` constant; re-export it under a typed module.
pub mod lean {
    pub use arborium_lean::language;
}

/// `arborium-kotlin` exposes a `language()` fn rather than a `LanguageFn`
/// constant; re-export it under a typed module.
pub mod kotlin {
    pub use arborium_kotlin::language;
}

pub mod toml {
    pub use tree_sitter_toml_ng::LANGUAGE;
}

/// Returns (name, language_fn) pairs for all large-tier languages.
pub fn all_languages() -> Vec<(&'static str, tree_sitter_language::LanguageFn)> {
    let mut langs = tokensave_medium_treesitters::all_languages();
    langs.extend([
        ("zig", tree_sitter_zig::LANGUAGE),
        ("nix", tree_sitter_nix::LANGUAGE),
        ("protobuf", protobuf::LANGUAGE),
        ("perl", tree_sitter_perl::LANGUAGE),
        ("fortran", tree_sitter_fortran::LANGUAGE),
        ("pascal", tree_sitter_pascal::LANGUAGE),
        ("powershell", tree_sitter_powershell::LANGUAGE),
        ("vbnet", tree_sitter_vb_dotnet::LANGUAGE),
        ("objc", tree_sitter_objc::LANGUAGE),
        ("batch", tree_sitter_batch::LANGUAGE),
        ("cobol", cobol::LANGUAGE),
        ("msbasic2", tree_sitter_msbasic2::LANGUAGE),
        ("gwbasic", tree_sitter_gwbasic::LANGUAGE),
        ("qbasic", tree_sitter_qbasic::LANGUAGE),
        ("glsl", tree_sitter_glsl::LANGUAGE_GLSL),
        ("dockerfile", dockerfile::LANGUAGE),
        ("markdown", markdown::LANGUAGE),
        ("r", tree_sitter_r::LANGUAGE),
        ("sql", tree_sitter_sequel::LANGUAGE),
        ("julia", tree_sitter_julia::LANGUAGE),
        ("haskell", tree_sitter_haskell::LANGUAGE),
        ("ocaml", tree_sitter_ocaml::LANGUAGE_OCAML),
        ("clojure", tree_sitter_clojure_orchard::LANGUAGE),
        ("erlang", tree_sitter_erlang::LANGUAGE),
        ("elixir", tree_sitter_elixir::LANGUAGE),
        ("fsharp", tree_sitter_fsharp::LANGUAGE_FSHARP),
        ("lean", arborium_lean::language()),
        ("kotlin", arborium_kotlin::language()),
        ("toml", tree_sitter_toml_ng::LANGUAGE),
        ("quint", quint::LANGUAGE),
    ]);
    langs
}
