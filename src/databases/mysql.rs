use crate::app_config::AppConfig;
use actix_web::{HttpResponse, ResponseError};
use derive_more::{Display, From};

pub struct Config {
    host: &'static str,
    port: u16,
    user: &'static str,
    password: &'static str,
    db_name: &'static str,
}

// async fn mysql_connect(conf: AppConfig) -> Result<Config, MysqlConnectError> {
//     let config = {
        
//     }
// }


#[derive(Display, From, Debug)]
pub enum MysqlConnectError {
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
