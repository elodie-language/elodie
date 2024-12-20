use std::collections::HashMap;

use crate::common::Span;
use crate::frontend::lex::Lexer;
use crate::frontend::lex::token::{KeywordToken, Token, TokenKind};

impl Lexer<'_> {
    pub(crate) fn is_keyword(&self, c: char) -> bool {
        let look_ahead = self.look_ahead().unwrap();

        match c {
            'b' => look_ahead == "break",
            'c' => matches!(look_ahead.as_str(), "const" | "continue"),
            'd' => look_ahead == "define",
            'e' => matches!(look_ahead.as_str(), "else" | "export" | "external"),
            'f' => matches!(look_ahead.as_str(), "from" | "for" | "function"),
            'i' => matches!(look_ahead.as_str(), "if" | "import" | "in"),
            'l' => matches!(look_ahead.as_str(), "let" | "loop"),
            'p' => look_ahead == "package",
            'r' => matches!(look_ahead.as_str(), "readonly" | "return"),
            's' => look_ahead == "self",
            't' => matches!(look_ahead.as_str(), "trait" | "type"),
            _ => false,
        }
    }

    pub(crate) fn consume_keyword(&mut self) -> crate::frontend::lex::Result<Token> {
        let start = self.position();

        for (keyword_str, keyword_enum) in Self::keyword_map() {
            if let Some(value) = self.consume_if(keyword_str) {
                let text = value.to_string();

                return Ok(Token {
                    kind: TokenKind::Keyword(keyword_enum),
                    span: Span {
                        start,
                        end: self.position(),
                    },
                    value: self.ctx.string_table.push_str(text.as_str()),
                });
            }
        }

        Err(crate::frontend::lex::Error::UnknownSeparator(
            "".to_string(),
        ))
    }

    fn keyword_map() -> HashMap<&'static str, KeywordToken> {
        let mut keywords = HashMap::new();
        keywords.insert("break", KeywordToken::Break);
        keywords.insert("const", KeywordToken::Const);
        keywords.insert("continue", KeywordToken::Continue);
        keywords.insert("define", KeywordToken::Define);
        keywords.insert("else", KeywordToken::Else);
        keywords.insert("export", KeywordToken::Export);
        keywords.insert("external", KeywordToken::External);
        keywords.insert("from", KeywordToken::From);
        keywords.insert("for", KeywordToken::For);
        keywords.insert("function", KeywordToken::Function);
        keywords.insert("if", KeywordToken::If);
        keywords.insert("import", KeywordToken::Import);
        keywords.insert("in", KeywordToken::In);
        keywords.insert("let", KeywordToken::Let);
        keywords.insert("loop", KeywordToken::Loop);
        keywords.insert("package", KeywordToken::Package);
        keywords.insert("readonly", KeywordToken::Readonly);
        keywords.insert("return", KeywordToken::Return);
        keywords.insert("self", KeywordToken::Itself);
        keywords.insert("trait", KeywordToken::Trait);
        keywords.insert("type", KeywordToken::Type);
        keywords
    }
}

#[cfg(test)]
mod test {
    use KeywordToken::*;

    use crate::frontend::context::Context;
    use crate::frontend::lex::Lexer;
    use crate::frontend::lex::token::{identifier, keyword, KeywordToken};
    use crate::frontend::lex::token::KeywordToken::If;

