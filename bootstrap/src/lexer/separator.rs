use crate::core::span::TextSpan;
use crate::core::token::{Token, TokenKind};
use crate::core::token::Separator::Whitespace;
use crate::lexer::Lexer;

impl Lexer<'_> {
    pub(crate) fn is_whitespace(&self, c: char) -> bool {
        match c {
            | '\u{0009}' // \t
            | '\u{000B}' // vertical tab
            | '\u{000C}' // form feed
            | '\u{000D}' // \r
            | '\u{0020}' // space
            => true,
            _ => false
        }
    }

    pub(crate) fn consume_whitespace(&self) -> crate::lexer::Result<Token> {
        let start = self.position();
        let text = self.consume_while(|c| self.is_whitespace(c))?;
        let end = self.position();

        Ok(Token { kind: TokenKind::Separator(Whitespace), span: TextSpan { start, end, text } })
    }
}


#[cfg(test)]
mod test {
    use Separator::Comma;

    use crate::core::token::{Separator, TokenKind};
    use crate::core::token::Separator::Whitespace;
    use crate::lexer::Lexer;

    #[test]
    fn tab() {
        let text = "\t";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Separator(Whitespace));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 2, 1));
        assert_eq!(result.span.text, "\t")
    }

    #[test]
    fn whitespace() {
        let text = "     ";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Separator(Whitespace));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 6, 5));
        assert_eq!(result.span.text, "     ")
    }

    #[test]
    fn comma() {
        let text = ",";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Separator(Comma));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 2, 1));
        assert_eq!(result.span.text, ",");
    }
}