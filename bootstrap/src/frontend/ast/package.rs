use std::fs::File;
use std::io;
use std::io::Read;
use std::path::PathBuf;

use crate::frontend::{ast, ast_from_str, parse};
use crate::frontend::ast::Generator;
use crate::frontend::ast::node::{DeclarePackageNode, ExportPackageNode, Identifier};

impl<'a> Generator<'a> {
    pub(crate) fn generator_declare_package(&mut self, node: &parse::PackageDeclarationNode) -> crate::frontend::ast::Result<ast::Node> {
        let mut compiled_body = vec![];

        for node in &node.block.nodes {
            compiled_body.push(self.generator_node(node)?);
        }

        let mut external_functions = vec![];
        let mut functions = vec![];
        let mut definitions = vec![];
        let mut packages = vec![];

        for node in compiled_body.into_iter() {
            if let ast::Node::Block(block) = node {
                for node in block.body {
                    if let ast::Node::ExportPackage(ExportPackageNode { identifier, .. }) = node {
                        let package = self.ctx.get_str(identifier.0).to_string();

                        // FIXME temporary hack to load std packages
                        // FIXME compiler needs to track scope so that the parent package can easily be determined

                        match package.as_str() {
                            "io" => packages.extend(self.load_declared_packages("std/io/index.ec")),
                            "collection" => packages.extend(self.load_declared_packages("std/collection/index.ec")),
                            "list" => packages.extend(self.load_declared_packages("std/collection/list/index.ec")),
                            "math" => packages.extend(self.load_declared_packages("std/math/index.ec")),
                            "process" => packages.extend(self.load_declared_packages("std/process/index.ec")),
                            "intrinsics" => packages.extend(self.load_declared_packages("core/intrinsics/index.ec")),
                            _ => unimplemented!()
                        }
                    } else if let ast::Node::DeclareFunction(declare_function) = node {
                        functions.push(declare_function)
                    } else if let ast::Node::DefineType(define_type) = node {
                        definitions.push(define_type);
                    }
                }
            } else if let ast::Node::DeclareFunction(declare_function) = node {
                functions.push(declare_function)
            } else if let ast::Node::DefineType(define_type) = node {
                definitions.push(define_type);
            } else if let ast::Node::DeclarePackage(package) = node {
                packages.push(package);
            } else if let ast::Node::DeclareExternalFunction(external) = node {
                external_functions.push(external);
            } else {
                // unimplemented!("{:?}", node)
            }
        }

        Ok(ast::Node::DeclarePackage(DeclarePackageNode {
            identifier: Identifier::from(&node.identifier),
            modifiers: node.modifiers.clone(),
            functions,
            packages,
            definitions,
            external_functions,
        }))
    }


    fn load_declared_packages(&mut self, name: &str) -> Vec<DeclarePackageNode> {
        let content = crate::load_library_file(name).unwrap();
        let src_file = ast_from_str(&mut self.ctx, content.as_str()).unwrap();

        let mut result = vec![];

        for node in src_file.nodes {
            if let ast::Node::DeclarePackage(package_node) = node {
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