use std::fmt::{Display, Formatter, Error as IOError};
use std::borrow::{Cow, Borrow};
use std::error::Error;
use std::any::TypeId;

//todo make it better
#[derive(Debug)]
pub struct CompileError {
    language: Cow<'static, str>,
    target: Cow<'static, str>,
    cause: Cow<'static, str>,
}

impl CompileError {
    pub fn new(language: Cow<'static, str>, target: Cow<'static, str>, cause: Cow<'static, str>) -> Self {
        CompileError {
            language,
            target,
            cause,
        }
    }
}

impl Default for CompileError {
    fn default() -> Self {
        CompileError {
            language: Default::default(),
            target: Default::default(),
            cause: "Cannot compile".into()
        }
    }
}

impl Display for CompileError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), IOError> {
        write!(
            f,
            "Error during compile with: {}, for source {} with cause {}",
            self.language.to_owned(),
            self.target.to_owned(),
            self.cause.to_owned()
        )
    }
}

impl Error for CompileError {
    fn description(&self) -> &str {
        self.language.borrow()
    }

    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self)
    }
}