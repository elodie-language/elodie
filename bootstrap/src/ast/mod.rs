use crate::{compile, lex, parse};
pub use crate::ast::ast::*;
use crate::lex::lex;
use crate::parse::parse;

mod ast;
pub mod r#type;
pub(crate) mod modifier;

