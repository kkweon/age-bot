use crate::config::Config;
use crate::slack;
use crate::slack::model::{SlackCommand, SlackResponse, SlackResponseType};
use actix_web::{
    error, AsyncResponder, Form, FromRequest, FutureResponse, HttpMessage, HttpRequest, Json,
    Responder, Result as HttpResult,
};
use futures::prelude::Future;

pub fn healthcheck(_req: &HttpRequest<Config>) -> impl Responder {
    "ok"
}

pub fn command(req: HttpRequest<Config>) -> FutureResponse<Json<SlackResponse>> {
    Form::<SlackCommand>::extract(&req)
        .and_then(move |slack_command: Form<SlackCommand>| {
            if slack_command.token == req.state().slack_verification_token {
                let text = slack_command.text.trim();
                match super::service::parse_and_get_age(text) {
                    Some(days) => Ok(Json(SlackResponse {
                        response_type:  SlackResponseType::InChannel,
                        text: format!("You are {:.2} years old. Average life expectancy is 78.7 years. You have lived {:.2}% of your life", (days as f64) / 365f64, (days as f64)/(78.7f64 * 365f64) * 100f64),
                    })),
                    None => Ok(Json(SlackResponse {
                        response_type:  SlackResponseType::Ephemeral,
                        text: format!("Unable to parse: {}", text),
                    })),
                }
            } else {
                Err(error::ErrorUnauthorized("token is not valid"))
            }
        })
        .responder()
}
// pub fn command(req: HttpRequest<Config>) -> FutureResponse<Json<SlackResponse>> {
// println!("command BEGIN");
// req.body()
// .from_err()
// .and_then(move |body_bytes| {
// let body_str = std::str::from_utf8(body_bytes.as_ref()).unwrap();
// let slack_command: SlackCommand =
// serde_urlencoded::from_bytes(body_bytes.as_ref()).unwrap();

// println!("after form");

// let header_map = req.headers();
// let ts = header_map
// .get("X-Slack-Request-Timestamp")
// .ok_or(error::ErrorBadRequest(
// "X-Slack-Request-Timestamp is missing",
// ))
// .and_then(|hv| {
// hv.to_str()
// .map_err(|_| error::ErrorInternalServerError("Unable to parse into str"))
// })
// .and_then(|hv| {
// hv.parse::<i64>()
// .map_err(|_| error::ErrorBadRequest("Unable to parse i64"))
// })?;
// let slack_signature = header_map
// .get("X-Slack-Signature")
// .ok_or(error::ErrorBadRequest("X-Slack-Signature is missing"))
// .and_then(|hv| {
// hv.to_str()
// .map_err(|_| error::ErrorBadRequest("Unable to parse into str"))
// })?;

// if slack::service::verify(
// ts,
// body_str.to_owned(),
// slack_signature.to_owned(),
// req.state(),
// ) {
// Ok(Json(SlackResponse {
// text: slack_command.text.clone().trim().to_string(),
// }))
// } else {
// Err(error::ErrorUnauthorized("Unable to verify token"))
// }
// })
// .responder()
// }
