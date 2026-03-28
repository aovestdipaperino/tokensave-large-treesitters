//! All tree-sitter grammars for tokensave.
//!
//! Tier: **large** — 22+ languages (includes all medium-tier languages).
//!
//! Additional languages: Zig, Nix, Protobuf, Perl, Fortran

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
pub fn all_languages() -> Vec<(&'static str, tree_sitter_language::LanguageFn)> {
    let mut langs = tokensave_medium_treesitters::all_languages();
    langs.extend([
        ("zig", tree_sitter_zig::LANGUAGE),
        ("nix", tree_sitter_nix::LANGUAGE),
        ("protobuf", tree_sitter_proto::LANGUAGE),
        ("perl", tree_sitter_perl::LANGUAGE),
        ("fortran", tree_sitter_fortran::LANGUAGE),
    ]);
    langs
}
