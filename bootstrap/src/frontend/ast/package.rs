use std::fs::File;
use std::io;
use std::io::Read;
use std::ops::Deref;
use std::path::PathBuf;

use crate::common::node::{Node, Source};
use crate::common::node::Node::{Block, ExportPackage};
use crate::frontend::{ast, ast_from_str, parse};
use crate::frontend::ast::{AstBlockNode, AstDeclareExternalFunctionNode, AstDeclareFunctionNode, AstDeclarePackageNode, AstDefineTypeNode, AstExportPackageNode, AstIdentifier, AstTreeNode, Generator, SPAN_NOT_IMPLEMENTED};
use crate::frontend::parse::LiteralNode;

impl<'a> Generator<'a> {
    pub(crate) fn generate_from(&mut self, node: &parse::FromNode) -> ast::Result<AstTreeNode> {
        if let parse::FromNode::Export(export_node) = node {
            return self.generate_from_export(export_node);
        }

        unimplemented!();
    }

    pub(crate) fn generate_from_export(
        &mut self,
        node: &parse::FromExportNode,
    ) -> ast::Result<AstTreeNode> {
        let source = if let parse::Node::Literal(LiteralNode::String(from)) = &node.from_node.deref() {
            Source::LocalFile {
                path: self.ctx.get_str(from.value()).to_string(),
            }
        } else {
            todo!()
        };

        let package = if let parse::Node::Identifier(identifier) = &node.what_node.deref() {
            // at this point in time it should be clear what identifier refers to at the moment in can only be package
            ast::AstIdentifier(identifier.value())
        } else {
            todo!()
        };

        return Ok(AstTreeNode::new(Block(AstBlockNode {
            nodes: vec![AstTreeNode::new(ExportPackage(AstExportPackageNode {
                package: AstIdentifier(package.0.clone()),
                source,
            }), SPAN_NOT_IMPLEMENTED.clone())],
        }), SPAN_NOT_IMPLEMENTED.clone()));
    }


    pub(crate) fn generate_declare_package(
        &mut self,
        node: &parse::PackageDeclarationNode,
    ) -> ast::Result<AstTreeNode> {
        let mut compiled_body = vec![];

        for node in &node.block.nodes {
            compiled_body.push(self.generate_node(node)?);
        }

        let mut external_functions: Vec<AstDeclareExternalFunctionNode> = vec![];
        let mut functions: Vec<AstDeclareFunctionNode> = vec![];
        let mut definitions: Vec<AstDefineTypeNode> = vec![];
        let mut packages: Vec<AstDeclarePackageNode> = vec![];

        for node in compiled_body.into_iter() {
            if let Block(block) = node.node() {
                for node in &block.nodes {
                    if let ExportPackage(AstExportPackageNode { package, .. }) = node.node() {
                        let package = self.ctx.get_str(package.0).to_string();

                        // FIXME temporary hack to load std packages
                        // FIXME compiler needs to track scope so that the parent package can easily be determined

                        match package.as_str() {
                            "io" => packages.extend(self.load_declared_packages("std/io/index.ec")),
                            "collection" => packages
                                .extend(self.load_declared_packages("std/collection/index.ec")),
                            "list" => packages.extend(
                                self.load_declared_packages("std/collection/list/index.ec"),
                            ),
                            "math" => {
                                packages.extend(self.load_declared_packages("std/math/index.ec"))
                            }
                            "process" => {
                                packages.extend(self.load_declared_packages("std/process/index.ec"))
                            }
                            "intrinsics" => packages
                                .extend(self.load_declared_packages("core/intrinsics/index.ec")),
                            _ => unimplemented!(),
                        }
                    } else if let Node::DeclareFunction(declare_function) = node.node() {
                        functions.push(declare_function.clone())
                    } else if let Node::DefineType(define_type) = node.node() {
                        definitions.push(define_type.clone());
                    }
                }
            } else if let Node::DeclareFunction(declare_function) = node.node() {
                functions.push(declare_function.clone())
            } else if let Node::DefineType(define_type) = node.node() {
                definitions.push(define_type.clone());
            } else if let Node::DeclarePackage(package) = node.node() {
                packages.push(package.clone());
            } else if let Node::DeclareExternalFunction(external) = node.node() {
                external_functions.push(external.clone());
            } else {
                // unimplemented!("{:?}", node)
            }
        }

        Ok(AstTreeNode::new(Node::DeclarePackage(AstDeclarePackageNode {
            package: AstIdentifier(node.identifier.value()),
            modifiers: node.modifiers.clone(),
            functions,
            packages,
            definitions: definitions,
            external_functions,
        }), SPAN_NOT_IMPLEMENTED.clone()))
    }

    fn load_declared_packages(&mut self, name: &str) -> Vec<AstDeclarePackageNode> {
        let content = crate::load_library_file(name).unwrap();
        let ast = ast_from_str(&mut self.ctx, content.as_str()).unwrap();

        let mut result = vec![];

        for node in ast.nodes {
            if let Node::DeclarePackage(package_node) = node.node_to_owned() {
                result.push(package_node.clone());
            }
        }
        result
    }
}


fn load_library_file(filename: &str) -> io::Result<String> {
    // Get the path to the project root directory
    let manifest_dir = "/home/ddymke/repo/elodie/src/lib/";

    // Construct the full path to the file
    let file_path = PathBuf::from(manifest_dir).join(filename);
    // println!("{file_path:?}");

    // Read the file's contents
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    // println!("{contents}");
    Ok(contents)
}
