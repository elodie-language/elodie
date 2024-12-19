use std::rc::Rc;

use crate::common::StringTableId;
use crate::frontend::lex::token::{LiteralToken, Token, TokenKind};
use crate::frontend::modifier::Modifiers;

#[derive(Debug, PartialEq)]
pub(crate) enum Node {
    Block(BlockNode),
    Break(BreakNode),
    Call(CallNode),
    Continue(ContinueNode),
    From(FromNode),
    ExternalFunctionDeclaration(ExternalFunctionDeclarationNode),
    FunctionDeclaration(FunctionDeclarationNode),
    DefineDeclaration(DefineDeclarationNode),
    Identifier(IdentifierNode),
    If(IfNode),
    Infix(InfixNode),
    Itself(ItselfNode),
    Literal(LiteralNode),
    Loop(LoopNode),
    Nop,
    PackageDeclaration(PackageDeclarationNode),
    Prefix(PrefixNode),
    Return(ReturnNode),
    StringInterpolation(StringInterpolationNode),
    Tuple(TupleNode),
    Type(TypeNode),
    TypeDeclaration(TypeDeclarationNode),
    VariableDeclaration(VariableDeclarationNode),
}

impl Node {
    pub(crate) fn token(&self) -> Token {
        match self {
            Node::Block(n) => n.token.clone(),
            Node::Break(n) => n.token.clone(),
            Node::Call(n) => n.token.clone(),
            Node::Continue(n) => n.token.clone(),
            Node::From(n) => match n {
                FromNode::Export(n) => n.token.clone(),
            },
            Node::ExternalFunctionDeclaration(n) => n.token.clone(),
            Node::FunctionDeclaration(n) => n.token.clone(),
            Node::DefineDeclaration(n) => n.token.clone(),
            Node::Identifier(n) => n.0.clone(),
            Node::If(n) => n.token.clone(),
            Node::Infix(n) => n.token.clone(),
            Node::Itself(n) => n.0.clone(),
            Node::Literal(n) => match n {
                LiteralNode::Number(n) => n.0.clone(),
                LiteralNode::String(n) => n.0.clone(),
                LiteralNode::Boolean(n) => n.0.clone(),
            },
            Node::Loop(n) => n.token.clone(),
            Node::Nop => unimplemented!(),
            Node::PackageDeclaration(n) => n.token.clone(),
            Node::Prefix(n) => match &n.operator {
                PrefixOperator::Plus(t) => t.clone(),
                PrefixOperator::Negate(t) => t.clone(),
                PrefixOperator::Not(t) => t.clone(),
            },
            Node::Return(n) => n.token.clone(),
            Node::StringInterpolation(n) => n.token.clone(),
            Node::Tuple(n) => n.token.clone(),
            Node::Type(n) => match n {
                TypeNode::Boolean(t) => t.clone(),
                TypeNode::Object(n) => n.token.clone(),
                TypeNode::Number(t) => t.clone(),
                TypeNode::String(t) => t.clone(),
                TypeNode::Function(n) => n.token.clone(),
            },
            Node::TypeDeclaration(n) => n.token.clone(),
            Node::VariableDeclaration(n) => n.token.clone(),
        }
    }
}

impl Node {
    pub(crate) fn is_block(&self) -> bool {
        matches!(self, Node::Block(_))
    }
    pub(crate) fn as_block(&self) -> &BlockNode {
        if let Node::Block(result) = self {
            result
        } else {
            panic!("not block")
        }
    }

    pub(crate) fn is_break(&self) -> bool {
        matches!(self, Node::Break(_))
    }
    pub(crate) fn as_break(&self) -> &BreakNode {
        if let Node::Break(result) = self {
            result
        } else {
            panic!("not break")
        }
    }

    pub(crate) fn is_call(&self) -> bool {
        matches!(self, Node::Call(_))
    }
    pub(crate) fn as_call(&self) -> &CallNode {
        if let Node::Call(result) = self {
            result
        } else {
            panic!("not call")
        }
    }

    pub(crate) fn is_continue(&self) -> bool {
        matches!(self, Node::Continue(_))
    }
    pub(crate) fn as_continue(&self) -> &ContinueNode {
        if let Node::Continue(result) = self {
            result
        } else {
            panic!("not continue")
        }
    }

    pub(crate) fn is_define_declaration(&self) -> bool {
        matches!(self, Node::DefineDeclaration(_))
    }
    pub(crate) fn as_define_declaration(&self) -> &DefineDeclarationNode {
        if let Node::DefineDeclaration(result) = self {
            result
        } else {
            panic!("not define declaration")
        }
    }

