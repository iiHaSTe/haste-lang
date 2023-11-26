#[derive(Debug)]
pub enum NodeExpr {
    Ident(String),
    IntLit(String),
    StringLit(String, bool),
    FloatLit(String),
    Boolean(bool),
    Null,
}

#[derive(Debug)]
pub enum NodeStatment {
    Exit(NodeExpr),
    Print(NodeExpr),
    Var(NodeExpr, NodeExpr),
    Assignment(NodeExpr, NodeExpr)
}

#[derive(Debug)]
pub struct NodeProgram {
    pub body: Vec<NodeStatment>
}
