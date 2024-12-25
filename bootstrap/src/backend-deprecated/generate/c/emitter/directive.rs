use crate::backend::generate::c;
use crate::backend::generate::c::emitter::Emitter;
use crate::backend::generate::c::{
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
