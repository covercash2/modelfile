use std::fmt::Display;

use derive_more::derive::{AsMut, AsRef, Deref, From, IntoIterator};
use serde::{Deserialize, Serialize};

use crate::Message;

use super::{Multiline, Parameter, TensorFile};

/// Represented by a line beginning with a `#` in the [`crate::Modelfile`].
#[derive(AsRef, Debug, Deref, Clone, From, Serialize, Deserialize, derive_more::Display)]
#[from(forward)]
pub struct Comment(String);

/// Represented by the `FROM` field in the [`crate::Modelfile`].
#[derive(
    AsRef, Debug, Deref, Clone, From, Serialize, Deserialize, derive_more::Display, PartialEq,
)]
#[from(forward)]
pub struct BaseModel(String);

/// Represented by the `PARAMETER` fields in the [`crate::Modelfile`].
#[derive(
    AsRef,
    AsMut,
    Debug,
    Deref,
    Default,
    Clone,
    From,
    IntoIterator,
    Serialize,
    Deserialize,
    PartialEq,
)]
pub struct Parameters(Vec<Parameter>);

impl Display for Parameters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for value in self.0.iter() {
            writeln!(f, "PARAMETER {value}")?;
        }
        Ok(())
    }
}

impl FromIterator<Parameter> for Parameters {
    fn from_iter<T: IntoIterator<Item = Parameter>>(iter: T) -> Self {
        let parameters: Vec<Parameter> = iter.into_iter().collect();

        Parameters(parameters)
    }
}

/// Represented by the `TEMPLATE` field in the [`crate::Modelfile`].
///
/// Should contain a valid [go template]
/// that uses the [Ollama template] language.
///
/// [go template]: https://pkg.go.dev/text/template
/// [Ollama template]: https://github.com/ollama/ollama/blob/main/docs/modelfile.md#template
#[derive(
    AsRef, Debug, Deref, Clone, From, Serialize, Deserialize, derive_more::Display, PartialEq,
)]
#[from(forward)]
pub struct Template(Multiline);

/// Represented by the `LICENSE` field in the [`crate::Modelfile`].
#[derive(
    AsRef, Debug, Deref, Clone, From, Serialize, Deserialize, derive_more::Display, PartialEq,
)]
#[from(forward)]
#[as_ref(forward)]
pub struct License(Multiline);

/// Represented by the `SYSTEM` field in the [`crate::Modelfile`].
#[derive(
    AsRef, Debug, Deref, Clone, From, Serialize, Deserialize, derive_more::Display, PartialEq,
)]
#[from(forward)]
pub struct SystemMessage(Multiline);

/// Represented by the `ADAPTER` field in the [`crate::Modelfile`].
#[derive(
    AsRef, Debug, Deref, Clone, From, Serialize, Deserialize, derive_more::Display, PartialEq,
)]
#[from(forward)]
pub struct Adapter(TensorFile);

/// Represented by `MESSAGE` fields in the [`crate::Modelfile`].
#[derive(
    AsRef,
    AsMut,
    Debug,
    Default,
    Deref,
    Clone,
    From,
    IntoIterator,
    Serialize,
    Deserialize,
    derive_more::Display,
    PartialEq,
)]
#[display("{}", _0.iter().map(ToString::to_string).collect::<Vec<String>>().join(", "))]
#[from(forward)]
#[as_ref(forward)]
pub struct Messages(Vec<Message>);
