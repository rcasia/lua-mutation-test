use tree_sitter::{Parser, Language};

// This function is provided by the generated parser (tree-sitter-lua)
extern "C" {
    fn tree_sitter_lua() -> Language;
}

fn main() {
    // Initialize the parser
    let mut parser = Parser::new();

    // Load the Lua language
    parser
        .set_language(unsafe { &tree_sitter_lua() })
        .expect("Error loading tree-sitter-lua");

    // Example code
    let source_code = "if a == b then return true end";

    // Parse
    let tree = parser.parse(source_code, None).expect("Parsing failed");

    let root_node = tree.root_node();
    println!("Root node: {}", root_node.kind());
    println!("Text: {:?}", &source_code[root_node.byte_range()]);
    println!();

    // Traverse children of the root node
    let mut cursor = root_node.walk();
    for child in root_node.children(&mut cursor) {
        println!(
            "- {} [{}..{}]",
            child.kind(),
            child.start_byte(),
            child.end_byte()
        );
    }
}
