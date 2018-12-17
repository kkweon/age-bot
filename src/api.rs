use crate::config::Config;
use actix_web::{middleware::Logger, App};

pub mod handlers;
pub mod service;

pub fn get_app(config: Config) -> App<Config> {
    App::with_state(config)
        .middleware(Logger::default())
        .resource("/api/slack", |r| r.post().with(handlers::command))
        .resource("/healthcheck", |r| r.f(handlers::healthcheck))
}
