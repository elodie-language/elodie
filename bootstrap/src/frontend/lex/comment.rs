use crate::frontend::lex::Lexer;

impl Lexer<'_> {
    pub(crate) fn is_comment(&self, c: char) -> bool {
        c == '/' && self.peek_if("//").is_some()
    }

    pub(crate) fn consume_comment(&self) -> crate::frontend::lex::Result<()> {
        self.consume_while(|c| c != '\n')?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::common::context::Context;
    use crate::frontend::lex::token::TokenKind;
    use crate::frontend::lex::Lexer;

    #[test]
    fn comment() {
        let text = "// some comment";
        let mut ctx = Context::testing();
        let mut lexer = Lexer::new(&mut ctx, text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::EOF);
        assert_eq!(result.span.start, (1, 16, 15));
        assert_eq!(result.span.end, (1, 16, 15));
        assert_eq!(ctx.str_get(result.value()), "")
    }
}
