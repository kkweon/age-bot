use crate::config::Config;
use crate::slack;
use crate::slack::model::{SlackCommand, SlackResponse};
use actix_web::{
    error, AsyncResponder, FutureResponse, HttpMessage, HttpRequest, Json, Responder,
    Result as HttpResult,
};
use futures::prelude::Future;

pub fn healthcheck(_req: &HttpRequest<Config>) -> impl Responder {
    "ok"
}

pub fn command(req: HttpRequest<Config>) -> FutureResponse<Json<SlackResponse>> {
    println!("command BEGIN");
    req.body()
        .from_err()
        .and_then(move |body_bytes| {
            let body_str = std::str::from_utf8(body_bytes.as_ref()).unwrap();
            let slack_command: SlackCommand =
                serde_urlencoded::from_bytes(body_bytes.as_ref()).unwrap();

            println!("after form");

            let header_map = req.headers();
            let ts = header_map
                .get("X-Slack-Request-Timestamp")
                .ok_or(error::ErrorBadRequest(
                    "X-Slack-Request-Timestamp is missing",
                ))
                .and_then(|hv| {
                    hv.to_str()
                        .map_err(|_| error::ErrorInternalServerError("Unable to parse into str"))
                })
                .and_then(|hv| {
                    hv.parse::<i64>()
                        .map_err(|_| error::ErrorBadRequest("Unable to parse i64"))
                })?;
            let slack_signature = header_map
                .get("X-Slack-Signature")
                .ok_or(error::ErrorBadRequest("X-Slack-Signature is missing"))
                .and_then(|hv| {
                    hv.to_str()
                        .map_err(|_| error::ErrorBadRequest("Unable to parse into str"))
                })?;

            if slack::service::verify(
                ts,
                body_str.to_owned(),
                slack_signature.to_owned(),
                req.state(),
            ) {
                Ok(Json(SlackResponse {
                    text: slack_command.text.clone().trim().to_string(),
                }))
            } else {
                Err(error::ErrorUnauthorized("Unable to verify token"))
            }
        })
        .responder()
}
