pub struct RootNode {
    nodes: Vec<Node>,
}

impl From<Vec<Node>> for RootNode {
    fn from(value: Vec<Node>) -> Self {
        Self { nodes: value }
    }
}

impl RootNode {
    pub fn len(&self) -> usize {
        self.nodes.len()
    }
}

pub enum Node {
    Block(BlockNode),
    Literal(LiteralNode),
}

pub struct BlockNode {
    nodes: Vec<Node>,
}

pub enum LiteralNode {
    Number(f64),
    String(String),
    Boolean(bool),
}
