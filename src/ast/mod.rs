pub mod value;
pub mod nodes;
pub mod environment;

// Re-export pour faciliter l'accès : use crate::ast::{Value, Instruction, ...}
pub use value::{Value, InstanceData};
pub use nodes::{Expression, Instruction, ClassDefinition, Statement};
pub use environment::{Environment, NativeFn};
