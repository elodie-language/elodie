use crate::backend::generate::c;
use crate::backend::generate::c::{DirectiveNode, IncludeLocalDirectiveNode, IncludeSystemDirectiveNode};
use crate::backend::generate::c::emitter::Emitter;

impl Emitter {
    pub(crate) fn emit_directive(&mut self, node: &c::DirectiveNode) {
        match node {
            DirectiveNode::IncludeSystemDirective(IncludeSystemDirectiveNode { indent, path }) => {
                self.emit_line(&format!("#include <{path}>"))
            }
            DirectiveNode::IncludeLocalDirective(IncludeLocalDirectiveNode { indent, path }) => {
                self.emit_line(&format!("#include \"{path}\""))
            }
        }
    }
}