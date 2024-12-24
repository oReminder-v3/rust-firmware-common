use std::collections::HashMap;

use reqwest::Url;

use crate::commandline::exit_with_cause;
use crate::constant;
use crate::result::ResultResponse;

const LOGIN_API: &str = "/account/login";

pub fn login(email: &String, password: &String) -> String {
    let client = reqwest::blocking::Client::new();
    let url = Url::parse_with_params(
        &format!("{}{}", constant::SERVER_ADDRESS, LOGIN_API),
        &[("platform", get_platform())],
    )
    .unwrap();
    let mut data = HashMap::new();
    data.insert("email", email);
    data.insert("password", password);
    let res = client.post(url).json(&data).send();
    let auth_key;
    if res
        .as_ref()
        .unwrap()
        .headers()
        .get("Authorization")
        .is_some()
    {
        auth_key = auth_key.unwrap().to_str().unwrap().to_string();
    } else {
        auth_key = "";
    }
    let body = res.unwrap().text();
    if body.is_err() {
        exit_with_cause("Failed to parse server data!");
    }
    let result = ResultResponse::new(body.unwrap());
    if result.is_failed() {
        exit_with_cause(result.get_message());
    }
    return auth_key;
}

#[allow(unreachable_code)]
fn get_platform() -> &'static str {
    #[cfg(target_os = "ios")]
    return "iOS";
    #[cfg(target_os = "macos")]
    return "macOS";
    #[cfg(target_os = "linux")]
    return "linux";
    #[cfg(target_os = "windows")]
    return "windows";
    #[cfg(target_os = "android")]
    return "android";
    #[cfg(target_arch = "wasm32")]
    return "web";
    exit_with_cause("Cannot identify the current platform!");
}
