// src/crypto.rs

use crypto::aes::Aes256Cbc;
use crypto::mac::Hmac;
use crypto::secretbox::SecretBox;
use crypto::symmetrickey::Key;
use crypto::digest::Digest;
use std::convert::TryInto;
use std::error::Error;
use std::fmt;

// Import des bibliothèques de cryptographie
extern crate crypto;

// Définition des types de cryptage
#[derive(Debug)]
enum CryptAlgo {
    Aes256Cbc,
    HmacSha256,
    SecretBox,
}

#[derive(Debug)]
enum CryptError {
    InvalidKey,
    InvalidNonce,
    InvalidMac,
}

// Implémentation des méthodes de cryptage
impl CryptAlgo {
    fn encrypt(&self, plaintext: &[u8], key: &[u8]) -> Result<Vec<u8>, CryptError> {
        match self {
            CryptAlgo::Aes256Cbc => {
                // Chiffrement avec AES-256-CBC
                let aes = Aes256Cbc::new(key);
                let ciphertext = aes.encrypt(plaintext)?;
                Ok(ciphertext)
            }
            CryptAlgo::HmacSha256 => {
                // Hachage avec HMAC-SHA-256
                let hmac = Hmac::new(&key);
                let mac = hmac.update(plaintext)?;
                Ok(mac)
            }
            CryptAlgo::SecretBox => {
                // Chiffrement avec SecretBox
                let secretbox = SecretBox::new(key);
                let ciphertext = secretbox.encrypt(plaintext)?;
                Ok(ciphertext)
            }
        }
    }

    fn decrypt(&self, ciphertext: &[u8], key: &[u8]) -> Result<Vec<u8>, CryptError> {
        match self {
            CryptAlgo::Aes256Cbc => {
                // Déchiffrement avec AES-256-CBC
                let aes = Aes256Cbc::new(key);
                let plaintext = aes.decrypt(ciphertext)?;
                Ok(plaintext)
            }
            CryptAlgo::HmacSha256 => {
                // Vérification du MAC avec HMAC-SHA-256
                let hmac = Hmac::new(&key);
                let mac = hmac.update(ciphertext)?;
                if mac == ciphertext {
                    Ok(ciphertext)
                } else {
                    Err(CryptError::InvalidMac)
                }
            }
            CryptAlgo::SecretBox => {
                // Déchiffrement avec SecretBox
                let secretbox = SecretBox::new(key);
                let plaintext = secretbox.decrypt(ciphertext)?;
                Ok(plaintext)
            }
        }
    }
}

// Implémentation des méthodes de cryptage pour les clés
impl TryFrom<&[u8]> for Key {
    type Error = Box<dyn Error>;

    fn try_from(key: &[u8]) -> Result<Self, Self::Error> {
        // Conversion de la clé en format de clé
        let key = key.try_into()?;
        Ok(Key::new(key))
    }
}

// Implémentation des méthodes de cryptage pour les nonces
impl TryFrom<&[u8]> for [u8; 12] {
    type Error = Box<dyn Error>;

    fn try_from(nonce: &[u8]) -> Result<Self, Self::Error> {
        // Conversion du nonce en format de nonce
        let nonce = nonce.try_into()?;
        if nonce.len() != 12 {
            return Err(Box::new(CryptError::InvalidNonce));
        }
        Ok(nonce)
    }
}

// Implémentation des méthodes de cryptage pour les MAC
impl TryFrom<&[u8]> for [u8; 32] {
    type Error = Box<dyn Error>;

    fn try_from(mac: &[u8]) -> Result<Self, Self::Error> {
        // Conversion du MAC en format de MAC
        let mac = mac.try_into()?;
        if mac.len() != 32 {
            return Err(Box::new(CryptError::InvalidMac));
        }
        Ok(mac)
    }
}

// Implémentation des méthodes de cryptage pour les erreurs
impl fmt::Display for CryptError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CryptError::InvalidKey => write!(f, "Clé invalid"),
            CryptError::InvalidNonce => write!(f, "Nonce invalid"),
            CryptError::InvalidMac => write!(f, "MAC invalid"),
        }
    }
}

impl Error for CryptError {}