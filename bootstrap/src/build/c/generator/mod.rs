use crate::build::c;
use crate::build::c::{
    DeclareFunctionNode, DeclareStructNode
    , DefineFunctionNode, DefineStructNode, DirectiveNode
    ,
    Statement,
};
use crate::common::context::Context;
use crate::common::StringTable;
use crate::ir;
use crate::ir::node::IrTreeNode;

#[derive(Debug)]
pub enum Error {}

type Result<T> = core::result::Result<T, Error>;

pub(crate) fn generate(ctx: Context, ir: ir::Ir) -> Result<Vec<c::Node>> {
    let mut generator = Generator {
        string_table: ctx.string_table,
        directives: Vec::new(),
        function_declarations: Vec::new(),
        function_definitions: Vec::new(),
        main_statements: Vec::new(),
        struct_definitions: Vec::new(),
        struct_declarations: Vec::new(),
    };
    generator.generate(ir.nodes)
}

pub(crate) struct Generator {
    string_table: StringTable,
    directives: Vec<DirectiveNode>,
    function_declarations: Vec<DeclareFunctionNode>,
    function_definitions: Vec<DefineFunctionNode>,
    main_statements: Vec<Statement>,
    struct_declarations: Vec<DeclareStructNode>,
    struct_definitions: Vec<DefineStructNode>,
}

impl Generator {
    pub(crate) fn generate(mut self, nodes: Vec<IrTreeNode>) -> Result<Vec<c::Node>> {
        unimplemented!()
    }

    pub(crate) fn nodes(&mut self, ir: &IrTreeNode) -> Result<()> {
        unimplemented!()
    }

    pub(crate) fn statements(&mut self, ir: &IrTreeNode) -> Result<Vec<c::Statement>> {
        unimplemented!()
    }

    pub(crate) fn expression(
        &mut self,
        ir: &IrTreeNode,
    ) -> Result<(Vec<c::Statement>, c::Expression)> {
        unimplemented!()
    }
}
