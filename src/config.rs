use std::fmt::{Display, Formatter};

#[derive(Clone)]
pub struct Config {
    pub port: u16,
    pub slack_signing_secret: String,
}

pub enum ConfigErr {
    PortErr,
    InvalidPortParseErr,
    SlackSecretErr,
}

impl Display for ConfigErr {
    fn fmt(self: &Self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            ConfigErr::PortErr => writeln!(f, "Env variable $PORT is not set/invalid"),
            ConfigErr::InvalidPortParseErr => writeln!(f, "$PORT has to be positive integer"),
            ConfigErr::SlackSecretErr => writeln!(
                f,
                "environment variable $SLACK_SIGNING_SECRET is not defined"
            ),
        };
        Ok(())
    }
}

pub fn get_config_from_env() -> Result<Config, ConfigErr> {
    let port = std::env::var("PORT").map_err(|_| ConfigErr::PortErr)?;
    let port_int = port
        .parse::<u16>()
        .map_err(|_| ConfigErr::InvalidPortParseErr)?;

    let slack_signing_secret =
        std::env::var("SLACK_SIGNING_SECRET").map_err(|_| ConfigErr::SlackSecretErr)?;
    Ok(Config {
        port: port_int,
        slack_signing_secret,
    })
}
