use lambda_http::{Body, Error, Request, Response};
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::json;

pub fn parse_body<T: DeserializeOwned>(event: &Request) -> Result<T, Error> {
    match event.body() {
        Body::Text(text) => Ok(serde_json::from_str(text)?),
        Body::Binary(input) => {
            let text = std::str::from_utf8(input)?;
            Ok(serde_json::from_str(text)?)
        }
        Body::Empty => Err("No event body was provided".into()),
    }
}

pub fn json_response<T: Serialize>(status: u16, body: &T) -> Result<Response<Body>, Error> {
    Ok(Response::builder()
        .status(status)
        .header("content-type", "application/json")
        .body(serde_json::to_string(body)?.into())
        .map_err(Box::new)?)
}

pub fn json_message(status: u16, message: &str) -> Result<Response<Body>, Error> {
    json_response(status, &json!({ "message": message }))
}

pub fn text_response(status: u16, body: impl Into<String>) -> Result<Response<Body>, Error> {
    Ok(Response::builder()
        .status(status)
        .header("content-type", "application/json")
        .body(body.into().into())
        .map_err(Box::new)?)
}

pub fn path_param(event: &Request) -> Option<&str> {
    event
        .uri()
        .path()
        .rsplit('/')
        .next()
        .filter(|s| !s.is_empty())
}
