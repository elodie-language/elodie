use crate::core::span::TextSpan;
use crate::core::token::{Token, TokenKind};
use crate::lexer::Lexer;

impl Lexer<'_> {
    pub(crate) fn is_comment(&self, c: char) -> bool {
        c == '/' && self.peek_if("//").is_some()
    }

    pub(crate) fn consume_comment(&self) -> crate::lexer::Result<Token> {
        let start = self.position();
        let text = self.consume_while(|c| c != '\n')?;
        let end = self.position();

        Ok(Token { kind: TokenKind::Comment, span: TextSpan { start, end, text } })
    }
}


#[cfg(test)]
mod test {
    use crate::core::token::TokenKind;
    use crate::lexer::Lexer;

    #[test]
    fn comment() {
        let text = "// some comment";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Comment);
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 16, 15));
        assert_eq!(result.span.text, "// some comment")
    }
}