    #[test]
    fn r#break() {
        let text = "break";
        let mut ctx = Context::new();
        let mut lexer = Lexer::new(&mut ctx, text);
        let result = lexer.advance().unwrap();
        assert!(result.is_keyword(Break));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 6, 5));
        assert_eq!(ctx.get_str(result.value()), "break");
    }

    #[test]
    fn not_break() {
        let text = "breaker";
        let mut ctx = Context::new();
        let mut lexer = Lexer::new(&mut ctx, text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, identifier());
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 8, 7));
        assert_eq!(ctx.get_str(result.value()), "breaker");
    }

    #[test]
    fn r#const() {
        let text = "const";
        let mut ctx = Context::new();
        let mut lexer = Lexer::new(&mut ctx, text);
        let result = lexer.advance().unwrap();
        assert!(result.is_keyword(Const));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 6, 5));
        assert_eq!(ctx.get_str(result.value()), "const");
    }

    #[test]
    fn not_const() {
        let text = "constant";
        let mut ctx = Context::new();
        let mut lexer = Lexer::new(&mut ctx, text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, identifier());
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 9, 8));
        assert_eq!(ctx.get_str(result.value()), "constant");
    }

    #[test]
    fn r#continue() {
        let text = "continue";
        let mut ctx = Context::new();
        let mut lexer = Lexer::new(&mut ctx, text);
        let result = lexer.advance().unwrap();
        assert!(result.is_keyword(Continue));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 9, 8));
        assert_eq!(ctx.get_str(result.value()), "continue");
    }

    #[test]
    fn not_continue() {
        let text = "continuation";
        let mut ctx = Context::new();
        let mut lexer = Lexer::new(&mut ctx, text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, identifier());
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 13, 12));
        assert_eq!(ctx.get_str(result.value()), "continuation");
    }

    #[test]
    fn define() {
        let text = "define";
        let mut ctx = Context::new();
        let mut lexer = Lexer::new(&mut ctx, text);
        let result = lexer.advance().unwrap();
        assert!(result.is_keyword(Define));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 7, 6));
        assert_eq!(ctx.get_str(result.value()), "define");
    }

    #[test]
    fn not_define() {
        let text = "defined";
        let mut ctx = Context::new();
        let mut lexer = Lexer::new(&mut ctx, text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, identifier());
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 8, 7));
        assert_eq!(ctx.get_str(result.value()), "defined");
    }

    #[test]
    fn r#else() {
        let text = "else";
        let mut ctx = Context::new();
        let mut lexer = Lexer::new(&mut ctx, text);
        let result = lexer.advance().unwrap();
        assert!(result.is_keyword(Else));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 5, 4));
        assert_eq!(ctx.get_str(result.value()), "else");
    }

    #[test]
    fn not_else() {
        let text = "elsewhere";
        let mut ctx = Context::new();
        let mut lexer = Lexer::new(&mut ctx, text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, identifier());
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 10, 9));
        assert_eq!(ctx.get_str(result.value()), "elsewhere");
    }

    #[test]
    fn external() {
        let text = "external";
        let mut ctx = Context::new();
        let mut lexer = Lexer::new(&mut ctx, text);
        let result = lexer.advance().unwrap();
        assert!(result.is_keyword(External));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 9, 8));
        assert_eq!(ctx.get_str(result.value()), "external");
    }

    #[test]
    fn not_external() {
        let text = "externald";
        let mut ctx = Context::new();
        let mut lexer = Lexer::new(&mut ctx, text);
        let result = lexer.advance().unwrap();
        assert!(result.is_identifier());
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 10, 9));
        assert_eq!(ctx.get_str(result.value()), "externald");
    }

    #[test]
    fn r#export() {
        let text = "export";
        let mut ctx = Context::new();
        let mut lexer = Lexer::new(&mut ctx, text);
        let result = lexer.advance().unwrap();
        assert!(result.is_keyword(Export));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 7, 6));
        assert_eq!(ctx.get_str(result.value()), "export");
    }

    #[test]
    fn not_export() {
        let text = "exporting";
        let mut ctx = Context::new();
        let mut lexer = Lexer::new(&mut ctx, text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, identifier());
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 10, 9));
        assert_eq!(ctx.get_str(result.value()), "exporting");
    }

    #[test]
    fn r#from() {
        let text = "from";
        let mut ctx = Context::new();
        let mut lexer = Lexer::new(&mut ctx, text);
        let result = lexer.advance().unwrap();
        assert!(result.is_keyword(From));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 5, 4));
        assert_eq!(ctx.get_str(result.value()), "from");
    }

    #[test]
    fn not_from() {
        let text = "fromage";
        let mut ctx = Context::new();
        let mut lexer = Lexer::new(&mut ctx, text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, identifier());
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 8, 7));
        assert_eq!(ctx.get_str(result.value()), "fromage");
    }

    #[test]
    fn r#for() {
        let text = "for";
        let mut ctx = Context::new();
        let mut lexer = Lexer::new(&mut ctx, text);
        let result = lexer.advance().unwrap();
        assert!(result.is_keyword(For));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 4, 3));
        assert_eq!(ctx.get_str(result.value()), "for");
    }

    #[test]
    fn not_for() {
        let text = "formal";
        let mut ctx = Context::new();
        let mut lexer = Lexer::new(&mut ctx, text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, identifier());
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 7, 6));
        assert_eq!(ctx.get_str(result.value()), "formal");
    }

    #[test]
    fn r#function() {
        let text = "function";
        let mut ctx = Context::new();
        let mut lexer = Lexer::new(&mut ctx, text);
        let result = lexer.advance().unwrap();
        assert!(result.is_keyword(Function));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 9, 8));
        assert_eq!(ctx.get_str(result.value()), "function");
    }

    #[test]
    fn not_function() {
        let text = "functionio";
        let mut ctx = Context::new();
        let mut lexer = Lexer::new(&mut ctx, text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, identifier());
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 11, 10));
        assert_eq!(ctx.get_str(result.value()), "functionio");
    }

    #[test]
    fn r#if() {
        let text = "if";
        let mut ctx = Context::new();
        let mut lexer = Lexer::new(&mut ctx, text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, keyword(If));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 3, 2));
        assert_eq!(ctx.get_str(result.value()), "if");
    }

    #[test]
    fn not_if() {
        let text = "iffy";
        let mut ctx = Context::new();
        let mut lexer = Lexer::new(&mut ctx, text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, identifier());
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 5, 4));
        assert_eq!(ctx.get_str(result.value()), "iffy");
    }

    #[test]
    fn r#import() {
        let text = "import";
        let mut ctx = Context::new();
        let mut lexer = Lexer::new(&mut ctx, text);
        let result = lexer.advance().unwrap();
        assert!(result.is_keyword(Import));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 7, 6));
        assert_eq!(ctx.get_str(result.value()), "import");
    }

    #[test]
    fn not_import() {
        let text = "importance";
        let mut ctx = Context::new();
        let mut lexer = Lexer::new(&mut ctx, text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, identifier());
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 11, 10));
        assert_eq!(ctx.get_str(result.value()), "importance");
    }

    #[test]
    fn r#in() {
        let text = "in";
        let mut ctx = Context::new();
        let mut lexer = Lexer::new(&mut ctx, text);
        let result = lexer.advance().unwrap();
        assert!(result.is_keyword(In));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 3, 2));
        assert_eq!(ctx.get_str(result.value()), "in");
    }

    #[test]
    fn not_in() {
        let text = "inner";
        let mut ctx = Context::new();
        let mut lexer = Lexer::new(&mut ctx, text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, identifier());
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 6, 5));
        assert_eq!(ctx.get_str(result.value()), "inner");
    }

    #[test]
    fn r#let() {
        let text = "let";
        let mut ctx = Context::new();
        let mut lexer = Lexer::new(&mut ctx, text);
        let result = lexer.advance().unwrap();
        assert!(result.is_keyword(Let));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 4, 3));
        assert_eq!(ctx.get_str(result.value()), "let");
    }

    #[test]
    fn not_let() {
        let text = "letter";
        let mut ctx = Context::new();
        let mut lexer = Lexer::new(&mut ctx, text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, identifier());
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 7, 6));
        assert_eq!(ctx.get_str(result.value()), "letter");
    }

    #[test]
    fn r#loop() {
        let text = "loop";
        let mut ctx = Context::new();
        let mut lexer = Lexer::new(&mut ctx, text);
        let result = lexer.advance().unwrap();
        assert!(result.is_keyword(Loop));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 5, 4));
        assert_eq!(ctx.get_str(result.value()), "loop");
    }

    #[test]
    fn not_loop() {
        let text = "loophole";
        let mut ctx = Context::new();
        let mut lexer = Lexer::new(&mut ctx, text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, identifier());
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 9, 8));
        assert_eq!(ctx.get_str(result.value()), "loophole");
    }

    #[test]
    fn package() {
        let text = "package";
        let mut ctx = Context::new();
        let mut lexer = Lexer::new(&mut ctx, text);
        let result = lexer.advance().unwrap();
        assert!(result.is_keyword(Package));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 8, 7));
        assert_eq!(ctx.get_str(result.value()), "package");
    }

    #[test]
    fn not_package() {
        let text = "packaged";
        let mut ctx = Context::new();
        let mut lexer = Lexer::new(&mut ctx, text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, identifier());
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 9, 8));
        assert_eq!(ctx.get_str(result.value()), "packaged");
    }

    #[test]
    fn readonly() {
        let text = "readonly";
        let mut ctx = Context::new();
        let mut lexer = Lexer::new(&mut ctx, text);
        let result = lexer.advance().unwrap();
        assert!(result.is_keyword(Readonly));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 9, 8));
        assert_eq!(ctx.get_str(result.value()), "readonly");
    }

    #[test]
    fn not_readonly() {
        let text = "readonlyness";
        let mut ctx = Context::new();
        let mut lexer = Lexer::new(&mut ctx, text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, identifier());
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 13, 12));
        assert_eq!(ctx.get_str(result.value()), "readonlyness");
    }

    #[test]
    fn r#return() {
        let text = "return";
        let mut ctx = Context::new();
        let mut lexer = Lexer::new(&mut ctx, text);
        let result = lexer.advance().unwrap();
        assert!(result.is_keyword(Return));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 7, 6));
        assert_eq!(ctx.get_str(result.value()), "return");
    }

    #[test]
    fn not_return() {
        let text = "returns";
        let mut ctx = Context::new();
        let mut lexer = Lexer::new(&mut ctx, text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, identifier());
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 8, 7));
        assert_eq!(ctx.get_str(result.value()), "returns");
    }

    #[test]
    fn itself() {
        let text = "self";
        let mut ctx = Context::new();
        let mut lexer = Lexer::new(&mut ctx, text);
        let result = lexer.advance().unwrap();
        assert!(result.is_keyword(Itself));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 5, 4));
        assert_eq!(ctx.get_str(result.value()), "self");
    }

    #[test]
    fn not_self() {
        let text = "selfmade";
        let mut ctx = Context::new();
        let mut lexer = Lexer::new(&mut ctx, text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, identifier());
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 9, 8));
        assert_eq!(ctx.get_str(result.value()), "selfmade");
    }

    #[test]
    fn r#trait() {
        let text = "trait";
        let mut ctx = Context::new();
        let mut lexer = Lexer::new(&mut ctx, text);
        let result = lexer.advance().unwrap();
        assert!(result.is_keyword(Trait));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 6, 5));
        assert_eq!(ctx.get_str(result.value()), "trait");
    }

    #[test]
    fn not_trait() {
        let text = "traitor";
        let mut ctx = Context::new();
        let mut lexer = Lexer::new(&mut ctx, text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, identifier());
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 8, 7));
        assert_eq!(ctx.get_str(result.value()), "traitor");
    }

    #[test]
    fn r#type() {
        let text = "type";
        let mut ctx = Context::new();
        let mut lexer = Lexer::new(&mut ctx, text);
        let result = lexer.advance().unwrap();
        assert!(result.is_keyword(Type));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 5, 4));
        assert_eq!(ctx.get_str(result.value()), "type");
    }

    #[test]
    fn not_type() {
        let text = "typeset";
        let mut ctx = Context::new();
        let mut lexer = Lexer::new(&mut ctx, text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, identifier());
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 8, 7));
        assert_eq!(ctx.get_str(result.value()), "typeset");
    }
}
