use std::fs::File;
use std::io;
use std::io::Read;
use std::path::PathBuf;

use crate::{ir, parse};
use crate::compile::{compile_str, Compiler};
use crate::ir::{DeclarePackageNode, ExportPackageNode, Identifier};

impl<'a> Compiler<'a> {
    pub(crate) fn compile_declare_package(&mut self, node: &parse::PackageDeclarationNode) -> crate::compile::Result<ir::Node> {
        let mut compiled_body = vec![];

        for node in &node.block.nodes {
            compiled_body.push(self.compile_node(node)?);
        }

        let mut functions = vec![];
        let mut definitions = vec![];
        let mut packages = vec![];

        for node in compiled_body.into_iter() {
            if let ir::Node::Block(block) = node {
                for node in block.body {
                    if let ir::Node::ExportPackage(ExportPackageNode { identifier, .. }) = node {
                        let package = self.ctx.get_str(identifier.0).to_string();

                        // FIXME temporary hack to load std packages
                        // FIXME compiler needs to track scope so that the parent package can easily be determined

                        match package.as_str() {
                            "io" => packages.extend(self.load_declared_packages("std/io/index.ec")),
                            "collection" => packages.extend(self.load_declared_packages("std/collection/index.ec")),
                            "list" => packages.extend(self.load_declared_packages("std/collection/list/index.ec")),
                            _ => unimplemented!()
                        }
                    } else if let ir::Node::DeclareFunction(declare_function) = node {
                        functions.push(declare_function)
                    } else if let ir::Node::DefineType(define_type) = node {
                        definitions.push(define_type);
                    }
                }
            }else if let ir::Node::DeclareFunction(declare_function) = node {
                functions.push(declare_function)
            } else if let ir::Node::DefineType(define_type) = node {
                definitions.push(define_type);
            }
        }

        Ok(ir::Node::DeclarePackage(DeclarePackageNode {
            identifier: Identifier::from(&node.identifier),
            modifiers: node.modifiers.clone(),
            functions,
            packages,
            definitions,
        }))
    }


    fn load_declared_packages(&mut self, name: &str) -> Vec<DeclarePackageNode> {
        let content = crate::load_library_file(name).unwrap();
        let src_file = compile_str(&mut self.ctx, content.as_str()).unwrap();

        let mut result = vec![];

        for node in src_file.body {
            if let ir::Node::DeclarePackage(package_node) = node {
                result.push(package_node);
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