use crate::generate::c;
use crate::generate::c::{CallFunctionStatement, Expression, Indent, Statement};
use crate::generate::c::generator::Generator;
use crate::generate::c::Statement::CallFunction;
use crate::ir::{CallFunctionNode, CallFunctionOfPackageNode, InterpolateStringNode, LoadValueNode, Node};

impl Generator {
    pub(crate) fn generate_call_function(&mut self, node: &CallFunctionNode) -> c::generator::Result<Vec<Statement>> {
        let function = self.string_cache.get(node.function.0).to_string();

        let mut result = vec![];

        for arg in &node.arguments {

            // to_string + concatenation
            if let Node::InterpolateString(InterpolateStringNode { nodes }) = arg {
                for node in nodes {
                    dbg!(&node);

                    if let Node::LoadValue(LoadValueNode { identifier, ty: type_id }) = node {

                    }
                }
            }
        }


        let arguments = self.generate_call_arguments(&node.arguments)?.into_boxed_slice();
        result.push(
            CallFunction(CallFunctionStatement {
                indent: Indent::none(),
                identifier: function,
                arguments,
            })
        );

        Ok(result)
    }

    pub(crate) fn generate_call_function_of_package(&mut self, node: &CallFunctionOfPackageNode) -> c::generator::Result<Vec<Statement>> {
        let std = self.string_cache.get(node.package.segments[0]).to_string();
        let io = self.string_cache.get(node.package.segments[1]).to_string();
        let function = self.string_cache.get(node.function.0).to_string();
        let arguments = self.generate_call_arguments(&node.arguments)?.into_boxed_slice();


        return Ok(vec![CallFunction(CallFunctionStatement {
            indent: Indent::none(),
            identifier: format!("{std}_{io}_{function}"),
            arguments,
        })]);

        unimplemented!()
    }

    fn generate_call_arguments(&mut self, nodes: &[Node]) -> c::generator::Result<Vec<Expression>> {
        let mut result = vec![];
        for node in nodes {
            result.push(self.generate_expression(node)?)
        }
        Ok(result)
    }
}