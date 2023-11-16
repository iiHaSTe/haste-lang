use crate::node_tree::{NodeTree, NodeStatment, NodeExpr};
use crate::traits::Token;

#[derive(Debug)]
pub struct JSGenerator<'a> {
    tree: &'a NodeTree
}

impl<'a> JSGenerator<'a> {
    pub fn new(tree: &'a NodeTree) -> Self {
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
            NodeStatment::Var(token, exp) => if let Some(value) = self.isIdent(&token) {
                format!("let {} = {};\n", value, self.generate_expr(&exp))
            } else {
                eprintln!("[Syntax Error] no valide identifier.");
                std::process::exit(1);
            },
            _ => {
                eprintln!("[Syntax Error] no freinds?");
                std::process::exit(1);
            }
        }
    }
    fn isIdent(&self, token: &Token) -> Option<String> {
        return match token {
            Token::Ident(value) => Some(value.to_string()),
            _ => None
        }
    }
    fn generate_expr(&self, expr: &NodeExpr) -> String {
        return match expr {
            NodeExpr::Ident(token) => match token {
                Token::Ident(value) => value.to_string(),
                _ => {
                    eprintln!("[Syntax Error] Univialable Token");
                    std::process::exit(1);
                }
            },
            NodeExpr::IntLit(token) => match token {
                Token::IntLit(value) => value.to_string(),
                _ => {
                    eprintln!("[Syntax Error] Univialable Token");
                    std::process::exit(1);
                }
            },
            NodeExpr::StringLit(token) => match token {
                Token::StringLit(value) => format!("\"{}\"", value),
                _ => {
                    eprintln!("[Syntax Error] Univialable Token");
                    std::process::exit(1);
                }
            },
            NodeExpr::FloatLit(token) => match token {
                Token::FloatLit(value) => value.to_string(),
                _ => {
                    eprintln!("[Syntax Error] Univialable Token");
                    std::process::exit(1);
                }
            },
            NodeExpr::Boolean(token) => match token {
                Token::Boolean(value) =>
                    if *value {
                        String::from("true")
                    } else {
                        String::from("false")
                    },
                _ => {
                    eprintln!("[Syntax Error] Univialable Token");
                    std::process::exit(1);
                },
            },
            NodeExpr::Null => String::from("null"),
        };
    }
}

