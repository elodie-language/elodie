use std::collections::HashMap;

use crate::core::span::TextSpan;
use crate::core::token::{Keyword, Token, TokenKind};
use crate::lexer::Lexer;

impl Lexer<'_> {
    pub(crate) fn is_keyword(&self, c: char) -> bool {
        match c {
            'b' => self.peek_if("break").is_some(),
            'c' => self.peek_if("console_log").is_some() || self.peek_if("const").is_some() || self.peek_if("continue").is_some(),
            'e' => self.peek_if("else").is_some() || self.peek_if("export").is_some(),
            'f' => self.peek_if("from").is_some() || self.peek_if("for").is_some() || self.peek_if("function").is_some(),
            'i' => self.peek_if("if").is_some() || self.peek_if("implement").is_some() || self.peek_if("import").is_some() || self.peek_if("in").is_some(),
            'l' => self.peek_if("let").is_some() || self.peek_if("loop").is_some(),
            'r' => self.peek_if("readonly").is_some() || self.peek_if("return").is_some(),
            't' => self.peek_if("trait").is_some() || self.peek_if("type").is_some(),
            _ => false,
        }
    }

    pub(crate) fn consume_keyword(&self) -> crate::lexer::Result<Token> {
        let start = self.position();

        for (keyword_str, keyword_enum) in Self::keyword_map() {
            if let Some(value) = self.consume_if(keyword_str) {
                self.current_column.borrow_mut().0 += value.len();
                let text = value.to_string();

                return Ok(Token {
                    kind: TokenKind::Keyword(keyword_enum),
                    span: TextSpan { start, end: self.position(), text },
                });
            }
        }

        Err(crate::lexer::Error::UnknownSeparator("".to_string()))
    }


    fn keyword_map() -> HashMap<&'static str, Keyword> {
        let mut keywords = HashMap::new();
        keywords.insert("break", Keyword::Break);
        keywords.insert("console_log", Keyword::ConsoleLog);
        keywords.insert("const", Keyword::Const);
        keywords.insert("continue", Keyword::Continue);
        keywords.insert("else", Keyword::Else);
        keywords.insert("export", Keyword::Export);
        keywords.insert("from", Keyword::From);
        keywords.insert("for", Keyword::For);
        keywords.insert("function", Keyword::Function);
        keywords.insert("if", Keyword::If);
        keywords.insert("implement", Keyword::Implement);
        keywords.insert("import", Keyword::Import);
        keywords.insert("in", Keyword::In);
        keywords.insert("let", Keyword::Let);
        keywords.insert("loop", Keyword::Loop);
        keywords.insert("readonly", Keyword::Readonly);
        keywords.insert("return", Keyword::Return);
        keywords.insert("trait", Keyword::Trait);
        keywords.insert("type", Keyword::Type);
        keywords
    }
}

#[cfg(test)]
mod test {
    use crate::core::token::{Keyword, TokenKind};
    use crate::lexer::Lexer;

    #[test]
    fn r#break() {
        let text = "break";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Keyword(Keyword::Break));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 6, 5));
        assert_eq!(result.span.text, "break");
    }

    #[test]
    fn r#console_log() {
        let text = "console_log";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Keyword(Keyword::ConsoleLog));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 12, 11));
        assert_eq!(result.span.text, "console_log");
    }

    #[test]
    fn r#const() {
        let text = "const";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Keyword(Keyword::Const));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 6, 5));
        assert_eq!(result.span.text, "const");
    }

    #[test]
    fn r#continue() {
        let text = "continue";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Keyword(Keyword::Continue));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 9, 8));
        assert_eq!(result.span.text, "continue");
    }

    #[test]
    fn r#else() {
        let text = "else";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Keyword(Keyword::Else));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 5, 4));
        assert_eq!(result.span.text, "else");
    }

    #[test]
    fn r#export() {
        let text = "export";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Keyword(Keyword::Export));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 7, 6));
        assert_eq!(result.span.text, "export");
    }

    #[test]
    fn r#from() {
        let text = "from";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Keyword(Keyword::From));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 5, 4));
        assert_eq!(result.span.text, "from");
    }

    #[test]
    fn r#for() {
        let text = "for";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Keyword(Keyword::For));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 4, 3));
        assert_eq!(result.span.text, "for");
    }

    #[test]
    fn r#function() {
        let text = "function";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Keyword(Keyword::Function));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 9, 8));
        assert_eq!(result.span.text, "function");
    }

    #[test]
    fn r#if() {
        let text = "if";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Keyword(Keyword::If));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 3, 2));
        assert_eq!(result.span.text, "if");
    }

    #[test]
    fn r#implement() {
        let text = "implement";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Keyword(Keyword::Implement));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 10, 9));
        assert_eq!(result.span.text, "implement");
    }

    #[test]
    fn r#import() {
        let text = "import";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Keyword(Keyword::Import));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 7, 6));
        assert_eq!(result.span.text, "import");
    }

    #[test]
    fn r#in() {
        let text = "in";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Keyword(Keyword::In));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 3, 2));
        assert_eq!(result.span.text, "in");
    }

    #[test]
    fn r#let() {
        let text = "let";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Keyword(Keyword::Let));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 4, 3));
        assert_eq!(result.span.text, "let");
    }

    #[test]
    fn r#loop() {
        let text = "loop";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Keyword(Keyword::Loop));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 5, 4));
        assert_eq!(result.span.text, "loop");
    }

    #[test]
    fn r#readonly() {
        let text = "readonly";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Keyword(Keyword::Readonly));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 9, 8));
        assert_eq!(result.span.text, "readonly");
    }

    #[test]
    fn r#return() {
        let text = "return";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Keyword(Keyword::Return));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 7, 6));
        assert_eq!(result.span.text, "return");
    }

    #[test]
    fn r#trait() {
        let text = "trait";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Keyword(Keyword::Trait));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 6, 5));
        assert_eq!(result.span.text, "trait");
    }

    #[test]
    fn r#type() {
        let text = "type";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Keyword(Keyword::Type));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 5, 4));
        assert_eq!(result.span.text, "type");
    }
}
