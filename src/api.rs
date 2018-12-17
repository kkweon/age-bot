use crate::config::Config;
use actix_web::{App, HttpMessage, HttpRequest};
use futures::prelude::Future;

mod handlers;

pub fn get_app(config: Config) -> App<Config> {
    App::with_state(config)
        .resource("/api/slack", |r| r.post().with(handlers::command))
        .resource("/healthcheck", |r| r.f(handlers::healthcheck))
}
