use lua_mutation_test::parser::Parser;

fn main() {
    // Initialize the parser
    let mut parser = Parser::new().expect("Error loading tree-sitter-lua");

    // Example code
    let source_code = "if a == b then return true end";

    // Parse
    let tree = parser.parse_source(source_code).expect("Parsing failed");

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
