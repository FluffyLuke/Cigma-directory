use argon2::{
    password_hash::{
        rand_core::{Error, OsRng}, PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};

use crate::models::{ToPasswordModel, ToSaltModel};

// TODO Create a token based system

pub struct SaltedPassword {
    hashed_password: String,
    salt: SaltString
}

impl SaltedPassword {
    pub fn new(password: &str) -> SaltedPassword {
        let salt = SaltString::generate(&mut OsRng);
        let argon = Argon2::default();

        // TODO make an error for this, don't unwrap
        let hashed_password = argon.hash_password(password.as_bytes(), &salt).unwrap()
            .to_string();
        
        SaltedPassword {
            hashed_password,
            salt
        }
    }

    pub fn verify(password: &str, salted_password: SaltedPassword) -> Result<(), argon2::password_hash::Error>{
        let hashed_password = PasswordHash::parse(&salted_password.hashed_password, argon2::password_hash::Encoding::default()).unwrap();
        Argon2::default().verify_password(password.as_bytes(), &hashed_password)
    }
}

impl ToPasswordModel for SaltedPassword {
    fn to_password(&self, user: crate::models::User) -> crate::models::Password {
        crate::models::Password {
            user_id: user.user_id,
            password: self.hashed_password.clone(),
        }
    }
}

impl ToSaltModel for SaltedPassword {
    fn to_password(&self, user: crate::models::User) -> crate::models::Salt {
        crate::models::Salt {
            user_id: user.user_id,
            salt: self.salt.clone().to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SaltedPassword;

    #[test]
    fn hash_and_verify() {
        let password = SaltedPassword::new("Password123");
        let result = SaltedPassword::verify("Password123", password);
        assert_eq!(result, Ok(()))
    }
}