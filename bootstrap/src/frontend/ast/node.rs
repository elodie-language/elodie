use std::hash::{Hash, Hasher};
use std::rc::Rc;

use crate::common::{PackagePath, StringTableId};
use crate::frontend::lex::token::{LiteralToken, Token, TokenKind};
use crate::frontend::modifier::Modifiers;
use crate::frontend::parse;
use crate::frontend::parse::{IdentifierNode, TupleNode};

#[derive(Debug, PartialEq)]
pub struct BlockNode {
    pub body: Vec<Node>,
}

#[derive(Debug, PartialEq)]
pub struct BreakLoopNode {
    pub token: Token,
    pub body: Option<Box<Node>>,
}

#[derive(Debug, PartialEq)]
pub struct CompareNode {
    pub left: Box<Node>,
    pub operator: CompareOperator,
    pub right: Box<Node>,
}

#[derive(Debug, PartialEq)]
pub enum CompareOperator {
    Equal,
    NotEqual,
    GreaterThan,
}

#[derive(Debug, PartialEq)]
pub struct CalculateNode {
    pub left: Box<Node>,
    pub operator: CalculationOperator,
    pub right: Box<Node>,
}

#[derive(Debug, PartialEq)]
pub enum CalculationOperator {
    Add,
    Multiply,
}

#[derive(Debug, PartialEq)]
pub struct ContinueLoopNode {
    pub token: Token,
}

#[derive(Debug, PartialEq)]
pub struct CallFunctionOfObjectNode {
    pub token: Token,
    pub object: Identifier,
    // FIXME object_tid : TypeId
    pub function: Identifier,
    // FIXME function_tid: TypeId
    pub arguments: Vec<Node>,
}

#[derive(Debug, PartialEq)]
pub struct CallFunctionOfPackageNode {
    pub token: Token,
    pub package: PackagePath,
    pub function: Identifier,
    pub arguments: Vec<Node>,
}

#[derive(Debug, PartialEq)]
pub struct CallFunctionNode {
    pub token: Token,
    pub function: Identifier,
    pub arguments: Vec<Node>,
}

#[derive(Debug, PartialEq)]
pub struct CallFunctionWithLambdaNode {
    pub token: Token,
    pub call_function: CallFunctionNode,
    pub lambda: Rc<BlockNode>,
}

#[derive(Debug, PartialEq)]
pub struct ExportPackageNode {
    pub token: Token,
    pub identifier: Identifier,
    pub source: Source,
}

#[derive(Debug, PartialEq)]
pub struct IfNode {
    pub token: Token,
    pub condition: Box<Node>,
    pub then: BlockNode,
    pub otherwise: Option<BlockNode>,
}

#[derive(Debug, PartialEq)]
pub struct LoopNode {
    pub token: Token,
    pub body: Vec<Node>,
}

#[derive(Debug, PartialEq)]
pub enum Node {
    Block(BlockNode),
    BreakLoop(BreakLoopNode),

    Calculate(CalculateNode),
    CallFunctionOfObject(CallFunctionOfObjectNode),
    CallFunctionOfPackage(CallFunctionOfPackageNode),
    CallFunction(CallFunctionNode),
    CallFunctionWithLambda(CallFunctionWithLambdaNode),

    ExportPackage(ExportPackageNode),
    ReturnFromFunction(ReturnFromFunctionNode),

    ContinueLoop(ContinueLoopNode),
    Compare(CompareNode),

    If(IfNode),
    // Itself(ItselfNode),
    LoadValue(LoadValueNode),
    LoadValueFromObject(LoadValueFromObjectNode),
    LoadValueFromSelf(LoadValueFromSelfNode),
    Loop(LoopNode),

    Literal(LiteralNode),
    Unit,

    DeclareVariable(DeclareVariableNode),
    DeclareFunction(DeclareFunctionNode),
    DeclareExternalFunction(DeclareExternalFunctionNode),
    DeclarePackage(DeclarePackageNode),
    DeclareType(DeclareTypeNode),

    InstantiateType(InstantiateTypeNode),
    DefineType(DefineTypeNode),

    InterpolateString(InterpolateStringNode),
}

#[derive(Debug, PartialEq)]
pub enum LiteralNode {
    Number(LiteralNumberNode),
    String(LiteralStringNode),
    Boolean(LiteralBooleanNode),
}

#[derive(Debug, PartialEq)]
pub struct LiteralNumberNode(pub Token);

impl LiteralNumberNode {
    pub fn value(&self) -> StringTableId {
        self.0.value()
    }
}

#[derive(Debug, PartialEq)]
pub struct LiteralStringNode(pub Token);

impl LiteralStringNode {
    pub fn value(&self) -> StringTableId {
        self.0.value()
    }
}

#[derive(Debug, PartialEq)]
pub struct LiteralBooleanNode(pub Token);

impl LiteralBooleanNode {
    pub fn value(&self) -> bool {
        self.0.kind == TokenKind::Literal(LiteralToken::True)
    }
}

#[derive(Debug, PartialEq)]
pub struct ReturnFromFunctionNode {
    pub token: Token,
    pub node: Box<Node>,
    pub return_type: Option<TypeNode>,
}

#[derive(Debug, PartialEq)]
pub struct LoadValueNode {
    pub token: Token,
    pub identifier: Identifier,
}

