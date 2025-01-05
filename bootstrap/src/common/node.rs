use std::fmt::Debug;
use std::marker::PhantomData;

pub trait Variant: Debug {}

#[derive(Clone, Debug, PartialEq)]
pub enum Node<
    V: Variant,
    AccessVariable: AccessVariableNode<V>,
    AccessVariableOfObject: AccessVariableOfObjectNode<V>,
    AccessVariableOfSelf: AccessVariableOfSelfNode<V>,
    Block: BlockNode<V>,
    BreakLoop: BreakLoopNode<V>,
    Calculate: CalculateNode<V>,
    CallFunction: CallFunctionNode<V>,
    CallFunctionWithLambda: CallFunctionWithLambdaNode<V>,
    CallFunctionOfObject: CallFunctionOfObjectNode<V>,
    CallFunctionOfPackage: CallFunctionOfPackageNode<V>,
    Compare: CompareNode<V>,
    ContinueLoop: ContinueLoopNode<V>,
    DeclareExternalFunction: DeclareExternalFunctionNode<V>,
    DeclareFunction: DeclareFunctionNode<V>,
    DeclarePackage: DeclarePackageNode<V>,
    DeclareType: DeclareTypeNode<V>,
    DeclareVariable: DeclareVariableNode<V>,
    DefineType: DefineTypeNode<V>,
    ExportPackage: ExportPackageNode<V>,
    If: IfNode<V>,
    InterpolateString: InterpolateStringNode<V>,
    InstantiateType: InstantiateTypeNode<V>,
    LiteralBoolean: LiteralBooleanNode<V>,
    LiteralFloat4: LiteralFloat4Node<V>,
    LiteralFloat8: LiteralFloat8Node<V>,
    LiteralInt1: LiteralInt1Node<V>,
    LiteralInt2: LiteralInt2Node<V>,
    LiteralInt4: LiteralInt4Node<V>,
    LiteralInt8: LiteralInt8Node<V>,
    LiteralInt16: LiteralInt16Node<V>,
    LiteralNumber: LiteralNumberNode<V>,
    LiteralString: LiteralStringNode<V>,
    LiteralUint1: LiteralUint1Node<V>,
    LiteralUint2: LiteralUint2Node<V>,
    LiteralUint4: LiteralUint4Node<V>,
    LiteralUint8: LiteralUint8Node<V>,
    LiteralUint16: LiteralUint16Node<V>,
    Loop: LoopNode<V>,
    ReturnFromFunction: ReturnFromFunctionNode<V>,
> {
    AccessVariable(AccessVariable),
    AccessVariableOfObject(AccessVariableOfObject),
    AccessVariableOfSelf(AccessVariableOfSelf),
    Block(Block),
    BreakLoop(BreakLoop),
    Calculate(Calculate),
    CallFunction(CallFunction),
    CallFunctionWithLambda(CallFunctionWithLambda),
    CallFunctionOfObject(CallFunctionOfObject),
    CallFunctionOfPackage(CallFunctionOfPackage),
    Compare(Compare),
    ContinueLoop(ContinueLoop),
    DeclareExternalFunction(DeclareExternalFunction),
    DeclareFunction(DeclareFunction),
    DeclarePackage(DeclarePackage),
    DeclareType(DeclareType),
    DeclareVariable(DeclareVariable),
    DefineType(DefineType),
    ExportPackage(ExportPackage),
    If(If),
    InterpolateString(InterpolateString),
    InstantiateType(InstantiateType),
    LiteralBoolean(LiteralBoolean),
    LiteralFloat4(LiteralFloat4),
    LiteralFloat8(LiteralFloat8),
    LiteralInt1(LiteralInt1),
    LiteralInt2(LiteralInt2),
    LiteralInt4(LiteralInt4),
    LiteralInt8(LiteralInt8),
    LiteralInt16(LiteralInt16),
    LiteralNumber(LiteralNumber),
    LiteralString(LiteralString),
    LiteralUint1(LiteralUint1),
    LiteralUint2(LiteralUint2),
    LiteralUint4(LiteralUint4),
    LiteralUint8(LiteralUint8),
    LiteralUint16(LiteralUint16),
    Loop(Loop),
    ReturnFromFunction(ReturnFromFunction),
    Marker(PhantomData<V>),
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

pub trait LiteralFloat4Node<V: Variant> {}

pub trait LiteralFloat8Node<V: Variant> {}

pub trait LiteralInt1Node<V: Variant> {}

pub trait LiteralInt2Node<V: Variant> {}

pub trait LiteralInt4Node<V: Variant> {}

pub trait LiteralInt8Node<V: Variant> {}

pub trait LiteralInt16Node<V: Variant> {}

pub trait LiteralNumberNode<V: Variant> {}

pub trait LiteralStringNode<V: Variant> {}

pub trait LiteralUint1Node<V: Variant> {}

pub trait LiteralUint2Node<V: Variant> {}

pub trait LiteralUint4Node<V: Variant> {}

pub trait LiteralUint8Node<V: Variant> {}

pub trait LiteralUint16Node<V: Variant> {}

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
    GreaterThanEqual,
    LessThan,
    LessThanEqual
}

#[derive(Debug, Clone, PartialEq)]
pub enum Source {
    LocalFile { path: String },
}
