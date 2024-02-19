use crate::app_config::AppConfig;
use actix_web::{HttpResponse, ResponseError};
use async_trait::async_trait;
use derive_more::{Display, From};
use sqlx::{Executor, MySql, Pool};

use super::{models, CreateUserError};

#[derive(Clone)]
pub struct Config {
    host: String,
    port: u16,
    user: String,
    password: String,
    db_name: String,
}

// TODO Extract sql queries to seperate file / struct

#[derive(Clone)]
pub struct MySqlDatabase {
    config: Config,
    pool: Pool<MySql>
}

#[async_trait]
impl super::Database for MySqlDatabase {
    async fn create_user(&self, user: models::User, password: models::Password) -> Result<(), CreateUserError> {
        let query = include_str!("../../sql/create_user.sql");
        let query = sqlx::query_as!(
            models::User,
            "SELECT id from ?",
            "abs", ":", "1", "35"
        );
        let result = self.pool.fetch_one(&*query)
            .await
            .map_err(|err| CreateUserError::SqlxError(err))?;
        Ok(())
    }
}

pub async fn connect(conf: &AppConfig) -> Result<MySqlDatabase, MysqlConnectError> {
    let conf = Config {
        host: conf.host.clone().ok_or(MysqlConnectError::CannotFindField("host"))?,
        port: conf.port.ok_or(MysqlConnectError::CannotFindField("port"))?,
        user: conf.user.clone().ok_or(MysqlConnectError::CannotFindField("user"))?,
        password: conf.password.clone().ok_or(MysqlConnectError::CannotFindField("password"))?,
        db_name: conf.db_name.clone().ok_or(MysqlConnectError::CannotFindField("db_name"))?,
    };

    // TODO Make better error handling than "unwrap"
    let url = format!("mysql://{}:{}@{}:{}/{}", conf.user, conf.password, conf.host, conf.port, conf.db_name);
    let pool = Pool::<MySql>::connect(&url)
        .await
        .unwrap();

    Ok(MySqlDatabase {
        config: conf,
        pool: pool
    })
}


#[derive(Display, From, Debug)]
pub enum MysqlConnectError {
    CannotFindField(&'static str),
    NotFound,
}
impl std::error::Error for MysqlConnectError {}

impl ResponseError for MysqlConnectError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            MysqlConnectError::NotFound => HttpResponse::NotFound().finish(),
            _ => HttpResponse::InternalServerError().finish(),
        }
    }

    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
    }
}
