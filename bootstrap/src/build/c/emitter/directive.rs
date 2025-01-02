use crate::build::c;
use crate::build::c::{
    DirectiveNode, IncludeLocalDirectiveNode, IncludeSystemDirectiveNode,
};
use crate::build::c::emitter::Emitter;

impl Emitter {
    pub(crate) fn directive(&mut self, node: &c::DirectiveNode) {
        match node {
            DirectiveNode::IncludeSystemDirective(IncludeSystemDirectiveNode { path }) => {
                self.line(&format!("#include <{path}>"))
            }
            DirectiveNode::IncludeLocalDirective(IncludeLocalDirectiveNode { path }) => {
                self.line(&format!("#include \"{path}\""))
            }
        }
    }
}
