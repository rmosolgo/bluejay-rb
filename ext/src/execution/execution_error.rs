use crate::execution::FieldError;
use crate::ruby_api::{CoercionError, ErrorLocation, ExecutionError as RubyExecutionError};
use bluejay_parser::error::SpanToLocation;
use bluejay_parser::{Error as ParseError, HasSpan};
use bluejay_validator::Path;

#[derive(Debug)]
pub enum ExecutionError<'a> {
    NoOperationWithName {
        name: &'a str,
    },
    CannotUseAnonymousOperation,
    RequiredVariableMissingValue {
        name: &'a str,
    },
    ApplicationError(String),
    CoercionError(CoercionError),
    ParseError(ParseError),
    FieldError {
        error: FieldError,
        path: Path<'a>,
        query: &'a str,
        fields: Vec<&'a bluejay_parser::ast::executable::Field<'a>>,
    },
}

impl<'a> From<ExecutionError<'a>> for RubyExecutionError {
    fn from(val: ExecutionError<'a>) -> Self {
        match val {
            ExecutionError::NoOperationWithName { name } => Self::new(format!("No operation definition named `{name}`"), None, None),
            ExecutionError::CannotUseAnonymousOperation => Self::new("Operation name is required when document does not contain exactly 1 operation definition", None, None),
            ExecutionError::RequiredVariableMissingValue { name } => Self::new(format!("No value was provided for required variable `${name}`"), None, None),
            ExecutionError::ApplicationError(error) => Self::new(format!("Internal error: {error}"), None, None),
            ExecutionError::CoercionError(error) =>  error.into(),
            ExecutionError::ParseError(error) => Self::new(error.message().to_owned(), None, None),
            ExecutionError::FieldError { error, path, fields, query } => {
                let get_location = |field: &&bluejay_parser::ast::executable::Field<'_>| {
                    let span = field.span();
                    let (line, column) = SpanToLocation::new(query).convert(span).unwrap();
                    ErrorLocation::new(line, column)
                };

                let locations = Some(fields.iter().map(get_location).collect());
                Self::new(
                    error.message().to_string(),
                    Some(path.to_vec()),
                    locations,
                )
            },
        }
    }
}
