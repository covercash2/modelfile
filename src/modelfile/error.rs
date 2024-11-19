use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum ModelfileError {
    /// Error in [`super::builder::ModelfileBuilder`].
    #[error("error building Modelfile from parts")]
    Builder(String),

    /// Error parsing [`super::Modelfile`]
    #[error("unable to parse Modelfile")]
    Parse(String),
}
