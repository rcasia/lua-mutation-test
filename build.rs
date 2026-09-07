use std::path::Path;

fn main() {
    let src_dir = Path::new("tree-sitter-lua/src");
    let parser_c = src_dir.join("parser.c");
    let scanner_c = src_dir.join("scanner.c");

    if !parser_c.exists() {
        panic!(
            "parser.c not found at {}. Run scripts/fetch-tree-sitter-lua.sh first.",
            parser_c.display()
        );
    }

    let mut build = cc::Build::new();
    build.file(&parser_c);

    if scanner_c.exists() {
        build.file(&scanner_c); // required for some grammar versions
    }

    build.compile("tree_sitter_lua");

    println!("cargo:rerun-if-changed={}", parser_c.display());
    println!("cargo:rerun-if-changed={}", scanner_c.display());
    println!("cargo:rerun-if-changed=build.rs");
}
