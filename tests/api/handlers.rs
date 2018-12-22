use actix_web::{test};

use age_bot::api::handlers::healthcheck;
use age_bot::config::get_config_from_env;

#[test]
fn test_healthcheck() {
    let req = test::TestRequest::with_state(get_config_from_env().unwrap()).finish();
    let resp = healthcheck(&req);

    assert_eq!(resp, "ok");
}
