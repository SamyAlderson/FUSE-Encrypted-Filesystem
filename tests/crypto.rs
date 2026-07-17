// tests/crypto.rs

use crate::crypto;
use crate::errors::Error;
use crypto::Cipher;
use crypto::EncryptionError;
use crypto::DecryptionError;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
struct TestVector {
    plaintext: String,
    ciphertext: String,
    key: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct TestResult {
    encrypted: String,
    decrypted: String,
}

#[tokio::test]
async fn test_aes_encrypt_decrypt() {
    let test_vectors = vec![
        TestVector {
            plaintext: "Hello, World!".to_string(),
            ciphertext: "U2FtdW0lMjBUb3BlbigpIHsgeyAgICBzdHJpbmc6ICdNYW5hZ2VyJ3QnLi4uKQ==".to_string(),
            key: "secret_key_1234567890abcdef".to_string(),
        },
        TestVector {
            plaintext: "This is a test".to_string(),
            ciphertext: "U2FtdW0lMjBCeSBIb3Vubw==".to_string(),
            key: "another_secret_key_1234567890abcdef".to_string(),
        },
    ];

    for test_vector in test_vectors {
        let encrypted = crypto::encrypt(&test_vector.plaintext, &test_vector.key);
        assert_eq!(encrypted, test_vector.ciphertext);

        let decrypted = crypto::decrypt(&encrypted, &test_vector.key);
        assert_eq!(decrypted, test_vector.plaintext);
    }
}

#[tokio::test]
async fn test_ecc_encrypt_decrypt() {
    let test_vectors = vec![
        TestVector {
            plaintext: "Hello, World!".to_string(),
            ciphertext: "0x9a6b1d8e8d5c4b3a2a1a0a0a0a0a0a0a0a0a0a".to_string(),
            key: "secret_key_ecc_1234567890abcdef".to_string(),
        },
        TestVector {
            plaintext: "This is a test".to_string(),
            ciphertext: "0x9a6b1d8e8d5c4b3a2a1a0a0a0a0a0a0a0a0a0a".to_string(),
            key: "another_secret_key_ecc_1234567890abcdef".to_string(),
        },
    ];

    for test_vector in test_vectors {
        let encrypted = crypto::ecc_encrypt(&test_vector.plaintext, &test_vector.key);
        assert_eq!(encrypted, test_vector.ciphertext);

        let decrypted = crypto::ecc_decrypt(&encrypted, &test_vector.key);
        assert_eq!(decrypted, test_vector.plaintext);
    }
}

#[tokio::test]
async fn test_aes_encrypt_decrypt_key_rotation() {
    let plaintext = "Hello, World!".to_string();
    let key = "secret_key_1234567890abcdef".to_string();
    let encrypted = crypto::encrypt(&plaintext, &key);
    assert!(encrypted.is_ok());

    let new_key = "another_secret_key_1234567890abcdef".to_string();
    let decrypted = crypto::decrypt(&encrypted, &new_key).unwrap_err();
    assert_eq!(decrypted, DecryptionError::KeyRotationError);
}

#[tokio::test]
async fn test_ecc_encrypt_decrypt_key_rotation() {
    let plaintext = "Hello, World!".to_string();
    let key = "secret_key_ecc_1234567890abcdef".to_string();
    let encrypted = crypto::ecc_encrypt(&plaintext, &key);
    assert!(encrypted.is_ok());

    let new_key = "another_secret_key_ecc_1234567890abcdef".to_string();
    let decrypted = crypto::ecc_decrypt(&encrypted, &new_key).unwrap_err();
    assert_eq!(decrypted, DecryptionError::KeyRotationError);
}

#[tokio::test]
async fn test_aes_encrypt_decrypt_invalid_input() {
    let invalid_input = "Invalid input".to_string();
    let key = "secret_key_1234567890abcdef".to_string();

    let encrypted = crypto::encrypt(&invalid_input, &key);
    assert!(encrypted.is_err());

    let decrypted = crypto::decrypt(&invalid_input, &key);
    assert!(decrypted.is_err());
}

#[tokio::test]
async fn test_ecc_encrypt_decrypt_invalid_input() {
    let invalid_input = "Invalid input".to_string();
    let key = "secret_key_ecc_1234567890abcdef".to_string();

    let encrypted = crypto::ecc_encrypt(&invalid_input, &key);
    assert!(encrypted.is_err());

    let decrypted = crypto::ecc_decrypt(&invalid_input, &key);
    assert!(decrypted.is_err());
}