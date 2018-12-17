use crate::config::Config;
use crate::slack::model::{SlackCommand, SlackResponse, SlackResponseType};
use actix_web::{
    error, AsyncResponder, Form, FromRequest, FutureResponse, HttpRequest, Json, Responder,
};
use futures::prelude::Future;

pub fn healthcheck(_req: &HttpRequest<Config>) -> &'static str {
    "ok"
}

pub fn command(req: HttpRequest<Config>) -> FutureResponse<Json<SlackResponse>> {
    Form::<SlackCommand>::extract(&req)
        .and_then(move |slack_command: Form<SlackCommand>| {
            if slack_command.token == req.state().slack_verification_token {
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
            } else {
                Err(error::ErrorUnauthorized("token is not valid"))
            }
        })
        .responder()
}
