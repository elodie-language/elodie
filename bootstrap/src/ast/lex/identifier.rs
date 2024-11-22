use crate::ast::lex::Lexer;
use crate::ast::lex::token::{TextSpan, Token, TokenKind};

impl Lexer<'_> {
    pub(crate) fn consume_identifier(&self) -> crate::ast::lex::Result<Token> {
        let start = self.position();

        let mut text = self.consume_while(|c| {
            c.is_alphanumeric() || c == '_'
        })?;

        Ok(Token {
            kind: TokenKind::Identifier,
            span: TextSpan { start, end: self.position(), value: text },
        })
    }
}

#[cfg(test)]
mod test {
    use crate::ast::lex::Lexer;
    use crate::ast::lex::token::{LiteralToken, OperatorToken, TokenKind};

    #[test]
    fn some_var() {
        let text = "some_var";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Identifier);
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 9, 8));
        assert_eq!(result.value(), "some_var");
    }

    #[test]
    fn var() {
        let text = "var";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Identifier);
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 4, 3));
        assert_eq!(result.value(), "var");
    }

    #[test]
    fn console_log() {
        let text = "console.log('test')";
        let lexer = Lexer::new(text);

        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Identifier);
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 8, 7));
        assert_eq!(result.value(), "console");

        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Operator(OperatorToken::Dot));
        assert_eq!(result.span.start, (1, 8, 7));
        assert_eq!(result.span.end, (1, 9, 8));
        assert_eq!(result.value(), ".");

        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Identifier);
        assert_eq!(result.span.start, (1, 9, 8));
        assert_eq!(result.span.end, (1, 12, 11));
        assert_eq!(result.value(), "log");

        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Operator(OperatorToken::OpenParen));
        assert_eq!(result.span.start, (1, 12, 11));
        assert_eq!(result.span.end, (1, 13, 12));
        assert_eq!(result.value(), "(");

        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Literal(LiteralToken::String));
        assert_eq!(result.span.start, (1, 13, 12));
        assert_eq!(result.span.end, (1, 19, 18));
        assert_eq!(result.value(), "test");

        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Operator(OperatorToken::CloseParen));
        assert_eq!(result.span.start, (1, 19, 18));
        assert_eq!(result.span.end, (1, 20, 19));
        assert_eq!(result.value(), ")");

        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::EOF);
        assert_eq!(result.span.start, (1, 20, 19));
        assert_eq!(result.span.end, (1, 20, 19));
        assert_eq!(result.value(), "");
    }
}