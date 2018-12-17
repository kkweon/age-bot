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
    if 60 * 5 < (Utc::now().timestamp() - ts).abs() {
        return false;
    }
    let sig_basestr = format!("v0:{}:{}", ts, body);

    let mut mac = HmacSha256::new_varkey(config.slack_signing_secret.as_bytes()).unwrap();
    mac.input(sig_basestr.as_bytes());

    match mac.verify(slack_signature.as_bytes()) {
        Ok(()) => true,
        _ => false,
    }
}
