use std::fmt;
use std::fmt::Debug;
use std::ops::Deref;
use std::rc::Rc;

use crate::common::{Span, WithSpan};

pub trait Variant: Debug {}

pub trait Tree<V: Variant, T: Tree<V, T>> {
    fn node(&self) -> &Node<V>;
    fn node_mut(&mut self) -> &mut Node<V>;
    fn node_to_owned(self) -> Node<V>;
}

#[derive(Debug, Clone, PartialEq)]
pub struct TreeNode<V: Variant> {
    node: Node<V>,
    span: Span,
}

impl<V: Variant> Debug for Node<V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl<V: Variant> Clone for Node<V>{
    fn clone(&self) -> Self {
        todo!()
    }
}

impl<V: Variant> PartialEq for Node<V>{
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl<V: Variant> WithSpan for TreeNode<V> {
    fn span(&self) -> Span { self.span.clone() }
}

impl<V: Variant> Deref for TreeNode<V> {
    type Target = Node<V>;
    fn deref(&self) -> &Self::Target {
        &self.node
    }
}

impl<V: Variant> TreeNode<V> {
    pub fn new(node: Node<V>, span: Span) -> TreeNode<V> {
        TreeNode { node, span }
    }
}

impl<V: Variant, T: Tree<V, T>> Tree<V, T> for TreeNode<V> {
    fn node(&self) -> &Node<V> { todo!()}
    fn node_mut(&mut self) -> &mut Node<V> { &mut self.node }
    fn node_to_owned(self) -> Node<V> { self.node }
}

pub enum Node<V: Variant> {
    AccessVariable(Rc<dyn AccessVariableNode<V>>),
    AccessVariableOfObject(Rc<dyn AccessVariableOfObjectNode<V>>),
    AccessVariableOfSelf(Rc<dyn AccessVariableOfSelfNode<V>>),
    Block(Rc<dyn BlockNode<V>>),
    BreakLoop(Rc<dyn BreakLoopNode<V>>),
    Calculate(Rc<dyn CalculateNode<V>>),
    CallFunction(Rc<dyn CallFunctionNode<V>>),
    CallFunctionWithLambda(Rc<dyn CallFunctionWithLambdaNode<V>>),
    CallFunctionOfObject(Rc<dyn CallFunctionOfObjectNode<V>>),
    CallFunctionOfPackage(Rc<dyn CallFunctionOfPackageNode<V>>),
    Compare(Rc<dyn CompareNode<V>>),
    ContinueLoop(Rc<dyn ContinueLoopNode<V>>),
    DeclareExternalFunction(Rc<dyn DeclareExternalFunctionNode<V>>),
    DeclareFunction(Rc<dyn DeclareFunctionNode<V>>),
    DeclarePackage(Rc<dyn DeclarePackageNode<V>>),
    DeclareType(Rc<dyn DeclareTypeNode<V>>),
    DeclareVariable(Rc<dyn DeclareVariableNode<V>>),
    DefineType(Rc<dyn DefineTypeNode<V>>),
    ExportPackage(Rc<dyn ExportPackageNode<V>>),
    If(Rc<dyn IfNode<V>>),
    InterpolateString(Rc<dyn InterpolateStringNode<V>>),
    InstantiateType(Rc<dyn InstantiateTypeNode<V>>),
    LiteralBoolean(Rc<dyn LiteralBooleanNode<V>>),
    LiteralNumber(Rc<dyn LiteralNumberNode<V>>),
    LiteralString(Rc<dyn LiteralStringNode<V>>),
    Loop(Rc<dyn LoopNode<V>>),
    ReturnFromFunction(Rc<dyn ReturnFromFunctionNode<V>>),
}

pub trait AccessVariableNode<V: Variant> {}

pub trait AccessVariableOfObjectNode<V: Variant> {}

pub trait AccessVariableOfSelfNode<V: Variant> {}

pub trait BlockNode<V: Variant> {}

pub trait BreakLoopNode<V: Variant> {}

pub trait CalculateNode<V: Variant> {}

pub trait CallFunctionNode<V: Variant> {}

pub trait CallFunctionWithLambdaNode<V: Variant> {}

pub trait CallFunctionOfObjectNode<V: Variant> {}

pub trait CallFunctionOfPackageNode<V: Variant> {}

pub trait CompareNode<V: Variant> {}

pub trait ContinueLoopNode<V: Variant> {}

pub trait DeclareExternalFunctionNode<V: Variant> {}

pub trait DeclareFunctionNode<V: Variant> {}

pub trait DeclarePackageNode<V: Variant> {}

pub trait DeclareTypeNode<V: Variant> {}

pub trait DeclareVariableNode<V: Variant> {}

pub trait DefineTypeNode<V: Variant> {}

pub trait ExportPackageNode<V: Variant> {}

pub trait IfNode<V: Variant> {}

pub trait InterpolateStringNode<V: Variant> {}

pub trait InstantiateTypeNode<V: Variant> {}

pub trait LiteralBooleanNode<V: Variant> {}

pub trait LiteralNumberNode<V: Variant> {}

pub trait LiteralStringNode<V: Variant> {}

pub trait LoopNode<V: Variant> {}

pub trait ReturnFromFunctionNode<V: Variant> {}

#[derive(Debug, Clone, PartialEq)]
pub enum CalculationOperator {
    Add,
    Multiply,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CompareOperator {
    Equal,
    NotEqual,
    GreaterThan,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Source {
    LocalFile { path: String },
}

//
// pub struct Ir {}
//
// impl Variant for Ir {}
//
// pub struct Ast {}
//
// impl Variant for Ast {}
//
// pub struct AstAccessVariableNode {}
//
// impl AccessVariableNode<Ast> for AstAccessVariableNode {}
//
// pub struct IrAccessVariableNode {}
//
// impl AccessVariableNode<Ir> for IrAccessVariableNode {}
//
//
//
// pub struct AstBlockNode {
//     pub nodes: Vec<Node<Ast>>,
// }
//
// impl BlockNode<Ast> for AstBlockNode {
//     fn nodes(&self) -> &[Node<Ast>] {
//         todo!()
//     }
// }
//
// pub struct IrBlockNode {
//     pub nodes: Vec<Node<Ir>>,
// }
//
// impl BlockNode<Ir> for IrBlockNode {
//     fn nodes(&self) -> &[Node<Ir>] {
//         todo!()
//     }
// }
//
// #[test]
// fn test() {
//     let n : TreeNode<Ir> = TreeNode::new(Node::Block(
//         Rc::new(IrBlockNode {
//             nodes: vec![
//                 Node::Block(Rc::new(IrBlockNode { nodes: vec![] }))
//             ]
//         })
//     ), SPAN_NOT_IMPLEMENTED.clone());
//
// }