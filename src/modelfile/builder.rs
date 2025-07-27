//! Build a [Modelfile] from parts.
//!
//! Use [`Modelfile::build_on`] to create a Builder
//! from an existing parsed [Modelfile]

use crate::message::Message;

use super::{
    error::ModelfileError,
    instruction::{Adapter, BaseModel, License, Messages, Parameters, SystemMessage, Template},
    Instruction, Modelfile, Parameter,
};

/// Used to build a [`Modelfile`].
///
/// This structure was originally a helper
/// to create a [`Modelfile`] from parsed [`super::Instruction`]s,
/// but it can also be used to build upon an existing [Modelfile].
#[derive(Clone, Debug, Default)]
pub struct ModelfileBuilder {
    pub from: Option<BaseModel>,
    pub parameters: Parameters,
    pub template: Option<Template>,
    pub system: Option<SystemMessage>,
    pub adapter: Option<Adapter>,
    pub license: Option<License>,
    pub messages: Messages,
}

impl ModelfileBuilder {
    pub fn build(self) -> Result<Modelfile, ModelfileError> {
        let ModelfileBuilder {
            from,
            parameters,
            template,
            system,
            adapter,
            license,
            messages,
        } = self;

        let parameters = Parameters::from_iter(parameters);

        if let Some(from) = from {
            Ok(Modelfile {
                from,
                parameters,
                template,
                system,
                adapter,
                license,
                messages,
            })
        } else {
            Err(ModelfileError::Builder(
                "Modelfile requires a FROM instruction".into(),
            ))
        }
    }

    pub fn instruction(self, instruction: Instruction) -> Result<Self, ModelfileError> {
        match instruction {
            Instruction::From(model) => self.from(model),
            Instruction::Parameter(parameter) => Ok(self.parameter(parameter)),
            Instruction::Template(template) => self.template(template),
            Instruction::System(system) => self.system(system),
            Instruction::Adapter(tensor_file) => self.adapter(tensor_file),
            Instruction::License(license) => Ok(self.license(license)),
            Instruction::Message(message) => Ok(self.message(message)),
            Instruction::Skip => Ok(self),
        }
    }

    pub fn from(mut self, input: impl ToString) -> Result<Self, ModelfileError> {
        if self.from.is_some() {
            Err(ModelfileError::Builder(format!(
                "Modelfile can only have one FROM instruction: {}",
                input.to_string()
            )))
        } else {
            self.from = Some(input.to_string().into());
            Ok(self)
        }
    }

    pub fn parameter(mut self, parameter: Parameter) -> Self {
        self.parameters.as_mut().push(parameter);
        self
    }

    pub fn template(mut self, template: Template) -> Result<Self, ModelfileError> {
        if self.template.is_some() {
            Err(ModelfileError::Builder(format!(
                "Modelfile can only have one TEMPLATE instruction: {template}",
            )))
        } else {
            self.template = Some(template);
            Ok(self)
        }
    }

    pub fn system(mut self, system: impl ToString) -> Result<Self, ModelfileError> {
        if self.system.is_some() {
            Err(ModelfileError::Builder(format!(
                "Modelfile can only have one SYSTEM instruction: {}",
                system.to_string(),
            )))
        } else {
            self.system = Some(system.to_string().into());
            Ok(self)
        }
    }

    pub fn adapter(mut self, adapter: Adapter) -> Result<Self, ModelfileError> {
        if self.adapter.is_some() {
            Err(ModelfileError::Builder(format!(
                "Modelfile can only have one ADAPTER instruction: {adapter:?}",
            )))
        } else {
            self.adapter = Some(adapter);
            Ok(self)
        }
    }

    pub fn license(mut self, license: impl AsRef<str>) -> Self {
        if let Some(existing) = &self.license {
            self.license = Some(existing.extend(license.as_ref()).into());
        } else {
            self.license = Some(license.as_ref().into());
        }

        self
    }

    pub fn message(mut self, message: Message) -> Self {
        self.messages.as_mut().push(message);
        self
    }
}

impl TryFrom<Vec<Instruction>> for ModelfileBuilder {
    type Error = ModelfileError;

    fn try_from(instructions: Vec<Instruction>) -> Result<Self, Self::Error> {
        instructions
            .into_iter()
            .try_fold(ModelfileBuilder::default(), ModelfileBuilder::instruction)
    }
}

impl From<Modelfile> for ModelfileBuilder {
    fn from(value: Modelfile) -> Self {
        let Modelfile {
            from,
            parameters,
            template,
            system,
            adapter,
            license,
            messages,
        } = value;

        ModelfileBuilder {
            from: Some(from),
            parameters,
            template,
            system,
            adapter,
            license,
            messages,
        }
    }
}