    pub(crate) fn is_from(&self) -> bool {
        matches!(self, Node::From(_))
    }
    pub(crate) fn as_from(&self) -> &FromNode {
        if let Node::From(result) = self {
            result
        } else {
            panic!("not from")
        }
    }

    pub(crate) fn is_external_function_declaration(&self) -> bool {
        matches!(self, Node::ExternalFunctionDeclaration(_))
    }
    pub(crate) fn as_external_function_declaration(&self) -> &ExternalFunctionDeclarationNode {
        if let Node::ExternalFunctionDeclaration(result) = self {
            result
        } else {
            panic!("not external function declaration")
        }
    }

    pub(crate) fn is_function_declaration(&self) -> bool {
        matches!(self, Node::FunctionDeclaration(_))
    }
    pub(crate) fn as_function_declaration(&self) -> &FunctionDeclarationNode {
        if let Node::FunctionDeclaration(result) = self {
            result
        } else {
            panic!("not function declaration")
        }
    }

    pub(crate) fn is_identifier(&self) -> bool {
        matches!(self, Node::Identifier(_))
    }
    pub(crate) fn as_identifier(&self) -> &IdentifierNode {
        if let Node::Identifier(result) = self {
            result
        } else {
            panic!("not identifier")
        }
    }

    pub(crate) fn is_if(&self) -> bool {
        matches!(self, Node::If(_))
    }
    pub(crate) fn as_if(&self) -> &IfNode {
        if let Node::If(result) = self {
            result
        } else {
            panic!("not if")
        }
    }

    pub(crate) fn is_infix(&self) -> bool {
        matches!(self, Node::Infix(_))
    }
    pub(crate) fn as_infix(&self) -> &InfixNode {
        if let Node::Infix(result) = self {
            result
        } else {
            panic!("not infix")
        }
    }

    pub(crate) fn is_declare_variable(&self) -> bool {
        matches!(self, Node::VariableDeclaration(_))
    }
    pub(crate) fn as_declare_variable(&self) -> &VariableDeclarationNode {
        if let Node::VariableDeclaration(result) = self {
            result
        } else {
            panic!("not let")
        }
    }

    pub(crate) fn is_literal(&self) -> bool {
        matches!(self, Node::Literal(_))
    }
    pub(crate) fn as_literal(&self) -> &LiteralNode {
        if let Node::Literal(result) = self {
            result
        } else {
            panic!("not literal")
        }
    }

    pub(crate) fn is_loop(&self) -> bool {
        matches!(self, Node::Loop(_))
    }
    pub(crate) fn as_loop(&self) -> &LoopNode {
        if let Node::Loop(result) = self {
            result
        } else {
            panic!("not loop")
        }
    }

    pub(crate) fn is_package_declaration(&self) -> bool {
        matches!(self, Node::PackageDeclaration(_))
    }
    pub(crate) fn as_package_declaration(&self) -> &PackageDeclarationNode {
        if let Node::PackageDeclaration(result) = self {
            result
        } else {
            panic!("not package declaration")
        }
    }

    pub(crate) fn is_prefix(&self) -> bool {
        matches!(self, Node::Prefix(_))
    }
    pub(crate) fn as_prefix(&self) -> &PrefixNode {
        if let Node::Prefix(result) = self {
            result
        } else {
            panic!("not prefix")
        }
    }

    pub(crate) fn is_return(&self) -> bool {
        matches!(self, Node::Return(_))
    }
    pub(crate) fn as_return(&self) -> &ReturnNode {
        if let Node::Return(result) = self {
            result
        } else {
            panic!("not return")
        }
    }

    pub(crate) fn is_itself(&self) -> bool {
        matches!(self, Node::Itself(_))
    }
    pub(crate) fn as_itself(&self) -> &ItselfNode {
        if let Node::Itself(result) = self {
            result
        } else {
            panic!("not itself")
        }
    }

    pub(crate) fn is_tuple(&self) -> bool {
        matches!(self, Node::Tuple(_))
    }
    pub(crate) fn as_tuple(&self) -> &TupleNode {
        if let Node::Tuple(result) = self {
            result
        } else {
            panic!("not tuple")
        }
    }

    pub(crate) fn is_type(&self) -> bool {
        matches!(self, Node::Type(_))
    }
    pub(crate) fn as_type(&self) -> &TypeNode {
        if let Node::Type(result) = self {
            result
        } else {
            panic!("not type")
        }
    }

