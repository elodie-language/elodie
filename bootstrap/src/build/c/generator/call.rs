use std::rc::Rc;

use crate::build::c;
use crate::build::c::{CallFunctionStatement, Expression, Indent};
use crate::build::c::generator::Generator;
use crate::build::c::Statement::CallFunction;
use crate::ir::{IrCallFunctionOfPackageNode, IrTreeNode};

impl Generator {
    pub(crate) fn call_function_of_package(&mut self, node: &IrCallFunctionOfPackageNode) -> c::generator::Result<()> {
        let arguments = self.generate_call_arguments(&node.arguments)?;

        self.statements().push(CallFunction(CallFunctionStatement {
            indent: Indent::none(),
            function: "rt_io_println".to_string(),
            arguments,
            result: None,
        }));

        self.include_local("rt/io.h");

        return Ok(());
    }

    fn generate_call_arguments(&mut self, args: &[Rc<IrTreeNode>]) -> c::generator::Result<Box<[Expression]>> {
        let mut result = vec![];
        for arg in args {
            // match self.expression(arg)? {
            //     Expression::Literal(literal) => {
            //         match literal {
            //             LiteralExpression::String(_) => {
            //                 result.push(c::Expression::CallFunction(CallFunctionExpression {
            //                     indent: Indent::none(),
            //                     function: "string_view_from_c_str".to_string(),
            //                     arguments: Box::new([
            //                         Expression::Literal(literal)
            //                     ]),
            //                 }))
            //             }
            //             _ => unimplemented!()
            //         }
            //     }
            //     expr => unimplemented!("{expr:#?}")
            // }
            result.push(self.expression(arg)?)
        }
        Ok(result.into_boxed_slice())
    }
}