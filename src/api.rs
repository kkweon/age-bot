use actix_web::{App, http};

mod handlers;

pub fn get_app() -> App<()> {
    App::new()
        .resource("/api/slack", |r| r.method(http::Method::POST).with(handlers::command))
        .resource("/healthcheck", |r| r.f(handlers::healthcheck))
}
