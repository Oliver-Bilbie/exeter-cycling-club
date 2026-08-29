use aws_sdk_dynamodb::types::AttributeValue;
use std::collections::HashMap;

pub trait FromAttr: Sized {
    fn from_attr(value: &AttributeValue) -> Result<Self, &AttributeValue>;
}

impl FromAttr for String {
    fn from_attr(value: &AttributeValue) -> Result<Self, &AttributeValue> {
        value.as_s().map(ToOwned::to_owned)
    }
}

impl FromAttr for bool {
    fn from_attr(value: &AttributeValue) -> Result<Self, &AttributeValue> {
        value.as_bool().copied()
    }
}

pub fn attr<T: FromAttr>(item: &HashMap<String, AttributeValue>, key: &str) -> Option<T> {
    match item.get(key) {
        Some(value) => match T::from_attr(value) {
            Ok(v) => Some(v),
            Err(err) => {
                tracing::error!("{:?}", err);
                None
            }
        },
        None => {
            tracing::error!("No {key} found");
            None
        }
    }
}
