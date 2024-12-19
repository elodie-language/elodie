use crate::frontend::lex::Lexer;
use crate::frontend::lex::token::{LiteralToken, TextSpan, Token, TokenKind};
use crate::frontend::lex::token::LiteralToken::{False, Number, True};

impl Lexer<'_> {
    pub(crate) fn is_string(&self, c: char) -> bool {
        c == '\''
    }
    pub(crate) fn consume_string(&mut self) -> crate::frontend::lex::Result<Token> {
        let start = self.position();
        let mut text = String::from("");
        let next = String::from(self.consume_next()?);
        assert_eq!(next, "\'");

        loop {
            let next = self.consume_next()?;
            if next == '\\' {
                if self.peek_if("'").is_some() {
                    let _ = self.consume_next()?;
                    text.push_str("'");
                    continue;
                }
            }

            if next == '$' && self.peek_if("{").is_some() {
                text.push('$');
                text.push('{');

                self.consume_next()?; // Consume '{'

                loop {
                    let next = self.consume_next()?;
                    if next == '}' {
                        text.push('}');
                        break;
                    }
                    text.push(next);
                }

                continue;
            }

            if next == '\'' {
                break;
            }

            text.push(next);
        }

        Ok(Token {
            kind: TokenKind::Literal(LiteralToken::String),
            span: TextSpan {
                start,
                end: self.position(),
                value: self.ctx.string_table.push_str(text.as_str()),
            },
        })
    }

    pub(crate) fn is_number(&self, c: char) -> bool {
        c.is_digit(10)
    }

    pub(crate) fn consume_number(&mut self) -> crate::frontend::lex::Result<Token> {
        let start = self.position();
        let mut text = String::from("");
        let next = String::from(self.consume_next()?);

        if next == "0" {
            if let Some(c) = self.peek_next() {
                match c {
                    'x' | 'X' => {
                        self.consume_next()?;
                        text.push_str("0x");
                        text.push_str(&*self.consume_while(|c| c.is_digit(16))?);
                    }
                    'o' | 'O' => {
                        self.consume_next()?;
                        text.push_str("0o");
                        text.push_str(&*self.consume_while(|c| c.is_digit(8))?);
                    }
                    'b' | 'B' => {
                        self.consume_next()?;
                        text.push_str("0b");
                        text.push_str(&*self.consume_while(|c| c.is_digit(2))?);
                    }
                    c if c.is_digit(10) => {
                        text.push_str(&*self.consume_while(|c| c.is_digit(10))?);
                    }
                    _ => {
                        text.push_str("0");
                    }
                }
            }
        } else {
            text.push_str(&next);
            let next = self.consume_while(|c| c.is_digit(10))?;
            text.push_str(&next);

            if let Some('.') = self.peek_next() {
                let _ = self.consume_next()?;
                text.push('.');
                let next = self.consume_while(|c| c.is_digit(10))?;
                text.push_str(&next);
            }
        }

        Ok(Token {
            kind: TokenKind::Literal(Number),
            span: TextSpan {
                start,
                end: self.position(),
                value: self.ctx.string_table.push_str(text.as_str()),
            },
        })
    }

    pub(crate) fn is_bool(&self, c: char) -> bool {
        if c != 't' && c != 'f' {
            return false;
        }
        let look_ahead = self.look_ahead().unwrap();
        return matches!(look_ahead.as_str(), "true" | "false");
    }

    pub(crate) fn consume_bool(&mut self) -> crate::frontend::lex::Result<Token> {
        let start = self.position();
        let next = self.consume_next()?;
        if next == 't' {
            self.consume_if("rue").unwrap();
            return Ok(Token {
                kind: TokenKind::Literal(True),
                span: TextSpan {
                    start,
                    end: self.position(),
                    value: self.ctx.string_table.push_str("true"),
                },
            });
        }

        assert_eq!(next, 'f');
        self.consume_if("alse").unwrap();
        Ok(Token {
            kind: TokenKind::Literal(False),
            span: TextSpan {
                start,
                end: self.position(),
                value: self.ctx.string_table.push_str("false"),
            },
        })
    }
}

