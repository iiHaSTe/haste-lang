use crate::parser::node_types::{
    NodeProgram,
    NodeStatment,
    NodeExpr
};

#[derive(Debug)]
pub struct JSGenerator<'a> {
    tree: &'a NodeProgram
}

impl<'a> JSGenerator<'a> {
    pub fn new(tree: &'a NodeProgram) -> Self {
        return JSGenerator {tree}
    }
    pub fn generate_programe(&self) -> String {
        let mut result = String::new();
        for line in self.tree.body.iter() {
            result.push_str(&self.generate_statment(&line));
        }
        return result;
    }
    fn generate_statment(&self, statment: &NodeStatment) -> String {
        return match statment {
            NodeStatment::Exit(value) =>
                format!("process.exit({});\n", self.generate_expr(&value)),
            NodeStatment::Print(value) =>
                format!("console.log({});\n", self.generate_expr(&value)),
            NodeStatment::Var(ident, value) =>
                format!("let {} = {};\n", self.generate_expr(ident), self.generate_expr(value)),
            NodeStatment::Assignment(ident, value) =>
                format!("{} = {};\n", self.generate_expr(ident), self.generate_expr(value)),
            _ => {
                eprintln!("[Syntax Error] no freinds?");
                std::process::exit(1);
            }
        }
    }
    fn generate_expr(&self, expr: &NodeExpr) -> String {
        return match expr {
            NodeExpr::Ident(value) => value.to_string(),
            NodeExpr::IntLit(value) => value.to_string(),
            NodeExpr::StringLit(value, formated) =>
                if *formated {
                    format!("`{}`", value)
                } else {
                    format!("\"{}\"", value)
                },
            NodeExpr::FloatLit(value) => value.to_string(),
            NodeExpr::Boolean(value) =>
                if *value {
                    String::from("true")
                } else {
                    String::from("false")
                },
            NodeExpr::Null => String::from("null"),
        };
    }
}

