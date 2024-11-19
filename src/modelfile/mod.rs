//! This module defines the [`Modelfile`] structure
//! that represents a structured format of the [Ollama Modelfile].
//!
//! [Ollama Modelfile]: https://github.com/ollama/ollama/blob/main/docs/modelfile.md

use std::{
    fmt::Display,
    path::{Path, PathBuf},
    str::FromStr,
};

use builder::ModelfileBuilder;
use error::ModelfileError;
use parser::instructions;
use serde::{Deserialize, Serialize};
use strum::{EnumDiscriminants, EnumIter, EnumString, IntoStaticStr, VariantArray};

use crate::message::Message;

pub mod builder;
pub mod error;
mod parser;

#[cfg(test)]
pub mod test_data;

const HEADER_COMMENT: &str = "# This file was generated by the Ollama-CLI client\n";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Modelfile {
    from: String,
    parameters: Vec<Parameter>,
    template: Option<Multiline>,
    system: Option<Multiline>,
    adapter: Option<TensorFile>,
    license: Option<Multiline>,
    messages: Vec<Message>,
}

impl Modelfile {
    pub fn render(&self) -> String {
        let mut renderer = Renderer::default();
        renderer.push_raw(HEADER_COMMENT);
        renderer.push("FROM", self.from.as_str());
        renderer.newline();

        renderer.push_opt(
            "ADAPTER",
            self.adapter.clone().map(|file| file.to_string()).as_ref(),
        );

        renderer.push_opt("SYSTEM", self.system.as_ref());
        renderer.push_opt("TEMPLATE", self.template.as_ref());
        renderer.push_vec("PARAMETER", &self.parameters);
        renderer.push_vec("MESSAGE", &self.messages);
        renderer.push_opt("LICENSE", self.license.as_ref());

        renderer.finalize()
    }

    pub fn build_on(self) -> ModelfileBuilder {
        self.into()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Multiline(String);

impl Multiline {
    fn extend(&self, more: &str) -> Self {
        let mut new = self.clone();
        new.0.push('\n');
        new.0.push_str(more);
        new
    }
}

impl From<String> for Multiline {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl<'a> From<&'a str> for Multiline {
    fn from(value: &'a str) -> Self {
        Self(value.to_string())
    }
}

impl std::fmt::Display for Multiline {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"\"\"{}\"\"\"", self.0)
    }
}

#[derive(Clone, Debug, Default)]
struct Renderer {
    builder: String,
}

impl Renderer {
    fn push_raw(&mut self, s: &str) {
        self.builder.push_str(s);
    }

    fn push(&mut self, name: &'static str, s: &str) {
        self.builder.push_str(name);
        self.builder.push(' ');
        self.builder.push_str(s);
        self.builder.push('\n');
    }

    fn newline(&mut self) {
        self.builder.push('\n');
    }

    fn push_opt<T: ToString>(&mut self, name: &'static str, opt: Option<&T>) {
        if let Some(t) = opt {
            self.builder.push_str(name);
            self.builder.push(' ');
            self.builder.push_str(&t.to_string());
            self.builder.push('\n');
            self.builder.push('\n');
        }
    }

    fn push_vec<T: ToString>(&mut self, name: &'static str, vec: &[T]) {
        if vec.is_empty() {
            return;
        }

        for t in vec {
            self.builder.push_str(name);
            self.builder.push(' ');
            self.builder.push_str(&t.to_string());
            self.builder.push('\n')
        }

        self.builder.push('\n')
    }

    fn finalize(self) -> String {
        self.builder
    }
}

impl FromStr for Modelfile {
    type Err = ModelfileError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let instructions: Vec<Instruction> = instructions(input)
            .map_err(|error| ModelfileError::Parse(error.to_string()))
            .and_then(|(rest, instructions)| {
                if rest.is_empty() {
                    Ok(instructions)
                } else {
                    Err(ModelfileError::Parse(
                        "parser did not consume all input".to_string(),
                    ))
                }
            })?;

