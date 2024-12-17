use std::ops::Deref;

use crate::backend::generate::c;
use crate::backend::generate::c::{DeclareVariableStatement, Indent, InitialiseStructExpression, InitialiseStructField, LiteralDoubleExpression, Statement, VariableExpression};
use crate::backend::generate::c::generator::Generator;
use crate::ir::{DeclareVariableNode, LiteralNode, LoadValueFromSelfNode, LoadValueNode, Node};
use crate::ir::Node::Literal;

impl Generator {
    pub(crate) fn generate_declare_variable(&mut self, node: &DeclareVariableNode) -> crate::backend::generate::c::generator::Result<Vec<Statement>> {
        let variable = self.scope.push_variable(&node.identifier);

        if let Literal(LiteralNode::String(string)) = &node.value.deref() {
            return Ok(vec![
                Statement::DeclareVariable(DeclareVariableStatement {
                    indent: Indent::none(),
                    identifier: variable.to_string(&self.string_table),
                    r#type: "const char *".to_string(),
                    expression: c::Expression::Literal(
                        c::LiteralExpression::String(c::LiteralStringExpression {
                            indent: Indent::none(),
                            value: self.string_table.get(string.value).to_string(),
                        })
                    ),
                })
            ]);
        }


        if let Literal(LiteralNode::Number(number)) = &node.value.deref() {
            return Ok(vec![
                Statement::DeclareVariable(DeclareVariableStatement {
                    indent: Indent::none(),
                    identifier: variable.to_string(&self.string_table),
                    r#type: "double".to_string(),
                    expression: c::Expression::Literal(
                        c::LiteralExpression::Double(c::LiteralDoubleExpression {
                            indent: Indent::none(),
                            value: self.string_table.get(number.value).parse::<f64>().unwrap(),
                        })
                    ),
                })
            ]);
        }

        if let Literal(LiteralNode::Bool(boolean)) = &node.value.deref() {
            return Ok(vec![
                Statement::DeclareVariable(DeclareVariableStatement {
                    indent: Indent::none(),
                    identifier: variable.to_string(&self.string_table),
                    r#type: "_Bool".to_string(),
                    expression: c::Expression::Literal(
                        c::LiteralExpression::Bool(c::LiteralBooleanExpression {
                            indent: Indent::none(),
                            value: boolean.value,
                        })
                    ),
                })
            ]);
        }

        if let Node::InstantiateType(instantiate) = &node.value.deref() {
            let mut fields = Vec::new();

            let mut statements = vec![];

            for arg in &instantiate.arguments{
                let (s, expression) = self.generate_expression(&arg.value)?;

                statements.extend(s);
                fields.push(
                    InitialiseStructField {
                        indent: Indent::none(),
                        identifier: self.string_table.get(arg.identifier.0).to_string(),
                        expression,
                    }
                )
            }

            statements.push(
                Statement::DeclareVariable(DeclareVariableStatement {
                    indent: Indent::none(),
                    identifier: variable.to_string(&self.string_table),
                    r#type: format!("struct {}", self.string_table.get(instantiate.type_name.0)),
                    expression: c::Expression::StructInitialisation(
                        InitialiseStructExpression {
                            fields: fields.into_boxed_slice()
                        }
                    ),
                })
            );

            return Ok(statements);
        }

        unimplemented!("{node:#?}");
    }

    pub(crate) fn generate_load_value(&mut self, node: &LoadValueNode) -> c::generator::Result<c::Expression> {
        Ok(c::Expression::Variable(VariableExpression { indent: Indent::none(), identifier: self.scope.get_variable(&node.identifier).unwrap().to_string(&self.string_table) }))
    }

    pub(crate) fn generate_load_self_value(&mut self, node: &LoadValueFromSelfNode) -> c::generator::Result<c::Expression> {
        Ok(c::Expression::Variable(VariableExpression { indent: Indent::none(), identifier: format!("self.{}", self.string_table.get(node.property.0) ) }))
    }
}