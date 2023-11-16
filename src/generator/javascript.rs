use crate::node_tree::{NodeTree, NodeLine};
use crate::traits::Token;

#[derive(Debug)]
pub struct JSGenerator<'a> {
    tree: &'a NodeTree
}

impl<'a> JSGenerator<'a> {
    pub fn new(tree: &'a NodeTree) -> Self {
        return JSGenerator {tree}
    }
    pub fn generate(&self) -> String {
        let mut result = String::new();
        for line in self.tree.body.iter() {
            match line {
                NodeLine::Exit(value) => result.push_str(&format!("process.exit({});\n", value)),
                NodeLine::Print(value) => {
                    match value {
                        Token::IntLit(v) => result.push_str(&format!("console.log({});\n", v)),
                        Token::StringLit(v) => result.push_str(&format!("console.log(\"{}\");\n", v)),
                        Token::Boolean(v) => result.push_str(&format!("console.log({});\n", v)),
                        Token::FloatLit(v) => result.push_str(&format!("console.log({});\n", v)),
                        _ => {},
                    }
                },
                _ => {},
            }
        }
        return result;
    }
}

