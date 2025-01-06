use crate::build::c;
use crate::build::c::{BlockStatement, BreakStatement, ExpressionStatement, IfStatement, LoopStatement, Statement, StatementResult};
use crate::build::c::generator::Generator;
use crate::build::c::Statement::If;
use crate::common::node::Node;
use crate::ir::{IrIfNode, IrLoopNode};

impl Generator {
    pub(crate) fn r#loop(&mut self, node: &IrLoopNode, sr: Option<StatementResult>) -> c::generator::Result<()> {
        // let temp = self.scope.push_temp(Storage::Stack);

        self.scope.enter();

        for node in &node.block.nodes {
            if let Node::BreakLoop(inner) = &node.node {
                if let Some(value) = &inner.node {
                    let e = self.expression(value.as_ref())?;
                    self.statements().push(
                        Statement::Expression(
                            ExpressionStatement {
                                expression: e,
                                result: sr.clone(),
                            })
                    );
                }

                let frame = self.scope.frame();
                let cleanup_statements = frame.cleanup_statements();
                self.statements().extend(cleanup_statements);

                self.statements().push(Statement::Break(BreakStatement {}))
            } else {
                self.nodes(node.as_ref())?
            }
        }

        let mut frame = self.scope.leave();
        // let cleanup_statements = frame.cleanup();

        let mut statements = vec![];
        statements.extend(frame.statements);
        // statements.extend(cleanup_statements);


        // self.scope.push_local_variable("result".to_string(), Storage::Memory);

        self.statements().push(Statement::Loop(
            LoopStatement {
                block: BlockStatement { statements },
                // result: Some(StatementResult::Declare {
                //     variable: temp.to_string(),
                //     r#type: "struct val_num *".to_string(),
                // }),
                result: None,
            }
        ));

        Ok(())
    }

    pub(crate) fn r#if(&mut self, node: &IrIfNode) -> c::generator::Result<()> {
        let condition = self.expression(node.condition.as_ref())?;

        self.scope.enter();
        for node in &node.then.nodes {
            self.nodes(node.as_ref())?
        }

        let mut then_frame = self.scope.leave();
        let then_cleanup_statements = then_frame.cleanup_statements();

        let mut then_statements = vec![];
        then_statements.extend(then_frame.statements);
        then_statements.extend(then_cleanup_statements);


        let otherwise = if let Some(otherwise) = &node.otherwise {
            self.scope.enter();
            for node in &otherwise.nodes {
                self.nodes(node.as_ref())?
            }

            let mut frame = self.scope.leave();
            let cleanup_statements = frame.cleanup_statements();

            let mut statements = vec![];
            statements.extend(frame.statements);
            statements.extend(cleanup_statements);

            Some(BlockStatement { statements })
        } else {
            None
        };


        self.statements().push(If(IfStatement {
            condition,
            then: BlockStatement { statements: then_statements },
            otherwise,
        }));

        return Ok(());
    }
}