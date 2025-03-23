use tree_sitter::{Parser, Language};

// Esta función la provee el parser generado (tree-sitter-lua)
extern "C" {
    fn tree_sitter_lua() -> Language;
}

fn main() {
    // Inicializar el parser
    let mut parser = Parser::new();

    // Cargar el lenguaje Lua
    parser
        .set_language(unsafe { &tree_sitter_lua() })
        .expect("Error al cargar tree-sitter-lua");

    // Código de ejemplo
    let source_code = "if a == b then return true end";

    // Parsear
    let tree = parser.parse(source_code, None).expect("Fallo el parseo");

    let root_node = tree.root_node();
    println!("Root node: {}", root_node.kind());
    println!("Text: {:?}", &source_code[root_node.byte_range()]);
    println!();

    // Recorrer hijos del nodo raíz
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
