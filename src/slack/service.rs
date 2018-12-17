use crate::config::Config;
use actix_web::{HttpMessage, HttpRequest};
use bytes::{buf::IntoBuf, Bytes};
use chrono::prelude::Utc;
use futures::{future, Future};
use std::io::Read;

use hmac::{Hmac, Mac};
use sha2::Sha256;

// Create alias for HMAC-SHA256
type HmacSha256 = Hmac<Sha256>;

pub fn verify(ts: i64, body: String, slack_signature: String, config: &Config) -> bool {
    println!("ts => {}", ts);
    if 60 * 5 < (Utc::now().timestamp() - ts).abs() {
        return false;
    }
    let sig_basestr = format!("v0:{}:{}", ts, body);
    println!("sig_basestr =>");
    println!("{}", sig_basestr);

    let mut mac = HmacSha256::new_varkey(config.slack_signing_secret.as_bytes()).unwrap();
    mac.input(sig_basestr.as_bytes());

    println!("slack_signature => {}", slack_signature);
    // let result = mac.result();
    // let code_bytes = result.code();

    match mac.verify(slack_signature.replace("v0=", "").as_ref()) {
        Ok(()) => println!("verified"),
            _ => println!("not verified"),
    };

    false
}
