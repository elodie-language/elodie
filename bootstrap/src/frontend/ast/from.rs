use std::ops::Deref;

use crate::frontend::{ast, parse};
use crate::frontend::ast::Generator;
use crate::frontend::ast::node::{BlockNode, ExportPackageNode, Node, Source, SourceLocalFileNode};
use crate::frontend::parse::LiteralNode;

impl<'a> Generator<'a> {
    pub(crate) fn generator_from(&mut self, node: &parse::FromNode) -> ast::Result<ast::Node> {
        if let parse::FromNode::Export(export_node) = node {
            return self.generator_from_export(export_node);
        }

        unimplemented!();
    }

    pub(crate) fn generator_from_export(&mut self, node: &parse::FromExportNode) -> ast::Result<ast::Node> {
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
            })
        );
    }
}