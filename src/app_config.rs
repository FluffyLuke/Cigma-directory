use std::env;
use dotenv::dotenv;

use strum::IntoEnumIterator;
use strum_macros::EnumString;

use crate::databases::AvailableDatabases;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub db: AvailableDatabases,
    pub host: Option<String>,
    pub port: Option<u16>,
    pub user: Option<String>,
    pub password: Option<String>,
    pub db_name: Option<String>,
    pub pool_size: Option<u16>,
}

pub fn get_config_env() -> Result<AppConfig, GetConfigError> {
    dotenv().ok();

    let db = env::var("DB");

    if db.is_err() { return Err(GetConfigError::NoFieldFound("DB")) }
    let db: AvailableDatabases = db.unwrap()
        .parse()
        .map_err(|_err| GetConfigError::BadDatabase)?;

    let host = match env::var("HOST") {
        Ok(host) => Some(host),
        Err(_err) => None,
    };
    let port = match env::var("PORT") {
        Ok(v) => {
            let parsed_v = v.parse::<u16>();
            if parsed_v.is_err() {
                return Err(GetConfigError::CannotParse("port", v));
            }
            Some(parsed_v.unwrap())
        },
        Err(_err) => None,
    };
    let user = match env::var("USER") {
        Ok(v) => Some(v),
        Err(_err) => None,
    };
    let password = match env::var("PASSWORD") {
        Ok(v) => Some(v),
        Err(_err) => None,
    };
    let db_name = match env::var("DBNAME") {
        Ok(v) => Some(v),
        Err(_err) => None,
    };
    let pool_size = match env::var("POOL.MAX_SIZE") {
        Ok(v) => {
            let parsed_v = v.parse::<u16>();
            if parsed_v.is_err() {
                return Err(GetConfigError::CannotParse("pool", v));
            }
            Some(parsed_v.unwrap())
        },
        Err(_err) => None,
    };

    Ok(AppConfig {
        db,
        host,
        port,
        user,
        password,
        db_name,
        pool_size
    })
}

// TODO Make a better 
#[derive(Debug, Clone)]
pub enum GetConfigError {
    CannotParse(&'static str, String),
    BadDatabase,
    NoFieldFound(&'static str)
}

impl std::fmt::Display for GetConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GetConfigError::CannotParse(field, value) => write!(f, "Cannot parse field {}={} to int", field, value),
            GetConfigError::BadDatabase => {
                let mut message = format!("Provided database is not available. List of databases available:\n");
                for e in AvailableDatabases::iter() {
                    let e = format!("{}\n", e);
                    message.push_str(&e)
                }

                write!(f, "{}", message)
            },
            GetConfigError::NoFieldFound(v) => write!(f, "Field not found: {}", v),
        }
    }
}