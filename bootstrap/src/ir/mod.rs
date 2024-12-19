pub use r#type::{TypeVariable, Type, TypeId, TypeName, TypeTable};

mod check;
mod context;
mod infer;
mod unify;
mod r#type;
mod symbol;
