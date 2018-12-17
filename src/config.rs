#[derive(Clone)]
pub struct Config {
    pub port: u16,
    pub slack_signing_secret: String,
    pub slack_verification_token: String,
}

#[derive(Fail, Debug)]
pub enum ConfigErr {
    #[fail(display = "$PORT is not available")]
    PortErr,
    #[fail(display = "$PORT is not valid (not a positive integer)")]
    InvalidPortParseErr,

    #[fail(display = "$SLACK_SIGNING_SECRET is undefined")]
    SlackSecretErr,

    #[fail(display = "$SLACK_VERIFICATION_TOKEN is undefined")]
    SlackVerificationToeknErr,
}

pub fn get_config_from_env() -> Result<Config, ConfigErr> {
    let port = std::env::var("PORT").map_err(|_| ConfigErr::PortErr)?;
    let port_int = port
        .parse::<u16>()
        .map_err(|_| ConfigErr::InvalidPortParseErr)?;

    let slack_signing_secret =
        std::env::var("SLACK_SIGNING_SECRET").map_err(|_| ConfigErr::SlackSecretErr)?;

    let slack_verification_token = std::env::var("SLACK_VERIFICATION_TOKEN")
        .map_err(|_| ConfigErr::SlackVerificationToeknErr)?;

    Ok(Config {
        port: port_int,
        slack_signing_secret,
        slack_verification_token,
    })
}
