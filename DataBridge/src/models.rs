use serde::{Deserialize, Serialize};

use crate::hashing::SaltedPassword;

#[derive(Serialize, Deserialize, Debug)]
pub struct Password {
    pub user_id: u64,
    pub password: String
}

impl Password {
    pub fn hash_and_salt(&self) -> SaltedPassword {
        SaltedPassword::new(&self.password)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Salt {
    pub user_id: u64,
    pub salt: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub user_id: u64,
    pub name: String,
    pub last_name: String,
    pub email: String,
    pub nickname: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WeakUser {
    pub password: String,
    pub email: String,
    pub nickname: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserAccounts {
    pub user_id: u64,
    pub service_id: u64,
    pub conf: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RegisteredService {
    pub service_id: u64,
    pub service_name: String
}

type RegisteredServices = Vec<RegisteredService>;

#[derive(Serialize, Deserialize, Debug)]
pub struct UserInfo {
    user: User,
    password: Password,
    accounts: Vec<UserAccounts>,
    salt: Salt
}

pub trait ToPasswordModel {
    fn to_password(&self, user: User) -> Password;
}

pub trait ToSaltModel {
    fn to_password(&self, user: User) -> Salt;
}