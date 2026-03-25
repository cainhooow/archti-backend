use crate::application::ports::document_encryption::DocumentEncryption;


pub struct DocumentEncryptionImpl {}

impl DocumentEncryption for DocumentEncryptionImpl {
    fn encrypt(&self, plan_text: &[u8]) -> Result<Vec<&[u8]>, String> {
        let app_key = env::var("APP_KEY").expect("APP_KEY is not defined in .env file");
        
        Ok(vec![plan_text])
    }

    fn decrypt(&self, cipher_text: &[u8]) -> Result<Vec<&[u8]>, String> {
        let app_key = env::var("APP_KEY").expect("APP_KEY is not defined in .env file");
        
        Ok(vec![cipher_text])
    }
}