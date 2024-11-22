use std::ops::Deref;

use crate::{ast, parse};
use crate::ast::{BlockNode, ExportPackageNode, Node, Source, SourceLocalFileNode};
use crate::compile::Compiler;
use crate::parse::LiteralNode;
use crate::r#type::DefaultTypeIds;

impl<'a> Compiler<'a> {
    pub(crate) fn compile_from(&mut self, node: &parse::FromNode) -> crate::compile::Result<ast::Node> {
        if let parse::FromNode::Export(export_node) = node {
            return self.compile_from_export(export_node);
        }

        unimplemented!();
    }

    pub(crate) fn compile_from_export(&mut self, node: &parse::FromExportNode) -> crate::compile::Result<ast::Node> {
        let source = if let parse::Node::Literal(LiteralNode::String(from)) = &node.from_node.deref() {
            Source::LocalFile(SourceLocalFileNode { path: self.ctx.get_str(from.value()).to_string() })
        } else {
            todo!()
        };

        let identifier = if let parse::Node::Identifier(identifier) = &node.what_node.deref() {
            // at this point in time it should be clear what identifier refers to at the moment in can only be package
            ast::Identifier::from(identifier)
        } else {
            todo!()
        };

        return Ok(
            ast::Node::Block(BlockNode {
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