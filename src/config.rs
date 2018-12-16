use std::fmt::{Display, Formatter};

pub struct Config {
    pub port: u16,
}

pub enum ConfigErr {
    PortErr,
    InvalidPortParseErr,
}

impl Display for ConfigErr {
    fn fmt(self: &Self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            ConfigErr::PortErr => writeln!(f, "Env variable $PORT is not set/invalid"),
            ConfigErr::InvalidPortParseErr => writeln!(f, "$PORT has to be positive integer"),
        };
        Ok(())
    }
}

pub fn get_config_from_env() -> Result<Config, ConfigErr> {
    let port = std::env::var("PORT").map_err(|_| ConfigErr::PortErr)?;
    let port_int = port
        .parse::<u16>()
        .map_err(|_| ConfigErr::InvalidPortParseErr)?;

    Ok(Config { port: port_int })
}