        let modelfile = instructions.into_iter().try_fold(
            ModelfileBuilder::default(),
            |builder, instruction| match instruction {
                Instruction::From(model) => builder.from(model),
                Instruction::Parameter(parameter) => Ok(builder.parameter(parameter)),
                Instruction::Template(template) => builder.template(template),
                Instruction::System(system) => builder.system(system),
                Instruction::Adapter(tensor_file) => builder.adapter(tensor_file),
                Instruction::License(license) => Ok(builder.license(license)),
                Instruction::Message(message) => Ok(builder.message(message)),
                Instruction::Skip => Ok(builder),
            },
        )?;

        modelfile.build()
    }
}

/// A single instruction to [Ollama]
/// that tells [Ollama] how to configure a language model.
/// Each instruction is defined in the [Modelfile docs]
///
/// [Ollama]: https://ollama.com/
/// [Modelfile docs]: https://github.com/ollama/ollama/blob/main/docs/modelfile.md
pub enum Instruction {
    /// Some part of the file that is skipped,
    /// like an empty line or comment.
    Skip,
    /// The model to derive from,
    /// either a SHA blob,
    /// GGUF file,
    /// directory pointing to `safetensor` files,
    /// or a `<model>:<version>` identifier for an existing model.
    From(String),
    Parameter(Parameter),
    /// A [golang] [Ollama template].
    ///
    /// [golang]: https://pkg.go.dev/text/template
    /// [Ollama template]: https://github.com/ollama/ollama/blob/main/docs/modelfile.md#template
    Template(String),
    /// The system message for the given model.
    System(String),
    Adapter(TensorFile),
    License(String),
    Message(Message),
}

impl From<Parameter> for Instruction {
    fn from(value: Parameter) -> Self {
        Instruction::Parameter(value)
    }
}

impl From<TensorFile> for Instruction {
    fn from(value: TensorFile) -> Self {
        Instruction::Adapter(value)
    }
}

impl From<Message> for Instruction {
    fn from(value: Message) -> Self {
        Instruction::Message(value)
    }
}

/// A file that represents a Tensor.
/// Either a GGUF or safetensor file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TensorFile {
    Gguf(PathBuf),
    Safetensor(PathBuf),
}

impl AsRef<Path> for TensorFile {
    fn as_ref(&self) -> &Path {
        match self {
            TensorFile::Gguf(path_buf) => path_buf.as_ref(),
            TensorFile::Safetensor(path_buf) => path_buf.as_ref(),
        }
    }
}

impl Display for TensorFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.as_ref().display().to_string())
    }
}

