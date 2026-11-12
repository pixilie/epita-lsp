use tree_sitter::{Parser, Tree};

#[derive(Debug)]
pub struct Document {
    pub text: String,
    pub tree: Tree,
    pub version: i32,
}

impl Document {
    pub fn new(text: String, version: i32) -> Self {
        let tree = parse(&text, None);
        Self { text, tree, version }
    }

    pub fn update(&mut self, text: String, version: i32) {
        self.tree = parse(&text, None);
        self.text = text;
        self.version = version;
    }
}

fn parse(text: &str, old_tree: Option<&Tree>) -> Tree {
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_c::LANGUAGE.into())
        .expect("grammaire C incompatible");
    parser.parse(text, old_tree).expect("parsing impossible")
}
