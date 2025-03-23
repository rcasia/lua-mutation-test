fn main() {
    cc::Build::new()
        .file("tree-sitter-lua/src/parser.c")
        .file("tree-sitter-lua/src/scanner.c") // ¡necesario!
        .compile("tree_sitter_lua");

    println!("cargo:rerun-if-changed=tree-sitter-lua/src/parser.c");
    println!("cargo:rerun-if-changed=tree-sitter-lua/src/scanner.c");
}
