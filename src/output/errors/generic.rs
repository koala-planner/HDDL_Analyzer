use super::*;

#[derive(Debug)]
pub enum ParsingError{
    Lexiacal(LexicalError),
    Syntactic(SyntacticError),
    Semantic(SemanticErrorType)
}

impl From<LexicalError> for ParsingError {
    fn from(value: LexicalError) -> Self {
        ParsingError::Lexiacal(value)
    }
}

impl From<SyntacticError> for ParsingError {
    fn from(value: SyntacticError) -> Self {
        ParsingError::Syntactic(value)
    }
}

impl From<SemanticErrorType> for ParsingError {
    fn from(value: SemanticErrorType) -> Self {
        ParsingError::Semantic(value)
    }
}

impl std::fmt::Display for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Lexiacal(error) => write!(f, "{}", error),
            Self::Syntactic(error) => write!(f, "{}", error),
            Self::Semantic(error) => write!(f, "{}", error)
        }
    }
}
