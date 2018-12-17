use actix_web::server;
use age_bot::config;

fn main() {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let config = config::get_config_from_env();

    match config {
        Ok(c) => {
            let port = c.port;
            server::new(move || age_bot::api::get_app(c.clone()))
                .bind(format!("0.0.0.0:{}", port))
                .expect(format!("Unable to bind port {}", port).as_str())
                .run();
        }

        Err(err) => {
            eprintln!("{}", err);
        }
    }
}
