pub trait DocumentEncryption: Send + Sync {
    fn encrypt(&self, plan_text: &[u8]) -> Result<Vec<u8>, String>;
    fn decrypt(&self, cipher_text: &[u8]) -> Result<Vec<u8>, String>;
}
