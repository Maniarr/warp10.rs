use isahc::http::uri::Uri;

use crate::error::*;
use crate::reader::*;
use crate::token::*;
use crate::writer::*;

#[derive(Debug)]
pub struct Client {
    exec_uri: Uri,
    update_uri: Uri,
}

impl Client {
    pub fn new(uri: &str) -> Result<Client> {
        Ok(Client {
            exec_uri: format!("{}/api/v0/exec", uri).parse()?,
            update_uri: format!("{}/api/v0/update", uri).parse()?,
        })
    }

    pub fn exec_uri(&self) -> &Uri {
        &self.exec_uri
    }

    pub fn update_uri(&self) -> &Uri {
        &self.update_uri
    }

    pub fn host_and_maybe_port(&self) -> String {
        let host = self.update_uri.host().unwrap_or("localhost");

        self.update_uri
            .port()
            .map(|port| format!("{}:{}", host, port))
            .unwrap_or_else(|| host.to_string())
    }

    pub fn get_reader(&self) -> Reader<'_> {
        Reader::new(self)
    }

    pub fn get_writer(&self, token: String) -> Writer<'_> {
        Writer::new(self, Token::new(self, token))
    }
}
