use crate::frontend::lex::token::Token;

#[derive(Clone, Debug, PartialEq)]
pub struct Modifiers(pub(crate) Vec<Modifier>);

impl Modifiers {
    pub fn is_exported(&self) -> bool {
        self.0.iter().any(|m| matches!(m, Modifier::Export(_)))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Modifier {
    Export(Token),
}
