use crate::common::Span;
use crate::frontend::ast::AstCallFunctionOfPackageNode;
use crate::ir::analyse::pre::Pre;
use crate::ir::analyse::TypedTreeNode;

impl<'a> Pre<'a> {

    pub(crate) fn call_function_of_package(&mut self, span: Span, node: &AstCallFunctionOfPackageNode) -> crate::ir::analyse::Result<TypedTreeNode> {
        println!("{:#?}", node.package.to_strs(&self.string_table));

        todo!()
    }
}