use std::ops::Deref;

use crate::{ir};
use crate::ir::{BlockNode, ExportPackageNode, Node, Source, SourceLocalFileNode};
use crate::ir::compile::Compiler;
use crate::frontend::parse::LiteralNode;
use crate::common::DefaultTypeIds;
use crate::frontend::parse;

impl<'a> Compiler<'a> {
    pub(crate) fn compile_from(&mut self, node: &parse::FromNode) -> crate::ir::compile::Result<ir::Node> {
        if let parse::FromNode::Export(export_node) = node {
            return self.compile_from_export(export_node);
        }

        unimplemented!();
    }

    pub(crate) fn compile_from_export(&mut self, node: &parse::FromExportNode) -> crate::ir::compile::Result<ir::Node> {
        let source = if let parse::Node::Literal(LiteralNode::String(from)) = &node.from_node.deref() {
            Source::LocalFile(SourceLocalFileNode { path: self.ctx.get_str(from.value()).to_string() })
        } else {
            todo!()
        };

        let identifier = if let parse::Node::Identifier(identifier) = &node.what_node.deref() {
            // at this point in time it should be clear what identifier refers to at the moment in can only be package
            ir::Identifier::from(identifier)
        } else {
            todo!()
        };

        return Ok(
            ir::Node::Block(BlockNode {
                body: vec![
                    Node::ExportPackage(ExportPackageNode {
                        identifier,
                        source,
                    })
                ],
                return_type: DefaultTypeIds::never(),
            })
        );
    }
}