use crate::slack::model::{SlackCommand, SlackResponse};
use actix_web::{Form, HttpRequest, Json, Responder, Result as HttpResult};

pub fn healthcheck(_req: &HttpRequest) -> impl Responder {
    "ok"
}

pub fn command(req: Form<SlackCommand>) -> HttpResult<Json<SlackResponse>> {
    Ok(Json(SlackResponse { text: req.text.clone().trim().to_string() }))
}
