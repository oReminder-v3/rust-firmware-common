use crate::commandline::exit_with_cause;

#[derive(Debug)]
pub struct ResultResponse {
    response: String,
    ok_or_fail: bool,
    message: String,
    data: String,
}

#[allow(unused)]
impl ResultResponse {
    pub fn new(response: String) -> ResultResponse {
        let mut result = ResultResponse {
            response,
            ok_or_fail: false,
            message: String::new(),
            data: String::new(),
        };
        result.parse_response();
        result
    }

    pub fn from_result(result: Result<reqwest::blocking::Response, reqwest::Error>) -> Self {
        if result.as_ref().is_err() {
            exit_with_cause("cannot unpack response!");
        }
        let text = result.unwrap().text();
        if text.is_err() {
            exit_with_cause("cannot unpack response!");
        }
        return Self::new(text.unwrap().clone());
    }

    pub fn is_ok(&self) -> bool {
        self.ok_or_fail
    }

    pub fn is_failed(&self) -> bool {
        !self.ok_or_fail
    }

    pub fn if_failed(&self, consume: impl FnOnce(&str), or_else: Option<impl FnOnce()>) {
        if self.is_failed() {
            consume(&self.message);
        } else if let Some(or_else) = or_else {
            or_else();
        }
    }

    /// # Panics
    /// if result is ok, this will panic
    pub fn get_message(&self) -> &str {
        &self.message
    }

    pub fn get_data(&self) -> &str {
        &self.data
    }

    fn parse_response(&mut self) {
        if self.response.is_empty() {
            exit_with_cause("response is empty!");
        }

        let json_result = serde_json::from_str(&self.response);
        if json_result.is_err() {
            exit_with_cause("invalid json error");
        }
        let json: serde_json::Value = json_result.unwrap();
        self.ok_or_fail = json["status"].as_bool().unwrap();
        self.message = json["message"].as_str().unwrap_or("").to_owned();
        let data_value = &json["data"];
        if let Some(data_obj) = data_value.as_object() {
            self.data = serde_json::to_string(data_obj).unwrap_or_default();
        }
    }
}
