use std::collections::HashSet;

use Node::{AccessVariable, LiteralBoolean, LiteralNumber, LiteralString};

use crate::build::c;
use crate::build::c::{BlockStatement, CodeStatement, DeclareFunctionNode, DeclareStructNode, DefineFunctionNode, DefineStructNode, DirectiveNode, IncludeLocalDirectiveNode, IncludeSystemDirectiveNode, Statement};
use crate::build::c::DirectiveNode::{IncludeLocalDirective, IncludeSystemDirective};
use crate::build::c::generator::stack::Stack;
use crate::build::c::Node::DefineFunction;
use crate::common::{Context, SymbolTable, TypeTable};
use crate::common::node::Node;
use crate::common::node::Node::{CallFunctionOfPackage, DeclareVariable, InterpolateString};
use crate::common::StringTable;
use crate::ir;
use crate::ir::node::IrTreeNode;

mod call;
mod literal;
mod variable;
mod string;
mod stack;
mod rc;

#[derive(Debug)]
pub enum Error {}

type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone, Copy)]
pub(crate) struct FunctionPointer(usize);

pub(crate) fn generate(ctx: Context, ir: ir::Ir) -> Result<Vec<c::Node>> {
    let mut generator = Generator {
        string_table: ctx.string_table,
        symbol_table: ctx.symbol_table,
        type_table: ctx.type_table,
        stack: Stack::new(),

        directives: HashSet::new(),

        function_pointer: FunctionPointer(0),
        function_declarations: Vec::new(),
        function_definitions: Vec::new(),

        struct_definitions: Vec::new(),
        struct_declarations: Vec::new(),

    };
    generator.generate(ir.nodes)
}

pub(crate) struct Generator {
    string_table: StringTable,
    symbol_table: SymbolTable,
    type_table: TypeTable,
    stack: Stack,

    directives: HashSet<DirectiveNode>,
    function_declarations: Vec<DeclareFunctionNode>,
    function_pointer: FunctionPointer,
    function_definitions: Vec<DefineFunctionNode>,
    struct_declarations: Vec<DeclareStructNode>,
    struct_definitions: Vec<DefineStructNode>,
}

impl Generator {
    pub(crate) fn generate(mut self, nodes: Vec<IrTreeNode>) -> Result<Vec<c::Node>> {
        self.function_definitions.push(DefineFunctionNode {
            identifier: "main".to_string(),
            arguments: vec![].into_boxed_slice(),
            ty: "int".to_string(),
            statements: BlockStatement {
                statements: vec![
                    c::Statement::Code(
                        CodeStatement {
                            code: r#"
auto tm = mem_test_new_default (1024 * 1024 );
                            "#.to_string(),
                        }
                    )
                ],
            },
        });

        for node in &nodes {
            self.nodes(node)?
        }

        self.scope_leave();

        self.include_system("stdio.h");
        self.include_system("stdbool.h");
        self.include_local("core/core-api.h");
        self.include_local("core/string/string-api.h");

        let mut result = vec![];
        result.extend(self.directives.into_iter().map(|d| c::Node::Directive(d)));

        result.extend(
            self.struct_declarations
                .into_iter()
                .map(|ds| c::Node::DeclareStruct(ds)),
        );
        result.extend(
            self.struct_definitions
                .into_iter()
                .map(|ds| c::Node::DefineStruct(ds)),
        );

        result.extend(
            self.function_declarations
                .into_iter()
                .map(|df| c::Node::DeclareFunction(df)),
        );

        self.function_definitions[0].statements.statements.extend(vec![
            Statement::Code(CodeStatement {
                code: r#"
mem_test_verify (tm);
mem_test_free (tm);
            "#.to_string(),
            })
        ]);

        result.extend(
            self.function_definitions
                .into_iter()
                .map(|df| DefineFunction(df)),
        );

        Ok(result)
    }

    pub(crate) fn statements(&mut self) -> &mut Vec<c::Statement> {
        let ptr = self.function_pointer;
        &mut self.function_definitions[ptr.0].statements.statements
    }

    pub(crate) fn nodes(&mut self, ir: &IrTreeNode) -> Result<()> {
        match ir.node() {
            DeclareVariable(node) => self.declare_variable(node)?,
            CallFunctionOfPackage(node) => self.call_function_of_package(node)?,
            _ => unimplemented!("{ir:#?}")
        }
        Ok(())
    }

    pub(crate) fn statement(&mut self, ir: &IrTreeNode) -> Result<Vec<c::Statement>> {
        unimplemented!()
    }

    pub(crate) fn expression(&mut self, ir: &IrTreeNode) -> Result<c::Expression> {
        match ir.node() {
            AccessVariable(node) => Ok(c::Expression::Variable(self.access_variable(node)?)),
            InterpolateString(node) => Ok(c::Expression::Variable(self.interpolate_string(node)?)),
            LiteralBoolean(node) => Ok(self.literal_bool(node)?),
            LiteralNumber(node) => Ok(self.literal_number(node)?),
            LiteralString(node) => Ok(self.literal_string(node)?),
            _ => unimplemented!("{:#?}", ir)
        }
    }

    pub(crate) fn include_system(&mut self, path: &str) {
        self.directives.insert(IncludeSystemDirective(IncludeSystemDirectiveNode { path: path.to_string() }));
    }

    pub(crate) fn include_local(&mut self, path: &str) {
        self.directives.insert(IncludeLocalDirective(IncludeLocalDirectiveNode { path: path.to_string() }));
    }

    pub(crate) fn scope_leave(&mut self) {
        let cleanup_statements = self.stack.leave();
        self.statements().extend(cleanup_statements)
    }
}
