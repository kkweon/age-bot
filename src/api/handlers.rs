use crate::config::Config;
use crate::slack::model::{SlackCommand, SlackResponse, SlackResponseType};
use actix_web::{
    error, AsyncResponder, FutureResponse, HttpMessage, HttpRequest, Json,
};
use bytes::buf::{Buf, IntoBuf};
use bytes::Bytes;
use futures::prelude::Future;

pub fn healthcheck(_req: &HttpRequest<Config>) -> &'static str {
    "ok"
}

pub fn command(req: HttpRequest<Config>) -> FutureResponse<Json<SlackResponse>> {
    req.body()
        .from_err()
        .and_then(move |body: Bytes| {
            let headers = req.headers();

            let timestamp = headers
                .get("X-Slack-Request-Timestamp")
                .ok_or(error::ErrorBadRequest(
                    "X-Slack-Request-Timestamp is missing",
                ))?
                .to_str()
                .map_err(|_| error::ErrorBadRequest("is not str"))?
                .parse::<i64>()
                .map_err(|_| error::ErrorBadRequest("Unabel to parse into i64"))?;
            let expected_hex = headers.get("X-Slack-Signature").unwrap().to_str().unwrap();

            match slack_verify::verify(
                req.state().slack_signing_secret.as_str(),
                std::str::from_utf8(body.clone().into_buf().bytes()).unwrap(),
                timestamp,
                expected_hex,
            ) {
                Ok(true) => {
                    let slack_command: SlackCommand =
                        serde_urlencoded::from_bytes(body.into_buf().bytes()).unwrap();

                    let text = slack_command.text.trim();

                    match super::service::parse_and_get_age(text) {
                        Some(days) => Ok(Json(SlackResponse {
                            response_type: SlackResponseType::InChannel,
                            text: super::service::get_message(days),
                        })),
                        None => Ok(Json(SlackResponse {
                            response_type: SlackResponseType::Ephemeral,
                            text: format!("Unable to parse: {}", text),
                        })),
                    }
                }
                _ => Err(error::ErrorUnauthorized("Not authorized")),
            }
        })
        .responder()
}