/// A parameter for the model.
/// [docs]
///
/// [docs]: https://github.com/ollama/ollama/blob/main/docs/modelfile.md#parameter
#[derive(Debug, Clone, EnumDiscriminants, strum::Display, Serialize, Deserialize)]
#[strum_discriminants(name(ParameterName))]
#[strum_discriminants(derive(EnumIter, IntoStaticStr, EnumString, VariantArray))]
#[strum_discriminants(strum(serialize_all = "snake_case"))]
pub enum Parameter {
    /// Enable Mirostat sampling for controlling perplexity.
    /// (default: 0, 0 = disabled, 1 = Mirostat, 2 = Mirostat 2.0)
    #[strum(to_string = "mirostat {0}")]
    Mirostat(usize),
    /// Influences how quickly the algorithm responds
    /// to feedback from the generated text.
    /// A lower learning rate will result in slower adjustments,
    /// while a higher learning rate will make the algorithm more responsive.
    /// (Default: 0.1)
    #[strum(to_string = "mirostat_eta {0}")]
    MirostatEta(f32),
    /// Controls the balance between coherence and diversity of the output.
    /// A lower value will result in more focused and coherent text.
    /// (Default: 5.0)
    #[strum(to_string = "mirostat_tau {0}")]
    MirostatTau(f32),
    /// Sets the size of the context window
    /// used to generate the next token.
    /// (Default: 2048)
    #[strum(to_string = "num_ctx {0}")]
    NumCtx(usize),
    /// Sets how far back for the model
    /// to look back to prevent repetition.
    /// (Default: 64, 0 = disabled, -1 = num_ctx)
    #[strum(to_string = "repeat_last_n {0}")]
    RepeatLastN(usize),
    /// Sets how strongly to penalize repetitions.
    /// A higher value (e.g., 1.5) will penalize repetitions more strongly,
    /// while a lower value (e.g., 0.9) will be more lenient.
    /// (Default: 1.1)
    #[strum(to_string = "repeat_penalty {0}")]
    RepeatPenalty(f32),
    /// The temperature of the model.
    /// Increasing the temperature will make the model answer more creatively.
    /// (Default: 0.8)
    #[strum(to_string = "temperature {0}")]
    Temperature(f32),
    /// Sets the random number seed to use for generation.
    /// Setting this to a specific number will make the model generate the same text
    /// for the same prompt.
    /// (Default: 0)
    #[strum(to_string = "seed {0}")]
    Seed(usize),
    /// Sets the stop sequences to use.
    /// When this pattern is encountered the LLM will stop generating text and return.
    /// Multiple stop patterns may be set by specifying multiple separate stop parameters
    /// in a modelfile.
    #[strum(to_string = "stop {0}")]
    Stop(String),
    /// Tail free sampling is used to reduce the impact
    /// of less probable tokens from the output.
    /// A higher value (e.g., 2.0) will reduce the impact more,
    /// while a value of 1.0 disables this setting.
    /// (default: 1)
    #[strum(to_string = "tfs_z {0}")]
    TfsZ(f32),
    /// Maximum number of tokens to predict when generating text.
    /// (Default: 128, -1 = infinite generation, -2 = fill context)
    #[strum(to_string = "num_predict {0}")]
    NumPredict(usize),
    /// Reduces the probability of generating nonsense.
    /// A higher value (e.g. 100) will give more diverse answers,
    /// while a lower value (e.g. 10) will be more conservative.
    /// (Default: 40)
    #[strum(to_string = "top_k {0}")]
    TopK(usize),
    /// Works together with top-k.
    /// A higher value (e.g., 0.95) will lead to more diverse text,
    /// while a lower value (e.g., 0.5) will generate more focused and conservative text.
    /// (Default: 0.9)
    #[strum(to_string = "top_p {0}")]
    TopP(f32),
    /// Alternative to the top_p,
    /// and aims to ensure a balance of quality and variety.
    /// The parameter p represents the minimum probability for a token to be considered,
    /// relative to the probability of the most likely token.
    /// For example, with p=0.05 and the most likely token having a probability of 0.9,
    /// logits with a value less than 0.045 are filtered out.
    /// (Default: 0.0)
    #[strum(to_string = "min_p {0}")]
    MinP(f32),
}

#[cfg(test)]
mod tests {
    use insta::assert_snapshot;
    use test_data::{load_modelfiles, TEST_DATA_DIR};

    use super::*;

    #[test]
    fn modelfiles_are_parsed() {
        let modelfiles: Vec<(PathBuf, String)> = load_modelfiles(TEST_DATA_DIR);

        for (path, case) in modelfiles {
            dbg!(&path);
            let modelfile: Modelfile = case
                .parse::<Modelfile>()
                .expect("should be able to parse Modelfile");

            dbg!(modelfile);
        }
    }

    #[test]
    fn modelfiles_can_be_rendered_as_toml() {
        let modelfiles: Vec<(PathBuf, String)> = load_modelfiles(TEST_DATA_DIR);

        for (path, case) in modelfiles {
            dbg!(&path);
            let modelfile: Modelfile = case
                .parse::<Modelfile>()
                .expect("should be able to parse Modelfile");

            let _rendered =
                toml::to_string(&modelfile).expect("should be able to render Modelfiles as TOML");

            dbg!(modelfile);
        }
    }

    #[test]
    fn snapshot_render() {
        let modelfile: Modelfile = load_modelfiles(TEST_DATA_DIR)
            .into_iter()
            .find(|(path, _contents)| {
                path.file_name()
                    .expect("test data should have a valid filename")
                    .to_str()
                    .expect("should be able to convert OsStr to str")
                    == "llama3.2:latest.Modelfile"
            })
            .expect("should have at least one test case")
            .1
            .parse()
            .expect("should be able to parse test data");

        let render = modelfile.render();

        let _modelfile: Modelfile = render
            .parse()
            .expect("should be able to parse rendered Modelfile");

        assert_snapshot!(render);
    }

    #[test]
    fn snapshot_parameters() {
        let param = Parameter::Stop("<eos>".into());

        assert_snapshot!(param, @"stop <eos>");
    }
}
