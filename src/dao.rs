use chrono::{DateTime, Utc};
use failure::Error;
use models;
use postgres::{Connection, SslMode};
use reqwest;
use reqwest::header;
use serde;
use serde_json;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Deserialize)]
pub struct ConnectionConfig {
    host: String,
    username: String,
    password: String,
    port: u16,
    db: String,
}

pub struct DAO {
    conn_config: ConnectionConfig,
    connection: Connection,
}

impl DAO {
    pub fn new(file_name: &str) -> Result<Self, Error> {
        let file = File::open(file_name)?;
        let conn_config: ConnectionConfig = serde_json::from_reader(file)?;
        let conn_string = format!(
            "postgresql://{username}:{dbpass}@{addr}:{port}/{db}",
            username = conn_config.username,
            dbpass = conn_config.password,
            addr = conn_config.host,
            port = conn_config.port,
            db = conn_config.db
        );
        // TODO: test with configured tls config
        let connection = Connection::connect(&(*conn_string), SslMode::None)
            .expect("Unable to connect to database");
        Ok(DAO {
            conn_config,
            connection,
        })
    }

    // pub fn
    // should I just put all crud stuff in here instead of attached to the models
}
