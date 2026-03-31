use std::sync::Arc;

#[async_trait::async_trait]
pub trait PasswordHasher: Send + Sync {
    fn hash(&self, password: &str) -> Result<String, String>;
    fn verify(&self, password: &str, hash: &str) -> bool;
}

impl PasswordHasher for Arc<dyn PasswordHasher> {
    fn hash(&self, password: &str) -> Result<String, String> {
        (**self).hash(password)
    }

    fn verify(&self, password: &str, hash: &str) -> bool {
        (**self).verify(password, hash)
    }
}
