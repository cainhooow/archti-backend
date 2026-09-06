// use magic_crypt::{MagicCryptTrait, new_magic_crypt};
// use std::env;

// use crate::application::ports::document_encryption::DocumentEncryption as DocumentEncryptionPort;

// #[derive(Default, Debug)]
// pub struct AppDocumentEncryption {}

// impl DocumentEncryptionPort for AppDocumentEncryption {
//     fn encrypt(&self, plan_text: &[u8]) -> Result<Vec<u8>, String> {
//         let app_key = env::var("APP_KEY").expect("APP_KEY is not defined in .env file");
//         let magic = new_magic_crypt!(app_key, 256);

//         let encrypted = magic.encrypt_bytes_to_base64(plan_text);
//         let encrypted = encrypted.as_bytes().to_vec();

//         Ok(encrypted)
//     }

//     fn decrypt(&self, cipher_text: &[u8]) -> Result<Vec<u8>, String> {
//         let app_key = env::var("APP_KEY").expect("APP_KEY is not defined in .env file");
//         let magic = new_magic_crypt!(app_key, 256);

//         let text = std::str::from_utf8(cipher_text).map_err(|e| e.to_string())?;
//         let decrypted = magic
//             .decrypt_base64_to_bytes(text)
//             .map_err(|e| e.to_string())?;
//         let decrypted = decrypted.into_iter().collect::<Vec<u8>>();

//         Ok(decrypted)
//     }
// }
