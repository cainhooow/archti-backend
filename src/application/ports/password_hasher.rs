#[async_trait::async_trait]
pub trait PasswordHasher: Send + Sync {
    fn hash(&self, password: &str) -> Result<String, String>;
    fn verify(&self, password: &str, hash: &str) -> bool;
}