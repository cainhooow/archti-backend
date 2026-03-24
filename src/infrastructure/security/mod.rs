use crate::application::ports::password_hasher::PasswordHasher;
use argon2::Argon2;
use argon2::password_hash::{
    PasswordHash, PasswordHasher as Argon2Hasher, PasswordVerifier, SaltString, rand_core::OsRng,
};

#[derive(Default, Debug)]
pub struct Argon2HasherImpl {}

impl PasswordHasher for Argon2HasherImpl {
    fn hash(&self, password: &str) -> Result<String, String> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        argon2
            .hash_password(password.as_bytes(), &salt)
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