#[cfg(test)]
mod test {
    use crate::frontend::context::Context;
    use crate::frontend::lex::Lexer;
    use crate::frontend::lex::token::LiteralToken::{False, Number, String, True};
    use crate::frontend::lex::token::TokenKind;

    #[test]
    fn empty_string() {
        let text = "''";
        let mut ctx = Context::new();
        let mut lexer = Lexer::new(&mut ctx, text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Literal(String));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 3, 2));
        assert_eq!(ctx.get_str(result.value()), "");
    }

    #[test]
    fn hello_elodie() {
        let text = "'Hello Elodie'";
        let mut ctx = Context::new();
        let mut lexer = Lexer::new(&mut ctx, text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Literal(String));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 15, 14));
        assert_eq!(ctx.get_str(result.value()), "Hello Elodie");
    }

    #[test]
    fn string_with_interpolation() {
        let text = "'${'test'}'";
        let mut ctx = Context::new();
        let mut lexer = Lexer::new(&mut ctx, text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Literal(String));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 12, 11));
        assert_eq!(ctx.get_str(result.value()), "${'test'}");
    }

    #[test]
    fn escaped_string() {
        let text = "'{\\'hello\\':\\'world\\'}'";
        let mut ctx = Context::new();
        let mut lexer = Lexer::new(&mut ctx, text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Literal(String));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 24, 23));
        assert_eq!(ctx.get_str(result.value()), "{'hello':'world'}");
    }

    #[test]
    fn nat() {
        let text = "42";
        let mut ctx = Context::new();
        let mut lexer = Lexer::new(&mut ctx, text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Literal(Number));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 3, 2));
        assert_eq!(ctx.get_str(result.value()), "42");
    }

    #[test]
    fn float() {
        let text = "42.24";
        let mut ctx = Context::new();
        let mut lexer = Lexer::new(&mut ctx, text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Literal(Number));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 6, 5));
        assert_eq!(ctx.get_str(result.value()), "42.24");
    }

    #[test]
    fn hex() {
        let text = "0xDEADBEEF";
        let mut ctx = Context::new();
        let mut lexer = Lexer::new(&mut ctx, text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Literal(Number));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 11, 10));
        assert_eq!(ctx.get_str(result.value()), "0xDEADBEEF");
    }

    #[test]
    fn octal() {
        let text = "0o10";
        let mut ctx = Context::new();
        let mut lexer = Lexer::new(&mut ctx, text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Literal(Number));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 5, 4));
        assert_eq!(ctx.get_str(result.value()), "0o10");
    }

    #[test]
    fn binary() {
        let text = "0b10101";
        let mut ctx = Context::new();
        let mut lexer = Lexer::new(&mut ctx, text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Literal(Number));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 8, 7));
        assert_eq!(ctx.get_str(result.value()), "0b10101");
    }

    #[test]
    fn r#true() {
        let text = "true";
        let mut ctx = Context::new();
        let mut lexer = Lexer::new(&mut ctx, text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Literal(True));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 5, 4));
        assert_eq!(ctx.get_str(result.value()), "true");
    }

    #[test]
    fn not_true() {
        let text = "true_";
        let mut ctx = Context::new();
        let mut lexer = Lexer::new(&mut ctx, text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Identifier);
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 6, 5));
        assert_eq!(ctx.get_str(result.value()), "true_");
    }

    #[test]
    fn r#false() {
        let text = "false";
        let mut ctx = Context::new();
        let mut lexer = Lexer::new(&mut ctx, text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Literal(False));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 6, 5));
        assert_eq!(ctx.get_str(result.value()), "false");
    }

    #[test]
    fn not_false() {
        let text = "false_";
        let mut ctx = Context::new();
        let mut lexer = Lexer::new(&mut ctx, text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Identifier);
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 7, 6));
        assert_eq!(ctx.get_str(result.value()), "false_");
    }
}
