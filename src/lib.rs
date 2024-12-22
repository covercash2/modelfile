#![doc = include_str!("../README.md")]
pub mod message;
pub mod modelfile;

pub use message::{Message, MessageRole};
pub use modelfile::{builder::ModelfileBuilder, Modelfile};
