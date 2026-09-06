use crate::application::ports::password_hasher::PasswordHasher;
use argon2::Argon2;
use argon2::password_hash::{PasswordHasher as Argon2Hasher, PasswordVerifier};

use argon2::password_hash::phc::{PasswordHash, SaltString};

#[derive(Default, Debug)]
pub struct Argon2HasherImpl {}

impl PasswordHasher for Argon2HasherImpl {
    fn hash(&self, password: &str) -> Result<String, String> {
        // argon2 auto generate salt string new
        let argon2 = Argon2::default();
        argon2
            .hash_password(password.as_bytes())
            .map(|p| p.to_string())
            .map_err(|_| "App encryption argon2 error: Failed to generate hash".to_string())
    }

    fn verify(&self, password: &str, hash: &str) -> bool {
        let Ok(parsed_hash) = PasswordHash::new(hash) else {
            return false;
        };

        Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok()
    }
}
