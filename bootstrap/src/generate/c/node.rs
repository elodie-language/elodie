pub enum Node {
    Block(),
    DeclareFunction(DeclareFunctionNode),
}

#[derive(Debug)]
pub struct BlockNode {}

#[derive(Debug)]
pub struct DeclareFunctionNode {
    pub identifier: String,
    pub arguments: Vec<DeclareFunctionArgumentNode>,
    pub ty: String,
}

#[derive(Debug)]
pub struct DeclareFunctionArgumentNode {
    pub identifier: String,
    pub ty: String,
}