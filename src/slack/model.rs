#[derive(Deserialize)]
pub struct SlackCommand {
    pub token: String,           // gIkuvaNzQIHg97ATvDxqgjtO

    pub team_id: String,         // T0001
    pub team_domain: String,     // example


    pub channel_id: String,      // C2147483705
    pub channel_name: String,    // test

    pub user_id: String,         // U2147483697
    pub user_name: String,       // Steve

    pub command: String,         // /weather

    pub text: String,            // 94070

    pub response_url: String,    // https://hooks.slack.com/commands/1234/5678
    pub trigger_id: String,      // 13345224609.738474920.8088930838d88f008e0
}

#[derive(Serialize)]
pub struct SlackResponse {
    pub text: String,
    pub response_type: SlackResponseType,
}

#[derive(Serialize)]
pub enum SlackResponseType {
    #[serde(rename = "in_channel")]
    InChannel,

    #[serde(rename = "ephemeral")]
    Ephemeral,
}
