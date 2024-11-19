use std::collections::HashMap;

use crate::ast::lex::Lexer;
use crate::ast::token::{KeywordToken, TextSpan, Token, TokenKind};

impl Lexer<'_> {
    pub(crate) fn is_keyword(&self, c: char) -> bool {
        let look_ahead = self.look_ahead().unwrap();

        match c {
            'b' => look_ahead == "break",
            'c' => matches!(look_ahead.as_str(), "console_log" | "const" | "continue"),
            'e' => matches!(look_ahead.as_str(), "else" | "export"),
            'f' => matches!(look_ahead.as_str(), "from" | "for" | "fun"),
            'i' => matches!(look_ahead.as_str(), "if" | "implement" | "import" | "in"),
            'l' => matches!(look_ahead.as_str(), "let" | "loop"),
            'p' => look_ahead == "package",
            'r' => matches!(look_ahead.as_str(), "readonly" | "return"),
            't' => matches!(look_ahead.as_str(), "trait" | "type"),
            _ => false,
        }
    }

    pub(crate) fn consume_keyword(&self) -> crate::ast::lex::Result<Token> {
        let start = self.position();

        for (keyword_str, keyword_enum) in Self::keyword_map() {
            if let Some(value) = self.consume_if(keyword_str) {
                let text = value.to_string();

                return Ok(Token {
                    kind: TokenKind::Keyword(keyword_enum),
                    span: TextSpan { start, end: self.position(), value: text },
                });
            }
        }

        Err(crate::ast::lex::Error::UnknownSeparator("".to_string()))
    }


    fn keyword_map() -> HashMap<&'static str, KeywordToken> {
        let mut keywords = HashMap::new();
        keywords.insert("break", KeywordToken::Break);
        keywords.insert("const", KeywordToken::Const);
        keywords.insert("continue", KeywordToken::Continue);
        keywords.insert("else", KeywordToken::Else);
        keywords.insert("export", KeywordToken::Export);
        keywords.insert("from", KeywordToken::From);
        keywords.insert("for", KeywordToken::For);
        keywords.insert("fun", KeywordToken::Function);
        keywords.insert("if", KeywordToken::If);
        keywords.insert("implement", KeywordToken::Implement);
        keywords.insert("import", KeywordToken::Import);
        keywords.insert("in", KeywordToken::In);
        keywords.insert("let", KeywordToken::Let);
        keywords.insert("loop", KeywordToken::Loop);
        keywords.insert("package", KeywordToken::Package);
        keywords.insert("readonly", KeywordToken::Readonly);
        keywords.insert("return", KeywordToken::Return);
        keywords.insert("trait", KeywordToken::Trait);
        keywords.insert("type", KeywordToken::Type);
        keywords
    }
}

#[cfg(test)]
mod test {
    use KeywordToken::*;

    use crate::ast::lex::Lexer;
    use crate::ast::token::{identifier, keyword, KeywordToken};
    use crate::ast::token::KeywordToken::If;

