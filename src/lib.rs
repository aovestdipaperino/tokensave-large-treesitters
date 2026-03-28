//! All tree-sitter grammars for tokensave.
//!
//! Tier: **large** — 28 languages (includes all medium-tier languages).
//!
//! Additional languages: Zig, Nix, Protobuf, Perl, Fortran,
//! COBOL, QBasic, Batch, PowerShell, Pascal, VB.NET
//!
//! Note: Some niche languages (COBOL, QBasic, Batch, PowerShell, Pascal, VB.NET)
//! do not yet have stable tree-sitter crates on crates.io and are listed as
//! planned additions.

pub use tokensave_medium_treesitters;
pub use tree_sitter;

pub mod languages {
    pub use tokensave_medium_treesitters::languages::*;
    pub use tree_sitter_fortran;
    pub use tree_sitter_nix;
    pub use tree_sitter_perl;
    pub use tree_sitter_proto;
    pub use tree_sitter_zig;
}

/// Returns (name, language_fn) pairs for all large-tier languages.
pub fn all_languages() -> Vec<(&'static str, tree_sitter::Language)> {
    let mut langs = tokensave_medium_treesitters::all_languages();
    langs.extend([
        ("zig", tree_sitter_zig::LANGUAGE.into()),
        ("nix", tree_sitter_nix::LANGUAGE.into()),
        ("protobuf", tree_sitter_proto::LANGUAGE.into()),
        ("perl", tree_sitter_perl::LANGUAGE.into()),
        ("fortran", tree_sitter_fortran::LANGUAGE.into()),
    ]);
    langs
}
