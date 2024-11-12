pub struct RootNode {}

pub enum Node {
    Literal(LiteralNode)
}

#[derive(Debug, PartialEq, Clone)]
pub enum LiteralNode {
    Number(f64),
    String(String),
    Boolean(bool),
}
