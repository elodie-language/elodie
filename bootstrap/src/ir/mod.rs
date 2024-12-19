pub use context::Context;
pub use r#type::{Property, BaseType, TypeId, TypeName, Type, TypeTable};

mod check;
mod context;
mod infer;
mod unify;
mod r#type;
