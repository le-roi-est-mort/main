use std::{error, fmt};
use aws_sdk_dynamodb::model::AttributeValue;

pub type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
pub struct MissingPropError {
    pub id: Option<String>,
    pub prop_name: Option<String>,
}

impl fmt::Display for MissingPropError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Missing prop for Player with id [ {:?} ]: [ attribute name: {:?} ]",
            self.id, self.prop_name
        )
    }
}

impl error::Error for MissingPropError {}

#[derive(Debug)]
pub struct WrongPropTypeError {
    pub id: Option<String>,
    pub prop_name: Option<String>,
    pub prop_value: Option<AttributeValue>,
    pub prop_intended_type: Option<String>,
}

impl fmt::Display for WrongPropTypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Wrong prop type for Player with id [ {:?} ]: [ attribute name: {:?}, attribute value: {:?}, attribute intended type: {:?} ]",
            self.id, self.prop_name, self.prop_value, self.prop_intended_type
        )
    }
}

impl error::Error for WrongPropTypeError {}

#[derive(fmt::Debug)]
pub enum DynamoRequestError {
    NotFound,
    MissingProp(MissingPropError),
    WrongPropType(WrongPropTypeError),
}

impl fmt::Display for DynamoRequestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DynamoRequestError::NotFound => {
                write!(f, "please use a vector with at least one element")
            }
            // The wrapped error contains additional information and is available
            // via the source() method.
            DynamoRequestError::MissingProp(e) => e.fmt(f),
            DynamoRequestError::WrongPropType(e) => e.fmt(f),
        }
    }
}

impl error::Error for DynamoRequestError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            DynamoRequestError::NotFound => None,
            DynamoRequestError::MissingProp(ref e) => Some(e),
            DynamoRequestError::WrongPropType(ref e) => Some(e),
        }
    }
}
