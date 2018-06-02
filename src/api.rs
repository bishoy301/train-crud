use chrono::{DateTime, Utc};
use failure::Error;
use models;
use reqwest;
use reqwest::header;
use serde;
use serde_json;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Deserialize)]
pub struct Credentials {
    primary_key: String,
    secondary_key: String,
}

pub struct API {
    creds: Credentials,
}

impl API {
    pub fn new(file_name: &str) -> Result<Self, Error> {
        let file = File::open(file_name)?;
        let creds: Credentials = serde_json::from_reader(file)?;
        Ok(API { creds })
    }
    pub fn get_json(&self, url: &str) -> Result<String, Error> {
        let mut headers = header::Headers::new();
        headers.set_raw("api_key", self.creds.primary_key.clone());

        let client = reqwest::Client::builder().default_headers(headers).build()?;

        Ok(client.get(url).send()?.text()?)
    }
}
