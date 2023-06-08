use crate::{
    error::Error,
};

use eve_character_validator::{validate, JWKS, Response};
use hyper::{
    client::connect::HttpConnector,
    body::Body,
};
use hyper_tls::HttpsConnector;

pub struct Validator {
    hyper_client: hyper::Client<HttpsConnector<HttpConnector>, Body>,
    client_id: String,
    jwks: JWKS,
}

impl Validator {
    pub async fn new(client_id: String) -> Result<Self, Error> {
        let https = HttpsConnector::new();
        let hyper_client = hyper::Client::builder()
            .build::<_, hyper::Body>(https);
        let jwks = JWKS::new(&hyper_client).await?;
        Ok(Self {
            hyper_client: hyper_client,
            client_id: client_id,
            jwks: jwks,
        })
    }

    pub async fn validate<S: AsRef<str>>(
        &self,
        token: &str,
        characters: impl Iterator<Item = S>,
    ) -> Result<Response, Error> {
        let rep = validate(
            &self.hyper_client,
            &self.jwks,
            token,
            &self.client_id,
            characters,
        )
            .await?;
        Ok(rep)
    }
}
