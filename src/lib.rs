//! All tree-sitter grammars for tokensave.
//!
//! Tier: **large** — 34 languages (includes all medium-tier languages).
//!
//! Additional languages: Zig, Nix, Protobuf, Perl, Fortran, Pascal,
//! PowerShell, VB.NET, Objective-C, Batch, COBOL, MSBASIC2, GW-BASIC, QBasic,
//! GLSL, Markdown

pub use tokensave_medium_treesitters;
pub use tree_sitter;

pub mod languages {
    pub use tokensave_medium_treesitters::languages::*;
    pub use tree_sitter_batch;
    pub use tree_sitter_fortran;
    pub use tree_sitter_gwbasic;
    pub use tree_sitter_msbasic2;
    pub use tree_sitter_nix;
    pub use tree_sitter_objc;
    pub use tree_sitter_pascal;
    pub use tree_sitter_perl;
    pub use tree_sitter_powershell;
    pub use tree_sitter_qbasic;
    pub use tree_sitter_vb_dotnet;
    pub use tree_sitter_glsl;
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
    ]);
    langs
}
