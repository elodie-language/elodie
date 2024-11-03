#[derive(Debug, PartialEq, Clone)]
pub struct ElodieFile {
    pub imports: Vec<Import>,
    // exports
    pub declarations: Vec<Declaration>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Import {
    pub names: Vec<String>,
    pub is_wildcard: bool,
    pub alias: Option<String>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Declaration(Declaration),
    Expression(Expression),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Declaration {
    Entity(EntityDeclaration),
    EnumEntry(EnumEntryDeclaration),
    Function(FunctionDeclaration),
    TypeAlias(TypeAliasDeclaration),
}

#[derive(Debug, PartialEq, Clone)]
pub struct EntityDeclaration {
    pub modifiers: Vec<Modifier>,
    pub kind: EntityDeclarationKind,
    pub name: String,
    pub type_params: Vec<TypeParam>,
    pub bounds: Vec<TypeBound>,
    pub inner: Vec<Declaration>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum EntityDeclarationKind {
    Type,
    Trait,
    Object,
    Enum,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionDeclaration {
    pub modifiers: Vec<Modifier>,
    pub type_params: Vec<TypeParam>,
    pub receiver: Option<Type>,
    pub name: Option<String>,
    pub params: Vec<Param>,
    pub return_typepe: Option<Type>,
    pub bounds: Vec<TypeBound>,
    pub body: Option<Block>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Block {
    pub statements: Vec<Statement>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum PropertyAccessor {
    Getter {
        modifiers: Vec<Modifier>,
        return_type: Option<Type>,
        body: Option<Block>,
    },
    Setter {
        modifiers: Vec<Modifier>,
        field: PropertySetterField,
        body: Option<Block>,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub struct PropertySetterField {
    pub name: String,
    pub r#type: Option<Type>,
    pub return_type: Option<Type>,
    pub body: Option<Block>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TypeAliasDeclaration {
    pub modifiers: Vec<Modifier>,
    pub name: String,
    pub type_params: Vec<TypeParam>,
    pub r#type: Type,
}

#[derive(Debug, PartialEq, Clone)]
pub struct EnumEntryDeclaration {
    pub modifiers: Vec<Modifier>,
    pub name: String,
    pub args: Vec<CallArg>,
    pub inner: Vec<Declaration>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Literal(Literal),
    ArrayAccess(ArrayAccessExpression),
    BinaryOp(BinaryOperation),
    Break(BreakExpression),
    Call(CallExpression),
    Continue(ContinueExpression),
    For(ForExpression),
    If(IfExpression),
    Lambda(LambdaBlock),
    Loop(LoopExpression),
    Match(MatchExpression),
    Object(ObjectExpression),
    Parenthesized(ParenthesizedExpression),
    Return(ReturnExpression),
    StringTemplate(StringTemplateExpression),
    r#Self(SelfExpression),
    UnaryOp(UnaryOperation),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    UnsignedInteger(u64),
    Integer(i64),
    Decimal(f64),
    String(String),
    Char(char),
    Boolean(bool),
    Null,
}

#[derive(Debug, PartialEq, Clone)]
pub struct IfExpression {
    pub expr: Box<Expression>,
    pub then: Box<Expression>,
    pub otherwise: Option<Box<Expression>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ForExpression {
    pub vars: Tuple,
    pub iterable: Box<Expression>,
    pub body: Box<Expression>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct LoopExpression {
    pub body: Box<Expression>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct BinaryOperation {
    pub lhs: Box<Expression>,
    pub op: BinaryOperator,
    pub rhs: Box<Expression>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum BinaryOperator {
    Operator(BinaryOp),
    Infix(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum BinaryOp {
    Assign,
    AddAssign,
    SubtractAssign,
    MultiplyAssign,
    DivideAssign,
    ModuloAssign,
    Equal,
    NotEqual,
    ReferenceEqual,
    ReferenceNotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    And,
    Or,
    In,
    NotIn,
    Is,
    IsNot,
    RangeTo,
    RangeUntil,
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    As,
    AsNullable,
    Elvis,
    Dot,
    DotSafe,
}

#[derive(Debug, PartialEq, Clone)]
pub struct UnaryOperation {
    pub op: UnaryOperator,
    pub expr: Box<Expression>,
    pub is_prefix: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub enum UnaryOperator {
    Plus,
    Minus,
    Increment,
    Decrement,
    Not,
    NullDeref,
}

#[derive(Debug, PartialEq, Clone)]
pub struct SelfExpression {
    pub label: Option<String>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct MatchExpression {
    pub expr: Option<Box<Expression>>,
    pub clauses: Vec<MatchClause>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct MatchClause {
    pub exprs: Vec<Expression>,
    pub body: Box<Expression>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ObjectExpression {
    pub extends: Vec<EntityDeclaration>,
    pub inner: Vec<Declaration>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ParenthesizedExpression {
    pub expr: Box<Expression>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ReturnExpression {
    pub label: Option<String>,
    pub expr: Option<Box<Expression>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ContinueExpression {
    pub label: Option<String>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct BreakExpression {
    pub label: Option<String>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CallExpression {
    pub expr: Box<Expression>,
    pub args: Vec<CallArg>,
    pub type_args: Vec<Type>,
    pub lambda: Option<Box<Expression>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct LambdaBlock {
    pub label: Option<String>,
    pub vars: Tuple,
    pub body: Option<Block>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ArrayAccessExpression {
    pub expr: Box<Expression>,
    pub index: Vec<Expression>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum StringTemplateExpression {
    Simple(String),
    Block(Block),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Simple(Box<SimpleType>),
    Function(Box<FunctionType>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct SimpleType {
    pub name: Option<String>,
    pub type_args: Vec<Type>,
    pub is_nullable: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionType {
    pub receiver: Option<Type>,
    pub params: Vec<AnonymousParam>,
    pub return_type: Type,
    pub is_nullable: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub struct AnonymousParam {
    pub name: Option<String>,
    pub r#type: Type,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Param {
    pub name: String,
    pub r#type: Type,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TypeParam {
    pub name: String,
    pub bounds: Vec<TypeBound>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TypeBound {
    pub r#type: Type,
    pub kind: BoundKind,
}

#[derive(Debug, PartialEq, Clone)]
pub enum BoundKind {
    Unconstrained,
    Covariant,
    Contravariant,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CallArg {
    pub name: Option<String>,
    pub value: Box<Expression>,
    pub is_spread: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Tuple {
    pub is_destructured: bool,
    pub vars: Vec<VarDefinition>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct VarDefinition {
    pub name: String,
    pub r#type: Option<Type>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum AnnotationSite {
    Field,
    Property,
    Get,
    Set,
    Receiver,
    Param,
    SetParam,
    Delegate,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Modifier {
    Const,
}
