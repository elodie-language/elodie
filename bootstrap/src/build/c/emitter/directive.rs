use crate::build::c;
use crate::build::c::emitter::Emitter;
use crate::build::c::{
    DirectiveNode, IncludeLocalDirectiveNode, IncludeSystemDirectiveNode,
};

impl Emitter {
    pub(crate) fn directive(&mut self, node: &c::DirectiveNode) {
        match node {
            DirectiveNode::IncludeSystemDirective(IncludeSystemDirectiveNode { indent, path }) => {
                self.line(&format!("#include <{path}>"))
            }
            DirectiveNode::IncludeLocalDirective(IncludeLocalDirectiveNode { indent, path }) => {
                self.line(&format!("#include \"{path}\""))
            }
        }
    }
}
