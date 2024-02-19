use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub last_name: String,
    pub email: String,
    pub nickname: String,
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Password {
    pub user_id: u64,
    pub password: String
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct UserAccount {
    pub user_id: u64,
    pub service_id: u64,
    pub additional_conf: String,
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Salt {
    pub user_id: u64,
    pub salt: String
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct RegisteredService {
    pub id: u64,
    pub name: String,
}