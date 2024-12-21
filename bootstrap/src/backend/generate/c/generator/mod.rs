use std::ops::Deref;
use std::vec;

use crate::backend::generate::c;
use crate::backend::generate::c::{
    BlockStatement, CallFunctionStatement, CallFunctionStatementResult,
    DeclareFunctionArgumentNode, DeclareFunctionNode, DeclareStructNode,
    DefineFunctionArgumentNode, DefineFunctionNode, DefineStructNode, DirectiveNode, Expression,
    IncludeLocalDirectiveNode, IncludeSystemDirectiveNode, Indent, ReturnFromFunctionStatement,
    Statement, VariableExpression,
};
use crate::backend::generate::c::DirectiveNode::{IncludeLocalDirective, IncludeSystemDirective};
use crate::backend::generate::c::generator::scope::Scope;
use crate::backend::generate::c::Node::DefineFunction;
use crate::common::node::Node;
use crate::common::StringTable;
use crate::frontend;
use crate::frontend::ast::{AstDefineTypeNode, AstNode, AstTreeNode};

mod block;
mod control;
mod function;
mod infix;
mod literal;
mod scope;
mod string;
mod variable;

pub enum Error {}

type Result<T> = core::result::Result<T, Error>;

pub(crate) fn generate(ctx: frontend::Context, ast: frontend::Ast) -> Result<Vec<c::Node>> {
    let mut generator = Generator {
        string_table: ctx.string_table,
        // type_table: ctx.type_table,
        scope: Scope::new(),
        directives: Vec::new(),
        function_declarations: Vec::new(),
        function_definitions: Vec::new(),
        main_statements: Vec::new(),
        struct_definitions: Vec::new(),
        struct_declarations: Vec::new(),
    };
    generator.generate(ast.nodes)
}

pub(crate) struct Generator {
    string_table: StringTable,
    // type_table: TypeTable,
    scope: Scope,
    //
    directives: Vec<DirectiveNode>,
    function_declarations: Vec<DeclareFunctionNode>,
    function_definitions: Vec<DefineFunctionNode>,
    main_statements: Vec<Statement>,
    struct_declarations: Vec<DeclareStructNode>,
    struct_definitions: Vec<DefineStructNode>,
}

impl Generator {
    pub(crate) fn generate(mut self, nodes: Vec<AstTreeNode>) -> Result<Vec<c::Node>> {
        for node in &nodes {
            match node.node() {
                Node::DeclareFunction(_) => {}
                _ => {}
            }
            (self.generate_nodes(node)?)
        }

        self.directives.extend(vec![
            IncludeSystemDirective(IncludeSystemDirectiveNode {
                indent: Indent::none(),
                path: "stdio.h".to_string(),
            }),
            IncludeSystemDirective(IncludeSystemDirectiveNode {
                indent: Indent::none(),
                path: "stdbool.h".to_string(),
            }),
            IncludeLocalDirective(IncludeLocalDirectiveNode {
                indent: Indent::none(),
                path: "core_intrinsics_io.h".to_string(),
            }),
            IncludeLocalDirective(IncludeLocalDirectiveNode {
                indent: Indent::none(),
                path: "std_io.h".to_string(),
            }),
            IncludeLocalDirective(IncludeLocalDirectiveNode {
                indent: Indent::none(),
                path: "core_intrinsics_math.h".to_string(),
            }),
            IncludeLocalDirective(IncludeLocalDirectiveNode {
                indent: Indent::none(),
                path: "core_bool.h".to_string(),
            }),
        ]);

        let mut result = vec![];
        result.extend(self.directives.into_iter().map(|d| c::Node::Directive(d)));

        result.extend(
            self.struct_declarations
                .into_iter()
                .map(|ds| c::Node::DeclareStruct(ds)),
        );
        result.extend(
            self.struct_definitions
                .into_iter()
                .map(|ds| c::Node::DefineStruct(ds)),
        );

        result.extend(
            self.function_declarations
                .into_iter()
                .map(|df| c::Node::DeclareFunction(df)),
        );

        result.push(DefineFunction(DefineFunctionNode {
            indent: Indent::none(),
            identifier: "main".to_string(),
            arguments: vec![].into_boxed_slice(),
            ty: "int".to_string(),

            statements: BlockStatement {
                indent: Indent::none(),
                statements: self.main_statements,
            },
        }));

        result.extend(
            self.function_definitions
                .into_iter()
                .map(|df| c::Node::DefineFunction(df)),
        );

        // Ok(
        //     vec![
        //         DefineFunction(DefineFunctionNode {
        //             indent: Indent::none(),
        //             identifier: "main".to_string(),
        //             arguments: vec![].into_boxed_slice(),
        //             ty: "int".to_string(),
        //
        //             statements: BlockStatement {
        //                 indent: Indent::none(),
        //                 statements,
        //             },
        //         })],
        // )

        Ok(result)
    }