#[derive(Clone, Debug)]
pub struct ItselfNode(pub Token);

#[derive(Clone, Debug)]
pub struct Identifier(pub Token);

impl Hash for Identifier {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.value().hash(state)
    }
}

impl Eq for Identifier {}

impl PartialEq for Identifier {
    fn eq(&self, other: &Self) -> bool {
        self.0.value().eq(&other.0.value())
    }
}

impl From<parse::IdentifierNode> for Identifier {
    fn from(value: IdentifierNode) -> Self {
        Identifier(value.0.clone())
    }
}

impl From<Rc<parse::IdentifierNode>> for Identifier {
    fn from(value: Rc<IdentifierNode>) -> Self {
        Identifier(value.0.clone())
    }
}

impl From<&parse::IdentifierNode> for Identifier {
    fn from(value: &IdentifierNode) -> Self {
        Identifier(value.0.clone())
    }
}

impl AsRef<Identifier> for Identifier {
    fn as_ref(&self) -> &Identifier {
        &self
    }
}

#[derive(Debug, PartialEq)]
pub struct DeclareVariableNode {
    pub token: Token,
    pub identifier: Identifier,
    pub value: Rc<Node>,
    pub value_type: Option<TypeNode>,
}

#[derive(Debug, PartialEq)]
pub struct DeclareFunctionNode {
    pub token: Token,
    pub identifier: Identifier,
    pub arguments: Vec<Rc<FunctionArgumentNode>>,
    pub return_type: Option<TypeNode>,
    pub body: Rc<BlockNode>,
}

#[derive(Debug, PartialEq)]
pub struct DeclareExternalFunctionNode {
    pub token: Token,
    pub identifier: Identifier,
    pub arguments: Vec<Rc<FunctionArgumentNode>>,
    pub return_type: Option<TypeNode>,
}

#[derive(Debug, PartialEq)]
pub struct FunctionArgumentNode {
    pub identifier: Identifier,
    pub ty: Option<TypeNode>,
}

#[derive(Debug, PartialEq)]
pub struct DeclarePackageNode {
    pub token: Token,
    pub identifier: Identifier,
    pub modifiers: Modifiers,
    pub external_functions: Vec<DeclareExternalFunctionNode>,
    pub functions: Vec<DeclareFunctionNode>,
    pub packages: Vec<DeclarePackageNode>,
    pub definitions: Vec<DefineTypeNode>,
}

#[derive(Debug, PartialEq)]
pub struct DeclareTypeNode {
    pub token: Token,
    pub identifier: Identifier,
    pub modifiers: Modifiers,
    pub properties: Vec<DeclarePropertyNode>,
}

#[derive(Debug, PartialEq)]
pub struct DeclarePropertyNode {
    pub token: Token,
    pub identifier: Identifier,
    pub r#type: TypeNode,
}

#[derive(Debug, PartialEq)]
pub struct DefineTypeNode {
    pub token: Token,
    pub identifier: Identifier,
    pub modifiers: Modifiers,
    pub functions: Vec<DeclareFunctionNode>,
}

#[derive(Debug, PartialEq)]
pub enum Source {
    LocalFile(SourceLocalFileNode),
}

#[derive(Debug, PartialEq)]
pub struct SourceLocalFileNode {
    pub path: String,
}

// FIXME compiler should give a hint whether the interpolated string will be used locally only and whether it is small enough to be allocated on the stack only
#[derive(Debug, PartialEq)]
pub struct InterpolateStringNode {
    pub token: Token,
    pub nodes: Vec<Node>,
}

#[derive(Debug, PartialEq)]
pub struct InstantiateTypeNode {
    pub token: Token,
    pub type_name: Identifier,
    pub arguments: Vec<NamedArgumentNode>,
}

#[derive(Debug, PartialEq)]
pub struct NamedArgumentNode {
    pub token: Token,
    pub identifier: Identifier,
    pub value: Node,
}

#[derive(Debug, PartialEq)]
pub struct LoadValueFromObjectNode {
    pub token: Token,
    pub object: Identifier,
    pub property: Identifier,
}

#[derive(Debug, PartialEq)]
pub struct LoadValueFromSelfNode {
    pub token: Token,
    pub property: Identifier,
}

#[derive(Debug, PartialEq)]
pub enum TypeNode {
    Boolean(Token),
    Object(ObjectTypeNode),
    Number(Token),
    String(Token),
    Function(TypeFunctionNode),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ObjectTypeNode {
    pub token: Token,
}

#[derive(Debug, PartialEq)]
pub struct TypeFunctionNode {
    pub arguments: Vec<TypeFunctionArgumentNode>,
    pub return_type: Option<Box<TypeNode>>,
}

impl TypeFunctionNode {
    pub fn as_return_type(&self) -> &TypeNode {
        if let Some(ref node) = self.return_type {
            node
        } else {
            panic!()
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct TypeFunctionArgumentNode {
    pub identifier: Option<IdentifierNode>,
    pub r#type: Box<TypeNode>,
}

#[derive(Debug, PartialEq)]
pub struct TypeDeclarationNode {
    pub token: Token,
    pub identifier: IdentifierNode,
    pub properties: TupleNode,
    pub modifiers: Modifiers,
}
