use crate::frontend::lex::Lexer;
use crate::frontend::lex::token::{TextSpan, Token, TokenKind};

impl Lexer<'_> {
    pub(crate) fn consume_identifier(&mut self) -> crate::frontend::lex::Result<Token> {
        let start = self.position();

        let mut text = self.consume_while(|c| c.is_alphanumeric() || c == '_')?;

        Ok(Token {
            kind: TokenKind::Identifier,
            span: TextSpan {
                start,
                end: self.position(),
                value: self.ctx.string_table.push_str(text.as_str()),
            },
        })
    }
}

#[cfg(test)]
mod test {
    use crate::frontend::context::Context;
    use crate::frontend::lex::Lexer;
    use crate::frontend::lex::token::{LiteralToken, OperatorToken, TokenKind};

    #[test]
    fn some_var() {
        let text = "some_var";
        let mut ctx = Context::new();
        let mut lexer = Lexer::new(&mut ctx, text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Identifier);
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 9, 8));
        assert_eq!(ctx.get_str(result.value()), "some_var");
    }

    #[test]
    fn var() {
        let text = "var";
        let mut ctx = Context::new();
        let mut lexer = Lexer::new(&mut ctx, text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Identifier);
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 4, 3));
        assert_eq!(ctx.get_str(result.value()), "var");
    }

    #[test]
    fn console_log() {
        let text = "console.log('test')";
        let mut ctx = Context::new();
        let mut lexer = Lexer::new(&mut ctx, text);

        let token_one = lexer.advance().unwrap();
        let token_two = lexer.advance().unwrap();
        let token_three = lexer.advance().unwrap();
        let token_four = lexer.advance().unwrap();
        let token_five = lexer.advance().unwrap();
        let token_six = lexer.advance().unwrap();
        let token_seven = lexer.advance().unwrap();

        assert_eq!(token_one.kind, TokenKind::Identifier);
        assert_eq!(token_one.span.start, (1, 1, 0));
        assert_eq!(token_one.span.end, (1, 8, 7));
        assert_eq!(ctx.get_str(token_one.value()), "console");

        assert_eq!(token_two.kind, TokenKind::Operator(OperatorToken::Dot));
        assert_eq!(token_two.span.start, (1, 8, 7));
        assert_eq!(token_two.span.end, (1, 9, 8));
        assert_eq!(ctx.get_str(token_two.value()), ".");

        assert_eq!(token_three.kind, TokenKind::Identifier);
        assert_eq!(token_three.span.start, (1, 9, 8));
        assert_eq!(token_three.span.end, (1, 12, 11));
        assert_eq!(ctx.get_str(token_three.value()), "log");

        assert_eq!(
            token_four.kind,
            TokenKind::Operator(OperatorToken::OpenParen)
        );
        assert_eq!(token_four.span.start, (1, 12, 11));
        assert_eq!(token_four.span.end, (1, 13, 12));
        assert_eq!(ctx.get_str(token_four.value()), "(");

        assert_eq!(token_five.kind, TokenKind::Literal(LiteralToken::String));
        assert_eq!(token_five.span.start, (1, 13, 12));
        assert_eq!(token_five.span.end, (1, 19, 18));
        assert_eq!(ctx.get_str(token_five.value()), "test");

        assert_eq!(
            token_six.kind,
            TokenKind::Operator(OperatorToken::CloseParen)
        );
        assert_eq!(token_six.span.start, (1, 19, 18));
        assert_eq!(token_six.span.end, (1, 20, 19));
        assert_eq!(ctx.get_str(token_six.value()), ")");

        assert_eq!(token_seven.kind, TokenKind::EOF);
        assert_eq!(token_seven.span.start, (1, 20, 19));
        assert_eq!(token_seven.span.end, (1, 20, 19));
        assert_eq!(ctx.get_str(token_seven.value()), "");
    }
}
