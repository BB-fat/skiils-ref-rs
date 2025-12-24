//! Skill-related error types.

/// Base error type for all skill-related errors.
#[derive(Debug, thiserror::Error)]
pub enum SkillError {
    /// Raised when SKILL.md parsing fails.
    #[error("{0}")]
    Parse(String),

    /// Raised when skill properties are invalid.
    #[error("{message}")]
    Validation {
        message: String,
        errors: Vec<String>,
    },

    /// Raised when an I/O operation fails.
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

impl SkillError {
    /// Create a new parse error.
    pub fn parse(message: impl Into<String>) -> Self {
        Self::Parse(message.into())
    }

    /// Create a new validation error with a single message.
    pub fn validation(message: impl Into<String>) -> Self {
        let msg = message.into();
        Self::Validation {
            message: msg.clone(),
            errors: vec![msg],
        }
    }

    /// Create a new validation error with multiple error messages.
    pub fn validation_multiple(message: impl Into<String>, errors: Vec<String>) -> Self {
        Self::Validation {
            message: message.into(),
            errors,
        }
    }

    /// Get the list of validation errors, if this is a validation error.
    pub fn errors(&self) -> Option<&[String]> {
        match self {
            Self::Validation { errors, .. } => Some(errors),
            _ => None,
        }
    }
}

/// Result type alias for skill operations.
pub type Result<T> = std::result::Result<T, SkillError>;
