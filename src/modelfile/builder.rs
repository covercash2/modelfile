use crate::message::Message;

use super::{error::ModelfileError, Modelfile, Multiline, Parameter, TensorFile};

#[derive(Clone, Debug, Default)]
pub struct ModelfileBuilder {
    pub from: Option<String>,
    pub parameters: Vec<Parameter>,
    pub template: Option<Multiline>,
    pub system: Option<Multiline>,
    pub adapter: Option<TensorFile>,
    pub license: Option<Multiline>,
    pub messages: Vec<Message>,
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

    pub fn from(mut self, input: impl ToString) -> Result<Self, ModelfileError> {
        if self.from.is_some() {
            Err(ModelfileError::Builder(format!(
                "Modelfile can only have one FROM instruction: {}",
                input.to_string()
            )))
        } else {
            self.from = Some(input.to_string());
            Ok(self)
        }
    }

    pub fn parameter(mut self, parameter: Parameter) -> Self {
        self.parameters.push(parameter);
        self
    }

    pub fn template(mut self, template: impl ToString) -> Result<Self, ModelfileError> {
        if self.template.is_some() {
            Err(ModelfileError::Builder(format!(
                "Modelfile can only have one TEMPLATE instruction: {}",
                template.to_string()
            )))
        } else {
            self.template = Some(template.to_string().into());
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

    pub fn adapter(mut self, adapter: TensorFile) -> Result<Self, ModelfileError> {
        if self.adapter.is_some() {
            Err(ModelfileError::Builder(format!(
                "Modelfile can only have one ADAPTER instruction: {:?}",
                adapter,
            )))
        } else {
            self.adapter = Some(adapter);
            Ok(self)
        }
    }

    pub fn license(mut self, license: impl AsRef<str>) -> Self {
        if let Some(existing) = &self.license {
            self.license = Some(existing.extend(license.as_ref()));
        } else {
            self.license = Some(license.as_ref().into());
        }

        self
    }

    pub fn message(mut self, message: Message) -> Self {
        self.messages.push(message);
        self
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
