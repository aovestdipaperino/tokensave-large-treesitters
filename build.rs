use std::path::Path;

fn main() {
    // Block-level markdown — tree-sitter-grammars/tree-sitter-markdown.
    let md_dir = Path::new("vendor/tree-sitter-markdown/src");
    cc::Build::new()
        .include(md_dir)
        .file(md_dir.join("parser.c"))
        .file(md_dir.join("scanner.c"))
        .warnings(false)
        .compile("tree_sitter_markdown");
    println!("cargo::rerun-if-changed=vendor/tree-sitter-markdown/src/parser.c");
    println!("cargo::rerun-if-changed=vendor/tree-sitter-markdown/src/scanner.c");

    // Inline markdown — applied to each `(inline)` node produced by the block
    // grammar to extract links, emphasis, autolinks, etc.
    let md_inline_dir = Path::new("vendor/tree-sitter-markdown-inline/src");
    cc::Build::new()
        .include(md_inline_dir)
        .file(md_inline_dir.join("parser.c"))
        .file(md_inline_dir.join("scanner.c"))
        .warnings(false)
        .compile("tree_sitter_markdown_inline");
    println!("cargo::rerun-if-changed=vendor/tree-sitter-markdown-inline/src/parser.c");
    println!("cargo::rerun-if-changed=vendor/tree-sitter-markdown-inline/src/scanner.c");

    let proto_dir = Path::new("vendor/tree-sitter-protobuf/src");
    cc::Build::new()
        .include(proto_dir)
        .file(proto_dir.join("parser.c"))
        .warnings(false)
        .compile("tree_sitter_protobuf");
    println!("cargo::rerun-if-changed=vendor/tree-sitter-protobuf/src/parser.c");

    let dockerfile_dir = Path::new("vendor/tree-sitter-dockerfile/src");
    cc::Build::new()
        .include(dockerfile_dir)
        .file(dockerfile_dir.join("parser.c"))
        .file(dockerfile_dir.join("scanner.c"))
        .warnings(false)
        .compile("tree_sitter_dockerfile");
    println!("cargo::rerun-if-changed=vendor/tree-sitter-dockerfile/src/parser.c");
    println!("cargo::rerun-if-changed=vendor/tree-sitter-dockerfile/src/scanner.c");

    let cobol_dir = Path::new("vendor/tree-sitter-cobol/src");
    cc::Build::new()
        .include(cobol_dir)
        .file(cobol_dir.join("parser.c"))
        .file(cobol_dir.join("scanner.c"))
        .warnings(false)
        .compile("tree_sitter_cobol");
    println!("cargo::rerun-if-changed=vendor/tree-sitter-cobol/src/parser.c");
    println!("cargo::rerun-if-changed=vendor/tree-sitter-cobol/src/scanner.c");

    let quint_dir = Path::new("vendor/tree-sitter-quint/src");
    cc::Build::new()
        .include(quint_dir)
        .file(quint_dir.join("parser.c"))
        .warnings(false)
        .compile("tree_sitter_quint");
    println!("cargo::rerun-if-changed=vendor/tree-sitter-quint/src/parser.c");
}