    pub(crate) fn generate_nodes(&mut self, node: &AstTreeNode) -> Result<()> {
        let _ = match node.node() {
            Node::Block(node) => {
                let stmts = self.generate_block(node)?;
                self.main_statements.push(Statement::Block(stmts));
            }
            Node::BreakLoop(_) => unimplemented!(),
            Node::Calculate(_) => unimplemented!(),
            Node::CallFunctionOfObject(node) => {
                // FIXME use object tid to resolve type name

                self.main_statements
                    .push(Statement::CallFunction(CallFunctionStatement {
                        indent: Indent::none(),
                        identifier: format!(
                            "{}_{}",
                            "person",
                            self.string_table.get(node.function.0)
                        ),
                        arguments: Box::new([Expression::Variable(VariableExpression {
                            indent: Indent::none(),
                            identifier: format!(
                                "&{}",
                                self.scope
                                    .get_variable(&node.object)
                                    .unwrap()
                                    .to_string(&self.string_table)
                            ),
                        })]),
                        result: None,
                    }))
            }
            Node::CallFunctionOfPackage(node) => {
                let statements = self.generate_call_function_of_package(node)?;
                self.main_statements.extend(statements);
            }
            Node::CallFunction(node) => unimplemented!(),
            Node::CallFunctionWithLambda(_) => unimplemented!(),
            Node::ExportPackage(_) => unimplemented!(),
            Node::ReturnFromFunction(_) => unimplemented!(),
            Node::ContinueLoop(_) => unimplemented!(),
            Node::Compare(_) => unimplemented!(),
            Node::If(node) => {
                let stmts = self.generate_if(node)?;
                self.main_statements.extend(stmts);
            }
            Node::AccessVariable(_) => unimplemented!(),
            Node::AccessVariableOfObject(_) => unimplemented!(),
            Node::AccessVariableOfSelf(_) => unimplemented!(),
            Node::Loop(_) => unimplemented!(),
            Node::DeclareVariable(node) => {
                let stmts = self.generate_declare_variable(node)?;
                self.main_statements.extend(stmts);
            }
            Node::DeclareFunction(node) => {
                let func_ident = self.string_table.get(node.function.0).to_string();

                // let ty = if self.type_table.is_boolean(&node.return_type) {
                //     "_Bool"
                // } else if self.type_table.is_number(&node.return_type) {
                //     "double"
                // } else {
                //     unimplemented!()
                // };

                // self.function_declarations.push(DeclareFunctionNode {
                //     indent: Indent::none(),
                //     identifier: func_ident.to_string(),
                //     arguments: Box::new([]),
                //     ty: ty.to_string(),
                // });

                let statements = self.generate_block(node.nodes.as_ref())?;
                //
                // self.function_definitions.push(DefineFunctionNode {
                //     indent: Indent::none(),
                //     identifier: func_ident.to_string(),
                //     arguments: Box::new([]),
                //     ty: ty.to_string(),
                //     statements,
                // });
            }
            Node::DeclareExternalFunction(_) => unimplemented!(),
            Node::DeclarePackage(_) => unimplemented!(),
            Node::DeclareType(node) => {
                self.struct_declarations.push(DeclareStructNode {
                    indent: Indent::none(),
                    identifier: self.string_table.get(node.r#type.0).to_string(),
                });

                let mut fields = Vec::new();
                for prop in &node.variables {
                    // let ty = if self.type_table.is_number(&prop.r#type) {
                    //     "double".to_string()
                    // } else if self.type_table.is_boolean(&prop.r#type) {
                    //     "_Bool".to_string()
                    // } else if self.type_table.is_string(&prop.r#type) {
                    //     "const char *".to_string()
                    // } else {
                    //     panic!()
                    // };
                    //
                    // fields.push(DefineStructFieldNode {
                    //     indent: Indent::none(),
                    //     identifier: self.string_table.get(prop.identifier.0).to_string(),
                    //     ty,
                    // })
                }

                self.struct_definitions.push(DefineStructNode {
                    indent: Indent::none(),
                    identifier: self.string_table.get(node.r#type.0).to_string(),
                    fields: fields.into_boxed_slice(),
                })
            }
            Node::InstantiateType(_) => unimplemented!(),
            Node::DefineType(AstDefineTypeNode {
                                 r#type,
                                 modifiers,
                                 functions,
                                 ..
                             }) => {
                for function in functions {
                    self.function_declarations.push(DeclareFunctionNode {
                        indent: Indent::none(),
                        identifier: format!(
                            "{}_{}",
                            self.string_table.get(r#type.0).to_lowercase(),
                            self.string_table.get(function.function.0)
                        ),
                        arguments: Box::new([DeclareFunctionArgumentNode {
                            indent: Indent::none(),
                            identifier: "self".to_string(),
                            ty: format!("struct {} *", self.string_table.get(r#type.0)),
                        }]),
                        ty: "void".to_string(),
                    });

                    let statements = self.generate_block(function.nodes.as_ref())?;

                    self.function_definitions.push(DefineFunctionNode {
                        indent: Indent::none(),
                        identifier: "person_say_name".to_string(),
                        arguments: Box::new([DefineFunctionArgumentNode {
                            indent: Indent::none(),
                            identifier: "self".to_string(),
                            ty: format!("struct {} *", self.string_table.get(function.function.0)),
                        }]),
                        ty: "void".to_string(),
                        statements,
                    })
                }
            }
            Node::InterpolateString(_) => unimplemented!(),
            Node::LiteralBoolean(_) => {}
            Node::LiteralNumber(_) => {}
            Node::LiteralString(_) => {}
            AstNode::Marker(_) => {}
        };
        Ok(())
    }

    pub(crate) fn generate_statements(&mut self, node: &AstTreeNode) -> Result<Vec<c::Statement>> {
        match node.node() {
            Node::Block(node) => Ok(vec![Statement::Block(self.generate_block(node)?)]),
            Node::BreakLoop(_) => unimplemented!(),
            Node::Calculate(_) => unimplemented!(),
            Node::CallFunctionOfObject(_) => unimplemented!(),
            Node::CallFunctionOfPackage(node) => self.generate_call_function_of_package(node),
            Node::CallFunction(node) => self.generate_call_function(node),
            Node::CallFunctionWithLambda(_) => unimplemented!(),
            Node::ExportPackage(_) => unimplemented!(),
            Node::ReturnFromFunction(node) => {
                let mut result = vec![];


                let (statements, expression) = self.generate_expression(&node.node.clone().unwrap())?;

                result.extend(statements);

                result.push(c::Statement::ReturnFromFunction(
                    ReturnFromFunctionStatement {
                        indent: Indent::none(),
                        node: Some(expression),
                    },
                ));

                Ok(result)
            }
            Node::ContinueLoop(_) => unimplemented!(),
            Node::Compare(_) => unimplemented!(),
            Node::If(node) => self.generate_if(node),
            Node::AccessVariable(_) => unimplemented!(),
            Node::AccessVariableOfObject(_) => unimplemented!(),
            Node::AccessVariableOfSelf(_) => unimplemented!(),
            Node::Loop(_) => unimplemented!(),
            Node::DeclareVariable(node) => self.generate_declare_variable(node),
            Node::DeclareFunction(node) => {
                let func_ident = self.string_table.get(node.function.0).to_string();

                // let ty = if self.type_table.is_boolean(&node.return_type) {
                //     "_Bool"
                // } else if self.type_table.is_number(&node.return_type) {
                //     "double"
                // } else {
                //     unimplemented!()
                // };
                //
                // self.function_declarations.push(DeclareFunctionNode {
                //     indent: Indent::none(),
                //     identifier: func_ident.to_string(),
                //     arguments: Box::new([]),
                //     ty: ty.to_string(),
                // });
                //
                // let statements = self.generate_block(node.body.as_ref())?;
                //
                // self.function_definitions.push(DefineFunctionNode {
                //     indent: Indent::none(),
                //     identifier: func_ident.to_string(),
                //     arguments: Box::new([]),
                //     ty: ty.to_string(),
                //     statements,
                // });
                // Ok(vec![])
                todo!()
            }
            Node::DeclareExternalFunction(_) => unimplemented!(),
            Node::DeclarePackage(_) => unimplemented!(),
            Node::DeclareType(_) => unimplemented!(),
            Node::InstantiateType(_) => unimplemented!(),
            Node::DefineType(_) => unimplemented!(),
            Node::InterpolateString(_) => unimplemented!(),
            Node::LiteralBoolean(_) => unimplemented!(),
            Node::LiteralNumber(_) => unimplemented!(),
            Node::LiteralString(_) => unimplemented!(),
            AstNode::Marker(_) => unreachable!()
        }
    }

    pub(crate) fn generate_expression(
        &mut self,
        node: &AstTreeNode,
    ) -> Result<(Vec<c::Statement>, c::Expression)> {
        match node.node() {
            Node::LiteralString(node) => Ok((vec![], c::Expression::Literal(self.generate_literal_string(node)?))),
            Node::LiteralNumber(node) => Ok((vec![], c::Expression::Literal(self.generate_literal_number(node)?))),
            Node::LiteralBoolean(node) => Ok((vec![], c::Expression::Literal(self.generate_literal_bool(node)?))),
            Node::AccessVariable(node) => Ok((vec![], self.generate_load_value(node)?)),
            Node::AccessVariableOfSelf(node) => Ok((vec![], self.generate_load_self_value(node)?)),
            Node::Compare(node) => {
                let (statements, expression) = self.generate_compare(node)?;
                Ok((statements, Expression::Infix(expression)))
            }
            Node::Calculate(node) => {
                let (statements, expression) = self.generate_calculate(node)?;
                Ok((statements, Expression::Infix(expression)))
            }
            Node::CallFunction(node) => {
                let mut statements = vec![];
                let arg_identifier = self.scope.push_argument();

                statements.push(Statement::CallFunction(CallFunctionStatement {
                    indent: Indent::none(),
                    identifier: self.string_table.get(node.function.0).to_string(),
                    arguments: Box::new([]),
                    result: Some(CallFunctionStatementResult {
                        indent: Indent::none(),
                        identifier: arg_identifier.to_string(),
                        r#type: "bool".to_string(),
                    }),
                }));

                Ok((
                    statements,
                    Expression::Variable(VariableExpression {
                        indent: Indent::none(),
                        identifier: arg_identifier.to_string(),
                    }),
                ))
            }
            _ => unimplemented!("{:#?}", node),
        }
    }
}
