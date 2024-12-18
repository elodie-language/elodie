// use std::collections::HashMap;
// use std::ops::Index;
//
// pub use node::*;
//
// use crate::common::{Context, StringTableId};
// use crate::frontend::{parse, Ast};
// use crate::ir::infer::node::Node;
//
// mod node;
// mod literal;
// mod declare;
// mod r#type;
//
// #[derive(Debug, Clone, PartialEq)]
// pub enum InferredType {
//     Unknown,
//
//     Boolean,
//     Number,
//     String,
//     Tuple(Vec<InferredType>),
//     Type(HashMap<StringTableId, InferredType>),
//
//     OneOf(Vec<InferredType>),
//     AllOf(Vec<InferredType>),
//
// }
//
// impl InferredType {}
//
// #[derive(Debug)]
// pub enum Error {}
//
// pub(crate) type Result<T, E = Error> = core::result::Result<T, E>;
//
// #[derive(Debug)]
// pub struct Inferred<'a> {
//     pub nodes: Vec<Node<'a>>,
// }
//
// impl<'a> Index<usize> for Inferred<'a> {
//     type Output = Node<'a>;
//     fn index(&self, index: usize) -> &Self::Output {
//         self.nodes.index(index)
//     }
// }
//
// pub(crate) fn infer<'a>(ctx: &'a mut Context, parsed: &'a mut Ast) -> Result<Inferred<'a>> {
//     Ok(Inferred { nodes: Inference::new(ctx, parsed).infer()? })
// }
//
// struct Inference<'a> {
//     ctx: &'a Context,
//     parsed: &'a Ast,
// }
//
// impl<'a> Inference<'a> {
//     fn new(ctx: &'a mut Context, parsed: &'a Ast) -> Self {
//         Self { ctx, parsed }
//     }
//
//     fn infer(&mut self) -> Result<Vec<Node<'a>>> {
//         let mut nodes = vec![];
//         for x in &self.parsed.nodes {
//             nodes.push(self.infer_node(x)?);
//         }
//         Ok(nodes)
//     }
//
//     fn infer_node(&self, node: &'a parse::Node) -> Result<Node<'a>> {
//         match node {
//             parse::Node::VariableDeclaration(node) => self.infer_declare_variable(node),
//             parse::Node::Literal(node) => self.infer_literal(node),
//             _ => unimplemented!("{node:#?}")
//         }
//     }
// }