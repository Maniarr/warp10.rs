use isahc::http::{status::StatusCode, HeaderMap, HeaderValue};

#[derive(Debug)]
pub struct Warp10Response {
    status: StatusCode,
    payload: String,
}

impl Warp10Response {
    pub fn new(status: StatusCode, payload: String) -> Self {
        Self { status, payload }
    }

    pub fn status(&self) -> StatusCode {
        self.status
    }

    pub fn payload(&self) -> &str {
        &self.payload
    }
}

pub fn extract_header_err(headers: &HeaderMap<HeaderValue>) -> Option<String> {
    // Extract the error header from the Warp 10 response, the header is defined here
    // https://github.com/senx/warp10-platform/blob/master/warp10/src/main/java/io/warp10/continuum/store/Constants.java#L155
    headers
        .get("X-Warp10-Error-Message")
        .map(|hv| hv.as_bytes().to_vec())
        .and_then(|buf| String::from_utf8(buf).ok())
}