    #[test]
    fn r#break() {
        let text = "break";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert!(result.is_keyword(Break));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 6, 5));
        assert_eq!(result.value(), "break");
    }

    #[test]
    fn not_break() {
        let text = "breaker";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, identifier());
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 8, 7));
        assert_eq!(result.value(), "breaker");
    }

    #[test]
    fn r#const() {
        let text = "const";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert!(result.is_keyword(Const));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 6, 5));
        assert_eq!(result.value(), "const");
    }

    #[test]
    fn not_const() {
        let text = "constant";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, identifier());
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 9, 8));
        assert_eq!(result.value(), "constant");
    }

    #[test]
    fn r#continue() {
        let text = "continue";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert!(result.is_keyword(Continue));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 9, 8));
        assert_eq!(result.value(), "continue");
    }

    #[test]
    fn not_continue() {
        let text = "continuation";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, identifier());
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 13, 12));
        assert_eq!(result.value(), "continuation");
    }

    #[test]
    fn r#else() {
        let text = "else";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert!(result.is_keyword(Else));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 5, 4));
        assert_eq!(result.value(), "else");
    }

    #[test]
    fn not_else() {
        let text = "elsewhere";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, identifier());
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 10, 9));
        assert_eq!(result.value(), "elsewhere");
    }

    #[test]
    fn r#export() {
        let text = "export";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert!(result.is_keyword(Export));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 7, 6));
        assert_eq!(result.value(), "export");
    }

    #[test]
    fn not_export() {
        let text = "exporting";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, identifier());
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 10, 9));
        assert_eq!(result.value(), "exporting");
    }

    #[test]
    fn r#from() {
        let text = "from";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert!(result.is_keyword(From));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 5, 4));
        assert_eq!(result.value(), "from");
    }

    #[test]
    fn not_from() {
        let text = "fromage";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, identifier());
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 8, 7));
        assert_eq!(result.value(), "fromage");
    }

    #[test]
    fn r#for() {
        let text = "for";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert!(result.is_keyword(For));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 4, 3));
        assert_eq!(result.value(), "for");
    }

    #[test]
    fn not_for() {
        let text = "formal";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, identifier());
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 7, 6));
        assert_eq!(result.value(), "formal");
    }

    #[test]
    fn r#function() {
        let text = "fun";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert!(result.is_keyword(Function));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 4, 3));
        assert_eq!(result.value(), "fun");
    }

    #[test]
    fn not_function() {
        let text = "func";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, identifier());
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 5, 4));
        assert_eq!(result.value(), "func");
    }

    #[test]
    fn r#if() {
        let text = "if";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, keyword(If));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 3, 2));
        assert_eq!(result.value(), "if");
    }

    #[test]
    fn not_if() {
        let text = "iffy";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, identifier());
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 5, 4));
        assert_eq!(result.value(), "iffy");
    }

    #[test]
    fn r#implement() {
        let text = "implement";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert!(result.is_keyword(Implement));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 10, 9));
        assert_eq!(result.value(), "implement");
    }

    #[test]
    fn not_implement() {
        let text = "implementation";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, identifier());
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 15, 14));
        assert_eq!(result.value(), "implementation");
    }

    #[test]
    fn r#import() {
        let text = "import";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert!(result.is_keyword(Import));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 7, 6));
        assert_eq!(result.value(), "import");
    }

    #[test]
    fn not_import() {
        let text = "importance";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, identifier());
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 11, 10));
        assert_eq!(result.value(), "importance");
    }

    #[test]
    fn r#in() {
        let text = "in";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert!(result.is_keyword(In));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 3, 2));
        assert_eq!(result.value(), "in");
    }

    #[test]
    fn not_in() {
        let text = "inner";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, identifier());
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 6, 5));
        assert_eq!(result.value(), "inner");
    }

    #[test]
    fn r#let() {
        let text = "let";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert!(result.is_keyword(Let));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 4, 3));
        assert_eq!(result.value(), "let");
    }

    #[test]
    fn not_let() {
        let text = "letter";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, identifier());
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 7, 6));
        assert_eq!(result.value(), "letter");
    }

    #[test]
    fn r#loop() {
        let text = "loop";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert!(result.is_keyword(Loop));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 5, 4));
        assert_eq!(result.value(), "loop");
    }

    #[test]
    fn not_loop() {
        let text = "loophole";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, identifier());
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 9, 8));
        assert_eq!(result.value(), "loophole");
    }

    #[test]
    fn package() {
        let text = "package";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert!(result.is_keyword(Package));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 8, 7));
        assert_eq!(result.value(), "package");
    }

    #[test]
    fn not_package() {
        let text = "packaged";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, identifier());
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 9, 8));
        assert_eq!(result.value(), "packaged");
    }

    #[test]
    fn readonly() {
        let text = "readonly";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert!(result.is_keyword(Readonly));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 9, 8));
        assert_eq!(result.value(), "readonly");
    }

    #[test]
    fn not_readonly() {
        let text = "readonlyness";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, identifier());
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 13, 12));
        assert_eq!(result.value(), "readonlyness");
    }

    #[test]
    fn r#return() {
        let text = "return";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert!(result.is_keyword(Return));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 7, 6));
        assert_eq!(result.value(), "return");
    }


    #[test]
    fn not_return() {
        let text = "returns";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, identifier());
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 8, 7));
        assert_eq!(result.value(), "returns");
    }

    #[test]
    fn r#trait() {
        let text = "trait";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert!(result.is_keyword(Trait));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 6, 5));
        assert_eq!(result.value(), "trait");
    }

    #[test]
    fn not_trait() {
        let text = "traitor";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, identifier());
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 8, 7));
        assert_eq!(result.value(), "traitor");
    }

    #[test]
    fn r#type() {
        let text = "type";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert!(result.is_keyword(Type));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 5, 4));
        assert_eq!(result.value(), "type");
    }

    #[test]
    fn not_type() {
        let text = "typeset";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, identifier());
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 8, 7));
        assert_eq!(result.value(), "typeset");
    }
}
