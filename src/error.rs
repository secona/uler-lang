use crate::token::Token;
use crate::evaluator::object::Object;

#[derive(thiserror::Error, Debug)]
pub enum ParserError {
    #[error("unexpected token: {0}")]
    UnexpectedToken(Token),

    #[error("unknown prefix operator: {0}")]
    PrefixOperator(Token),

    #[error("error parsing integer: could not parse {0} as integer")]
    ParsingInteger(String),

    #[error("illegal token: {0}")]
    IllegalToken(String),

    #[error(r"unknown escape string: \{0}")]
    EscapeString(String),

    #[error("unclosed string")]
    UnclosedString(),

    #[error("unexpected EOF")]
    UnexpectedEOF(),
}

#[derive(thiserror::Error, Debug)]
pub enum EvaluatorError {
    #[error("unknown operator: {0}{1}")]
    PrefixOperator(Token, Object),

    #[error("unknown operator: {0} {1} {2}")]
    UnknownInfixOperator(Object, Token, Object),

    #[error("unknown variable: {0}")]
    UnknownVariable(String),

    #[error("not a function")]
    NotAFunction(),

    #[error("overwriting builtin: {0}")]
    OverwriteBuiltin(String),

    #[error("variable redeclaration: {0}")]
    VariableRedeclaration(String),

    #[error("illegal returning value: {0}")]
    ReturningValue(Object),

    #[error("unexpected token: {0}")]
    UnexpectedToken(Token),
}
