use isahc::{
    http::{status::StatusCode},
    AsyncBody, AsyncReadResponseExt, Body, ReadResponseExt, Request, RequestExt,
};

use crate::client::*;
use crate::data::*;
use crate::error::*;
use crate::response::*;
use crate::token::*;

#[derive(Debug)]
pub struct Reader<'a> {
    client: &'a Client,
}

impl<'a> Reader<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn post(&self, data: String) -> Result<Warp10Response> {
        let request = self.post_request::<AsyncBody>(data)?;
        let mut response = request.send_async().await?;
        let status = response.status();
        let err = extract_header_err(&response.headers());
        let payload = response.text().await?;
        self.handle_response(err, status, payload)
    }

    pub fn post_sync(&self, data: String) -> Result<Warp10Response> {
        let request = self.post_request::<Body>(data)?;
        let mut response = request.send()?;
        let status = response.status();
        let err = extract_header_err(&response.headers());
        let payload = response.text()?;
        self.handle_response(err, status, payload)
    }

    fn post_request<T: From<String>>(&self, data: String) -> Result<Request<T>> {
        let mut request = Request::post(self.client.exec_uri()).body(T::from(data))?;
        Ok(request)
    }

    fn handle_response(
        &self,
        err: Option<String>,
        status: StatusCode,
        payload: String,
    ) -> Result<Warp10Response> {
        let response = Warp10Response::new(status, payload);
        match response.status() {
            StatusCode::OK => Ok(response),
            _ => Err(Error::api_error(response, err)),
        }
    }
}
