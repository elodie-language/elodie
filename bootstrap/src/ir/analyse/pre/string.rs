use crate::common::Inferred;
use crate::common::node::Node::InterpolateString;
use crate::frontend::ast::AstInterpolateStringNode;
use crate::ir::analyse::{TypedTreeNode, TypeInterpolateStringNode};
use crate::ir::analyse::pre::Pre;

impl<'a> Pre<'a> {
    pub(crate) fn interpolate_string(
        &mut self,
        node: &AstInterpolateStringNode,
    ) -> crate::ir::analyse::Result<TypedTreeNode> {
        let mut nodes = Vec::with_capacity(node.nodes.len());

        for node in &node.nodes {
            nodes.push(self.node(node)?)
        }

        Ok(TypedTreeNode::new(
            InterpolateString(TypeInterpolateStringNode {
                nodes: nodes.into_boxed_slice()
            }),
            self.span(),
            Inferred::String,
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::common::{Context, Inferred};
    use crate::frontend::ast_from_str;
    use crate::ir::analyse::prepare;

    #[test]
    fn interpolate_number() {
        let mut ctx = Context::testing();
        let ast = ast_from_str(&mut ctx, r#"
            let value = 9924
            '${value}'
        "#).unwrap();
        let typed = prepare(&mut ctx, ast).unwrap();
        assert_eq!(typed.nodes.len(), 2);

        let result = &typed[1];
        let inner = result.as_interpolate_string();

        assert_eq!(result.inferred, Inferred::String);
    }
}