    pub(crate) fn is_type_declaration(&self) -> bool {
        matches!(self, Node::TypeDeclaration(_))
    }
    pub(crate) fn as_type_declaration(&self) -> &TypeDeclarationNode {
        if let Node::TypeDeclaration(result) = self {
            result
        } else {
            panic!("not type declaration")
        }
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct BlockNode {
    pub(crate) token: Token,
    pub(crate) nodes: Vec<Node>,
}

#[derive(Debug, PartialEq)]
pub(crate) struct BreakNode {
    pub(crate) token: Token,
    pub(crate) result: Option<Box<Node>>,
}

impl BreakNode {
    pub(crate) fn as_result(&self) -> &Node {
        if let Some(ref node) = self.result {
            node
        } else {
            panic!()
        }
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct CallNode {
    pub(crate) token: Token,
    pub(crate) callee: Box<Node>,
    pub(crate) arguments: Vec<CallArgument>,
}

#[derive(Debug, PartialEq)]
pub(crate) struct CallArgument {
    pub(crate) identifier: Option<IdentifierNode>,
    pub(crate) node: Box<Node>,
}

#[derive(Debug, PartialEq)]
pub(crate) struct ContinueNode {
    pub(crate) token: Token,
}

#[derive(Debug, PartialEq)]
pub(crate) struct ExternalFunctionDeclarationNode {
    pub(crate) token: Token,
    pub(crate) identifier: IdentifierNode,
    pub(crate) arguments: Vec<FunctionDeclarationArgumentNode>,
    pub(crate) return_type: Option<Box<TypeNode>>,
    pub(crate) modifiers: Modifiers,
}

#[derive(Debug, PartialEq)]
pub(crate) struct FromExportNode {
    pub(crate) token: Token,
    pub(crate) from_node: Box<Node>,
    pub(crate) what_node: Box<Node>,
}

#[derive(Debug, PartialEq)]
pub(crate) enum FromNode {
    Export(FromExportNode),
}

#[derive(Debug, PartialEq)]
pub(crate) struct FunctionDeclarationNode {
    pub(crate) token: Token,
    pub(crate) identifier: IdentifierNode,
    pub(crate) arguments: Vec<FunctionDeclarationArgumentNode>,
    pub(crate) return_type: Option<Box<TypeNode>>,
    pub(crate) block: BlockNode,
    pub(crate) modifiers: Modifiers,
}

impl FunctionDeclarationNode {
    pub(crate) fn as_return_type(&self) -> &TypeNode {
        if let Some(ref node) = self.return_type {
            node
        } else {
            panic!()
        }
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct FunctionDeclarationArgumentNode {
    pub(crate) identifier: IdentifierNode,
    pub(crate) r#type: Option<Box<TypeNode>>,
}

impl FunctionDeclarationArgumentNode {
    pub(crate) fn as_type(&self) -> &TypeNode {
        if let Some(ref node) = self.r#type {
            node
        } else {
            panic!()
        }
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct IdentifierNode(pub(crate) Token);

impl IdentifierNode {
    pub(crate) fn value(&self) -> StringTableId {
        self.0.span.value
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct ItselfNode(pub(crate) Token);

impl ItselfNode {
    pub(crate) fn value(&self) -> StringTableId {
        self.0.span.value
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct IfNode {
    pub(crate) token: Token,
    pub(crate) condition: Box<Node>,
    pub(crate) then: BlockNode,
    pub(crate) otherwise: Option<ElseNode>,
}

#[derive(Debug, PartialEq)]
pub(crate) struct ElseNode {
    pub(crate) token: Token,
    pub(crate) block: BlockNode,
}

#[derive(Debug, PartialEq)]
pub(crate) struct InfixNode {
    pub(crate) token: Token,
    pub(crate) left: Box<Node>,
    pub(crate) operator: InfixOperator,
    pub(crate) right: Box<Node>,
}

#[derive(Debug, PartialEq)]
pub(crate) enum InfixOperator {
    Add(Token),
    Arrow(Token),
    AccessPackage(Token),
    AccessProperty(Token),
    Assign(Token),
    Call(Token),
    Subtract(Token),
    Multiply(Token),
    Divide(Token),
    Modulo(Token),
    Equal(Token),
    NotEqual(Token),
    LambdaCall(Token),
    LessThan(Token),
    LessThanOrEqual(Token),
    GreaterThan(Token),
    GreaterThanOrEqual(Token),
    TypeAscription(Token),
}

impl InfixOperator {
    pub(crate) fn token(&self) -> Token {
        match self {
            InfixOperator::Add(t) => t.clone(),
            InfixOperator::Arrow(t) => t.clone(),
            InfixOperator::AccessPackage(t) => t.clone(),
            InfixOperator::AccessProperty(t) => t.clone(),
            InfixOperator::Assign(t) => t.clone(),
            InfixOperator::Call(t) => t.clone(),
            InfixOperator::Subtract(t) => t.clone(),
            InfixOperator::Multiply(t) => t.clone(),
            InfixOperator::Divide(t) => t.clone(),
            InfixOperator::Modulo(t) => t.clone(),
            InfixOperator::Equal(t) => t.clone(),
            InfixOperator::NotEqual(t) => t.clone(),
            InfixOperator::LambdaCall(t) => t.clone(),
            InfixOperator::LessThan(t) => t.clone(),
            InfixOperator::LessThanOrEqual(t) => t.clone(),
            InfixOperator::GreaterThan(t) => t.clone(),
            InfixOperator::GreaterThanOrEqual(t) => t.clone(),
            InfixOperator::TypeAscription(t) => t.clone(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct VariableDeclarationNode {
    pub(crate) token: Token,
    pub(crate) identifier: IdentifierNode,
    pub(crate) node: Rc<Node>,
    pub(crate) r#type: Option<TypeNode>,
}

#[derive(Debug, PartialEq)]
pub(crate) enum LiteralNode {
    Number(LiteralNumberNode),
    String(LiteralStringNode),
    Boolean(LiteralBooleanNode),
}

#[derive(Debug, PartialEq)]
pub(crate) struct LiteralNumberNode(pub(crate) Token);

impl LiteralNumberNode {
    pub(crate) fn value(&self) -> StringTableId {
        self.0.value()
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct LiteralStringNode(pub(crate) Token);

impl LiteralStringNode {
    pub(crate) fn value(&self) -> StringTableId {
        self.0.value()
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct LiteralBooleanNode(pub(crate) Token);

impl LiteralBooleanNode {
    pub(crate) fn value(&self) -> bool {
        self.0.kind == TokenKind::Literal(LiteralToken::True)
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct LoopNode {
    pub(crate) token: Token,
    pub(crate) block: BlockNode,
}

#[derive(Debug, PartialEq)]
pub(crate) struct PackageDeclarationNode {
    pub(crate) token: Token,
    pub(crate) identifier: IdentifierNode,
    pub(crate) block: BlockNode,
    pub(crate) modifiers: Modifiers,
}

#[derive(Debug, PartialEq)]
pub(crate) struct DefineDeclarationNode {
    pub(crate) token: Token,
    pub(crate) identifier: IdentifierNode,
    pub(crate) block: BlockNode,
    pub(crate) modifiers: Modifiers,
}

#[derive(Debug, PartialEq)]
pub(crate) struct PrefixNode {
    pub(crate) operator: PrefixOperator,
    pub(crate) node: Box<Node>,
}

#[derive(Debug, PartialEq)]
pub(crate) struct ReturnNode {
    pub(crate) token: Token,
    pub(crate) result: Option<Box<Node>>,
}

impl ReturnNode {
    pub(crate) fn as_result(&self) -> &Node {
        if let Some(ref node) = self.result {
            node
        } else {
            panic!()
        }
    }
}

#[derive(Debug, PartialEq)]
pub(crate) enum PrefixOperator {
    Plus(Token),
    Negate(Token),
    Not(Token),
}

#[derive(Debug, PartialEq)]
pub(crate) struct StringInterpolationNode {
    pub(crate) token: Token,
    pub(crate) nodes: Vec<Node>,
}

#[derive(Debug, PartialEq)]
pub(crate) struct TupleNode {
    pub(crate) token: Token,
    pub(crate) nodes: Vec<Node>,
}

#[derive(Debug, PartialEq)]
pub(crate) enum TypeNode {
    Boolean(Token),
    Object(ObjectTypeNode),
    Number(Token),
    String(Token),
    Function(TypeFunctionNode),
}

#[derive(Debug, PartialEq)]
pub(crate) struct ObjectTypeNode {
    pub(crate) token: Token,
}

#[derive(Debug, PartialEq)]
pub(crate) struct TypeFunctionNode {
    pub(crate) token: Token,
    pub(crate) arguments: Vec<TypeFunctionArgumentNode>,
    pub(crate) return_type: Option<Box<TypeNode>>,
}

impl TypeFunctionNode {
    pub(crate) fn as_return_type(&self) -> &TypeNode {
        if let Some(ref node) = self.return_type {
            node
        } else {
            panic!()
        }
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct TypeFunctionArgumentNode {
    pub(crate) identifier: Option<IdentifierNode>,
    pub(crate) r#type: Box<TypeNode>,
}

#[derive(Debug, PartialEq)]
pub(crate) struct TypeDeclarationNode {
    pub(crate) token: Token,
    pub(crate) identifier: IdentifierNode,
    pub(crate) properties: TupleNode,
    pub(crate) modifiers: Modifiers,
}
