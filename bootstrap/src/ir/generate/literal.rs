mod tests {
    use bigdecimal::BigDecimal;

    use crate::common::context::Context;
    use crate::ir::ir_from_str;

    #[test]
    fn number_literal() {
        let mut ctx = Context::testing();
        let ir = ir_from_str(&mut ctx, "9924").unwrap();
        assert_eq!(ir.len(), 1);

        let result = &ir[0];
        assert_eq!(result.as_literal_number().value, BigDecimal::from(9924));
        assert_eq!(result.type_id, ctx.type_id_number())
    }
}
