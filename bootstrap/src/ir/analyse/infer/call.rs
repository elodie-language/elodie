use crate::ir::analyse::infer::Inferrer;
use crate::ir::analyse::TypedTreeNode;

impl<'a> Inferrer<'a> {
    pub(crate) fn call_function_of_package(&mut self, node: &mut TypedTreeNode) -> crate::ir::analyse::Result<()> {
        // FIXME TODO
        Ok(())
    }
}