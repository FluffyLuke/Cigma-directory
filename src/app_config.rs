use std::env;
use dotenv::dotenv;

use strum_macros::EnumString;

#[derive(Debug, Clone)]
pub struct AppConfig {
    db: AvailableDatabases,
    host: Option<String>,
    port: Option<u16>,
    user: Option<String>,
    password: Option<String>,
    db_name: Option<String>,
    pool_size: Option<u16>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, EnumString, Clone, Copy)]
pub enum AvailableDatabases {
    //#[serde(rename(serialize = "Mysql"))]
    #[strum(serialize = "mysql")]
    Mysql
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
                return Err(GetConfigError::CannotParsePort);
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
                return Err(GetConfigError::CannotParsePort);
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
#[derive(Debug, Clone, Copy)]
pub enum GetConfigError {
    CannotParsePort,
    BadDatabase,
    NoFieldFound(&'static str)
}

