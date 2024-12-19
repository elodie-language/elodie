use crate::frontend::ast;
use crate::ir::infer::{DeclareVariableNode, Node};
use crate::ir::infer::Inference;
use crate::ir::symbol::SymbolName;

impl<'a> Inference<'a> {
    pub(crate) fn infer_declare_variable(&mut self, parsed_node: &'a ast::DeclareVariableNode) -> crate::ir::infer::Result<Node<'a>> {
        let symbol = self.register_variable(SymbolName::from(&parsed_node.identifier));

        let mut node = self.infer_node(&parsed_node.value)?;
        let inferred_type = if let Some(type_node) = &parsed_node.value_type {
            self.type_from_type_node(type_node)?
        } else {
            node.inferred_type()
        };

        Ok(Node::DeclareVariable(DeclareVariableNode {
            ast: parsed_node,
            symbol,
            node: Box::new(node),
            inferred_type,
        }))
    }
}

#[cfg(test)]
mod tests {
    use crate::frontend;
    use crate::frontend::ast_from_str;
    use crate::ir::context::Context;
    use crate::ir::infer::{infer, InferredType};
    use crate::ir::infer::node::Node::DeclareVariable;
    use crate::ir::symbol::SymbolId;

    #[test]
    fn declare_number_variable() {
        let mut ctx = frontend::Context::new();
        let ast = ast_from_str(&mut ctx, "let value = 23").unwrap();

        let mut ctx = Context::new(ctx, ast);
        let inferred = infer(&mut ctx).unwrap();
        assert_eq!(inferred.nodes.len(), 1);

        let DeclareVariable(node) = &inferred[0] else { panic!() };
        assert_eq!(node.symbol, SymbolId(1));
        assert_eq!(node.inferred_type, InferredType::Number)
    }

    #[test]
    fn declare_number_variable_with_explicit_type() {
        let mut ctx = frontend::Context::new();
        let ast = ast_from_str(&mut ctx, "let value: Number = 23").unwrap();

        let mut ctx = Context::new(ctx, ast);
        let inferred = infer(&mut ctx).unwrap();
        assert_eq!(inferred.nodes.len(), 1);

        let DeclareVariable(node) = &inferred[0] else { panic!() };
        assert_eq!(node.inferred_type, InferredType::Number)
    }

    #[test]
    fn declare_string_variable_with_explicit_type() {
        let mut ctx = frontend::Context::new();
        let ast = ast_from_str(&mut ctx, "let value: String = 'Elo'").unwrap();

        let mut ctx = Context::new(ctx, ast);
        let inferred = infer(&mut ctx).unwrap();
        assert_eq!(inferred.nodes.len(), 1);

        let DeclareVariable(node) = &inferred[0] else { panic!() };
        assert_eq!(node.inferred_type, InferredType::String)
    }

    #[test]
    fn declare_boolean_variable_with_explicit_type() {
        let mut ctx = frontend::Context::new();
        let ast = ast_from_str(&mut ctx, "let value = true").unwrap();

        let mut ctx = Context::new(ctx, ast);
        let inferred = infer(&mut ctx).unwrap();
        assert_eq!(inferred.nodes.len(), 1);

        let DeclareVariable(node) = &inferred[0] else { panic!() };
        assert_eq!(node.inferred_type, InferredType::Boolean)
    }
}