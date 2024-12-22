mod tests {
    use bigdecimal::BigDecimal;

    use crate::ir::ir_from_str;

    #[test]
    fn number_literal() {
        let ir = ir_from_str("9924").unwrap();
        assert_eq!(ir.len(), 1);

        let result = &ir[0];
        assert_eq!(result.as_literal_number().value, BigDecimal::from(9924));
    }
}
