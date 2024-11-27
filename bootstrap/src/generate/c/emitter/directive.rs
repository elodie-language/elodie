use crate::generate::c;
use crate::generate::c::{DirectiveNode, emitter, IncludeSystemDirectiveNode};
use crate::generate::c::emitter::Emitter;

impl Emitter {
    pub(crate) fn emit_directive(&mut self, node: &c::DirectiveNode) -> emitter::Result<()> {
        match node {
            DirectiveNode::IncludeSystemDirective(IncludeSystemDirectiveNode { indent, path }) => {
                self.emit_line(&format!("#include<{path}>"))
            }
            DirectiveNode::IncludeLocalDirective(_) => unimplemented!()
        }
        Ok(())
    }
}