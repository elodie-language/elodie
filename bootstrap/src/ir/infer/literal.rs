use crate::frontend::ast;
use crate::ir::infer::{Inference, InferredType};
use crate::ir::infer::{LiteralBooleanNode, LiteralNode, LiteralNumberNode, LiteralStringNode, Node};
use crate::ir::infer::Node::Literal;

impl<'a> Inference<'a> {
    pub(crate) fn infer_literal(&self, node: &'a ast::LiteralNode) -> crate::ir::infer::Result<Node<'a>> {
        match node {
            ast::LiteralNode::Boolean(parsed_node) => Ok(Literal(LiteralNode::Boolean(LiteralBooleanNode {
                parsed_node,
                inferred_type: InferredType::Boolean,
            }))),
            ast::LiteralNode::Number(parsed_node) => Ok(Literal(LiteralNode::Number(LiteralNumberNode {
                parsed_node,
                inferred_type: InferredType::Number,
            }))),
            ast::LiteralNode::String(parsed_node) => Ok(Literal(LiteralNode::String(LiteralStringNode {
                parsed_node,
                inferred_type: InferredType::String,
            }
            ))),
            _ => unimplemented!("{node:#?}")
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::common::Context;
    use crate::frontend::ast_from_str;
    use crate::ir::infer::{infer, InferredType};
    use crate::ir::infer::node::LiteralNode;
    use crate::ir::infer::node::Node::Literal;

    #[test]
    fn number_literal() {
        let mut ctx = Context::new();
        let mut parsed = ast_from_str(&mut ctx, "9924").unwrap();
        let inferred = infer(&mut ctx, &mut parsed).unwrap();
        assert_eq!(inferred.nodes.len(), 1);

        let Literal(LiteralNode::Number(node)) = &inferred[0] else { panic!() };
        assert_eq!(node.inferred_type, InferredType::Number)
    }

    #[test]
    fn string_literal() {
        let mut ctx = Context::new();
        let mut parsed = ast_from_str(&mut ctx, "'Elodie'").unwrap();
        let inferred = infer(&mut ctx, &mut parsed).unwrap();
        assert_eq!(inferred.nodes.len(), 1);

        let Literal(LiteralNode::String(node)) = &inferred[0] else { panic!() };
        assert_eq!(node.inferred_type, InferredType::String)
    }

    #[test]
    fn true_literal() {
        let mut ctx = Context::new();
        let mut parsed = ast_from_str(&mut ctx, "true").unwrap();
        let inferred = infer(&mut ctx, &mut parsed).unwrap();
        assert_eq!(inferred.nodes.len(), 1);

        let Literal(LiteralNode::Boolean(node)) = &inferred[0] else { panic!() };
        assert_eq!(node.inferred_type, InferredType::Boolean)
    }

    #[test]
    fn false_literal() {
        let mut ctx = Context::new();
        let mut parsed = ast_from_str(&mut ctx, "false").unwrap();
        let inferred = infer(&mut ctx, &mut parsed).unwrap();
        assert_eq!(inferred.nodes.len(), 1);

        let Literal(LiteralNode::Boolean(node)) = &inferred[0] else { panic!() };
        assert_eq!(node.inferred_type, InferredType::Boolean)
    }
}