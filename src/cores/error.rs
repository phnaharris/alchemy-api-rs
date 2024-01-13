use serde::{Deserialize, Serialize};

/// Alchemy error in response.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AlchemyError {
    /// Nested error.
    Nested(NestedError),
    /// Flatten error.
    Flatten(FlattenError),
}

/// Alchemy error in response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlattenError {
    message: String,
    name: Option<String>,
}

/// Alchemy error in response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedError {
    error: FlattenError,
}

impl std::fmt::Display for AlchemyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let err = match self {
            AlchemyError::Nested(err) => &err.error,
            AlchemyError::Flatten(err) => err,
        };

        match &err.name {
            Some(name) => write!(f, "{}: {}", name, err.message),
            None => write!(f, "{}", err.message),
        }
    }
}

impl std::error::Error for AlchemyError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        self.source()
    }
}
