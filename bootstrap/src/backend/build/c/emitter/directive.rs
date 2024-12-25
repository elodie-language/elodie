use crate::backend::build::c;
use crate::backend::build::c::emitter::Emitter;
use crate::backend::build::c::{
    DirectiveNode, IncludeLocalDirectiveNode, IncludeSystemDirectiveNode,
};

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
