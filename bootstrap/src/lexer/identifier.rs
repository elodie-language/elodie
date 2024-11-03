use crate::core::span::TextSpan;
use crate::core::token::{Token, TokenKind};
use crate::core::token::Literal::Identifier;
use crate::lexer::Lexer;

impl Lexer<'_> {
    pub(crate) fn consume_identifier(&self) -> crate::lexer::Result<Token> {
        let start = self.position();

        let mut text = self.consume_while(|c| {
            c.is_alphanumeric() || c == '_'
        })?;

        Ok(Token {
            kind: TokenKind::Literal(Identifier),
            span: TextSpan { start, end: self.position(), text },
        })
    }
}

#[cfg(test)]
mod test {
    use crate::core::token::{Literal, TokenKind};
    use crate::lexer::Lexer;

    #[test]
    fn some_var() {
        let text = "some_var";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Literal(Literal::Identifier));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 9, 8));
        assert_eq!(result.span.text, "some_var");
    }

    #[test]
    fn var() {
        let text = "var";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Literal(Literal::Identifier));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 4, 3));
        assert_eq!(result.span.text, "var");
    }
}