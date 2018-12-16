use actix_web::server;
use age_bot::config;

fn main() {
    let config = config::get_config_from_env();


    match config {
        Ok(c) => {
            server::new(|| age_bot::api::get_app())
                .bind(format!("0.0.0.0:{}", c.port))
                .expect(format!("Unable to bind port {}", c.port).as_str())
                .run();
        },

        Err(err) => {
            eprintln!("{}", err);
        }

    }
}
