use async_trait::async_trait;
use strum_macros::{EnumIter, EnumString};

use crate::app_config::AppConfig;

use self::mysql::MySqlDatabase;

pub mod mysql;
pub mod models;
pub mod utils;

// Used only for serializing the cofig. Not an actual enum with databases
#[derive(serde::Serialize, serde::Deserialize, Debug, EnumString, EnumIter, Clone, Copy)]
pub enum AvailableDatabases {
    //#[serde(rename(serialize = "Mysql"))]
    #[strum(serialize = "mysql")]
    MySql
}

impl std::fmt::Display for AvailableDatabases {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            AvailableDatabases::MySql => write!(f, "mysql")
        }
    }
}

#[derive(Clone)]
pub enum DatabaseVariant {
    MySql(MySqlDatabase)
}

pub async fn get_database(conf: &AppConfig) -> DatabaseVariant{
    match conf.db {
        AvailableDatabases::MySql => { 
            // TODO make error handling
            let mysql_database = mysql::connect(conf).await.unwrap();
            DatabaseVariant::MySql(mysql_database)
        },
    }
}

#[async_trait]
pub trait Database : Clone {
    // Create user
    async fn create_user(&self, user: models::User, password: models::Password) -> Result<(), CreateUserError>;
    // Login

    // Change user
}

#[derive(Debug)]
pub enum CreateUserError {
    SqlxError(sqlx::Error)
}
