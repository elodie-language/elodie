use std::collections::HashMap;

use crate::core::span::TextSpan;
use crate::core::token::{Keyword, Token, TokenKind};
use crate::lexer::Lexer;

impl Lexer<'_> {
    pub(crate) fn is_keyword(&self, c: char) -> bool {
        let look_ahead = self.peek_while(|c|c.is_ascii_alphanumeric()).unwrap();

        match c {
            'b' => look_ahead == "break",
            'c' => matches!(look_ahead.as_str(), "console_log" | "const" | "continue"),
            'e' => matches!(look_ahead.as_str(), "else" | "export"),
            'f' => matches!(look_ahead.as_str(), "from" | "for" | "function"),
            'i' => matches!(look_ahead.as_str(), "if" | "implement" | "import" | "in"),
            'l' => matches!(look_ahead.as_str(), "let" | "loop"),
            'r' => matches!(look_ahead.as_str(), "readonly" | "return"),
            't' => matches!(look_ahead.as_str(), "trait" | "type"),
            _ => false,
        }
    }

    pub(crate) fn consume_keyword(&self) -> crate::lexer::Result<Token> {
        let start = self.position();

        for (keyword_str, keyword_enum) in Self::keyword_map() {

            if let Some(value) = self.consume_if(keyword_str) {
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
    fn not_break() {
        let text = "breaker";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Identifier);
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 8, 7));
        assert_eq!(result.span.text, "breaker");
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
    fn not_const() {
        let text = "constant";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Identifier);
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 9, 8));
        assert_eq!(result.span.text, "constant");
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
    fn not_continue() {
        let text = "continuation";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Identifier);
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 13, 12));
        assert_eq!(result.span.text, "continuation");
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
    fn not_else() {
        let text = "elsewhere";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Identifier);
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 10, 9));
        assert_eq!(result.span.text, "elsewhere");
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
    fn not_export() {
        let text = "exporting";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Identifier);
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 10, 9));
        assert_eq!(result.span.text, "exporting");
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
    fn not_from() {
        let text = "fromage";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Identifier);
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 8, 7));
        assert_eq!(result.span.text, "fromage");
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
    fn not_for() {
        let text = "formal";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Identifier);
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 7, 6));
        assert_eq!(result.span.text, "formal");
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
    fn not_function() {
        let text = "functional";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Identifier);
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 11, 10));
        assert_eq!(result.span.text, "functional");
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
    fn not_if() {
        let text = "iffy";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Identifier);
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 5, 4));
        assert_eq!(result.span.text, "iffy");
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
    fn not_implement() {
        let text = "implementation";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Identifier);
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 15, 14));
        assert_eq!(result.span.text, "implementation");
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
    fn not_import() {
        let text = "importance";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Identifier);
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 11, 10));
        assert_eq!(result.span.text, "importance");
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
    fn not_in() {
        let text = "inner";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Identifier);
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 6, 5));
        assert_eq!(result.span.text, "inner");
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
    fn not_let() {
        let text = "letter";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Identifier);
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 7, 6));
        assert_eq!(result.span.text, "letter");
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
    fn not_loop() {
        let text = "loophole";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Identifier);
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 9, 8));
        assert_eq!(result.span.text, "loophole");
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
    fn not_readonly() {
        let text = "readonlyness";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Identifier);
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 13, 12));
        assert_eq!(result.span.text, "readonlyness");
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
    fn not_return() {
        let text = "returns";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Identifier);
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 8, 7));
        assert_eq!(result.span.text, "returns");
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
    fn not_trait() {
        let text = "traitor";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Identifier);
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 8, 7));
        assert_eq!(result.span.text, "traitor");
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

    #[test]
    fn not_type() {
        let text = "typeset";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Identifier);
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 8, 7));
        assert_eq!(result.span.text, "typeset");
    }
}
