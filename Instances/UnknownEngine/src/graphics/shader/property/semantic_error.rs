use std::{
    error::Error, fmt, str::FromStr
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShaderSemanticParseError {
    InvalidFormat,
    UnknownNamespace(String),
    UnknownEngineSemantic(String),
    EmptyGameSemantic,
    InvalidGameSemantic(String)
}

impl fmt::Display for ShaderSemanticParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidFormat => {
                write!(f, "Invalid shader semantic format")
            }
            
            Self::UnknownNamespace(namespace) => {
                write!(f, "Unknown shader semantic namespace: {namespace}")
            }

            Self::UnknownEngineSemantic(name) => {
                write!(f, "Unknown engine semantic: {name}")
            }

            Self::EmptyGameSemantic => {
                write!(f, "Game semantic cannot be empty")
            }

            Self::InvalidGameSemantic(name) => {
                write!(f, "Invalid game semantic: {name}")
            }
        }
    }
}

impl Error for ShaderSemanticParseError {}