use std::path::Path;

fn main() {
    let md_dir = Path::new("vendor/tree-sitter-markdown/src");
    cc::Build::new()
        .include(md_dir)
        .file(md_dir.join("parser.c"))
        .warnings(false)
        .compile("tree_sitter_markdown_parser");
    cc::Build::new()
        .cpp(true)
        .include(md_dir)
        .file(md_dir.join("scanner.cc"))
        .define("TREE_SITTER_MARKDOWN_AVOID_CRASH", None)
        .warnings(false)
        .compile("tree_sitter_markdown_scanner");
    println!("cargo::rerun-if-changed=vendor/tree-sitter-markdown/src/parser.c");
    println!("cargo::rerun-if-changed=vendor/tree-sitter-markdown/src/scanner.cc");


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
}
