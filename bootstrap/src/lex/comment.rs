use crate::lex::Lexer;

impl Lexer<'_> {
    pub(crate) fn is_comment(&self, c: char) -> bool {
        c == '/' && self.peek_if("//").is_some()
    }

    pub(crate) fn consume_comment(&self) -> crate::lex::Result<()> {
        self.consume_while(|c| c != '\n')?;
        Ok(())
    }
}


#[cfg(test)]
mod test {
    use crate::lex::Lexer;
    use crate::lex::token::TokenKind;

    #[test]
    fn comment() {
        let text = "// some comment";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::EOF);
        assert_eq!(result.span.start, (1, 16, 15));
        assert_eq!(result.span.end, (1, 16, 15));
        assert_eq!(result.value(), "")
    }
}