use crate::backend::generate::c;
use crate::backend::generate::c::generator::Generator;
use crate::backend::generate::c::{
    DeclareVariableStatement, Indent, InitialiseStructExpression, InitialiseStructField, Statement,
    VariableExpression,
};
use crate::common::node::Node;
use crate::common::node::Node::{LiteralBoolean, LiteralNumber, LiteralString};
use crate::frontend::ast::{
    AstAccessVariableNode, AstAccessVariableOfSelfNode, AstDeclareVariableNode,
    AstLiteralBooleanNode, AstLiteralNumberNode, AstLiteralStringNode,
};

impl Generator {
    pub(crate) fn generate_declare_variable(
        &mut self,
        node: &AstDeclareVariableNode,
    ) -> crate::backend::generate::c::generator::Result<Vec<Statement>> {
        let variable = self.scope.push_variable(&node.variable);

        if let LiteralString(AstLiteralStringNode(string)) = &node.value.node() {
            return Ok(vec![Statement::DeclareVariable(DeclareVariableStatement {
                indent: Indent::none(),
                identifier: variable.to_string(&self.string_table),
                r#type: "const char *".to_string(),
                expression: c::Expression::Literal(c::LiteralExpression::String(
                    c::LiteralStringExpression {
                        indent: Indent::none(),
                        value: self.string_table.get(string.value()).to_string(),
                    },
                )),
            })]);
        }

        if let LiteralNumber(AstLiteralNumberNode(number)) = &node.value.node() {
            return Ok(vec![Statement::DeclareVariable(DeclareVariableStatement {
                indent: Indent::none(),
                identifier: variable.to_string(&self.string_table),
                r#type: "double".to_string(),
                expression: c::Expression::Literal(c::LiteralExpression::Double(
                    c::LiteralDoubleExpression {
                        indent: Indent::none(),
                        value: self
                            .string_table
                            .get(number.value())
                            .parse::<f64>()
                            .unwrap(),
                    },
                )),
            })]);
        }

        if let LiteralBoolean(AstLiteralBooleanNode(boolean)) = &node.value.node() {
            return Ok(vec![Statement::DeclareVariable(DeclareVariableStatement {
                indent: Indent::none(),
                identifier: variable.to_string(&self.string_table),
                r#type: "_Bool".to_string(),
                expression: c::Expression::Literal(c::LiteralExpression::Bool(
                    c::LiteralBooleanExpression {
                        indent: Indent::none(),
                        value: self.string_table.get(boolean.value) == "true",
                    },
                )),
            })]);
        }

        if let Node::InstantiateType(instantiate) = &node.value.node() {
            let mut fields = Vec::new();

            let mut statements = vec![];

            for arg in &instantiate.arguments {
                let (s, expression) = self.generate_expression(&arg.value)?;

                statements.extend(s);
                fields.push(InitialiseStructField {
                    indent: Indent::none(),
                    identifier: self.string_table.get(arg.identifier.0).to_string(),
                    expression,
                })
            }

            statements.push(Statement::DeclareVariable(DeclareVariableStatement {
                indent: Indent::none(),
                identifier: variable.to_string(&self.string_table),
                r#type: format!("struct {}", self.string_table.get(instantiate.r#type.0)),
                expression: c::Expression::StructInitialisation(InitialiseStructExpression {
                    fields: fields.into_boxed_slice(),
                }),
            }));

            return Ok(statements);
        }

        unimplemented!("{node:#?}");
    }

    pub(crate) fn generate_load_value(
        &mut self,
        node: &AstAccessVariableNode,
    ) -> c::generator::Result<c::Expression> {
        Ok(c::Expression::Variable(VariableExpression {
            indent: Indent::none(),
            identifier: self
                .scope
                .get_variable(&node.variable)
                .unwrap()
                .to_string(&self.string_table),
        }))
    }

    pub(crate) fn generate_load_self_value(
        &mut self,
        node: &AstAccessVariableOfSelfNode,
    ) -> c::generator::Result<c::Expression> {
        Ok(c::Expression::Variable(VariableExpression {
            indent: Indent::none(),
            identifier: format!("self.{}", self.string_table.get(node.variable.0)),
        }))
    }
}
