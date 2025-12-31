use crate::error::{DeployError, Result};
use base64::{engine::general_purpose::STANDARD, Engine};
use rand::rngs::OsRng;
use rsa::pkcs1v15::{SigningKey, VerifyingKey};
use rsa::pkcs8::{DecodePrivateKey, DecodePublicKey, EncodePrivateKey, EncodePublicKey, LineEnding};
use rsa::sha2::Sha256;
use rsa::signature::{SignatureEncoding, Signer, Verifier};
use rsa::{RsaPrivateKey, RsaPublicKey};
use std::fs;
use std::path::Path;

const RSA_KEY_BITS: usize = 2048;

/// Generate a new RSA key pair
pub fn generate_key_pair() -> Result<(RsaPrivateKey, RsaPublicKey)> {
    let mut rng = OsRng;
    let private_key = RsaPrivateKey::new(&mut rng, RSA_KEY_BITS)
        .map_err(|e| DeployError::Other(format!("Failed to generate RSA key: {}", e)))?;
    let public_key = RsaPublicKey::from(&private_key);
    Ok((private_key, public_key))
}

/// Save private key to PEM file
pub fn save_private_key(key: &RsaPrivateKey, path: &Path) -> Result<()> {
    let pem = key
        .to_pkcs8_pem(LineEnding::LF)
        .map_err(|e| DeployError::Other(format!("Failed to encode private key: {}", e)))?;

    // Create parent directories if needed
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(path, pem.as_bytes())?;
    Ok(())
}

/// Save public key to PEM file
pub fn save_public_key(key: &RsaPublicKey, path: &Path) -> Result<()> {
    let pem = key
        .to_public_key_pem(LineEnding::LF)
        .map_err(|e| DeployError::Other(format!("Failed to encode public key: {}", e)))?;

    // Create parent directories if needed
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(path, pem)?;
    Ok(())
}

/// Load private key from PEM file
pub fn load_private_key(path: &Path) -> Result<RsaPrivateKey> {
    let pem = fs::read_to_string(path)?;
    RsaPrivateKey::from_pkcs8_pem(&pem)
        .map_err(|e| DeployError::InvalidKeyFormat(format!("Invalid private key: {}", e)))
}

/// Load public key from PEM file
pub fn load_public_key(path: &Path) -> Result<RsaPublicKey> {
    let pem = fs::read_to_string(path)?;
    RsaPublicKey::from_public_key_pem(&pem)
        .map_err(|e| DeployError::InvalidKeyFormat(format!("Invalid public key: {}", e)))
}

/// Sign data with private key
pub fn sign(private_key: &RsaPrivateKey, data: &[u8]) -> Result<String> {
    let signing_key = SigningKey::<Sha256>::new(private_key.clone());
    let signature = signing_key.sign(data);
    Ok(STANDARD.encode(signature.to_bytes()))
}

/// Verify signature with public key
pub fn verify(public_key: &RsaPublicKey, data: &[u8], signature_b64: &str) -> Result<bool> {
    let signature_bytes = STANDARD
        .decode(signature_b64)
        .map_err(|e| DeployError::Other(format!("Invalid signature encoding: {}", e)))?;

    let verifying_key = VerifyingKey::<Sha256>::new(public_key.clone());
    let signature = rsa::pkcs1v15::Signature::try_from(signature_bytes.as_slice())
        .map_err(|e| DeployError::Other(format!("Invalid signature format: {}", e)))?;

    match verifying_key.verify(data, &signature) {
        Ok(()) => Ok(true),
        Err(_) => Ok(false),
    }
}

/// Sign a file and save signature
pub fn sign_file(private_key_path: &Path, data_path: &Path, sig_path: &Path) -> Result<()> {
    let private_key = load_private_key(private_key_path)?;
    let data = fs::read(data_path)?;
    let signature = sign(&private_key, &data)?;
    fs::write(sig_path, signature)?;
    Ok(())
}

/// Verify a file's signature
pub fn verify_file(public_key_path: &Path, data_path: &Path, sig_path: &Path) -> Result<bool> {
    let public_key = load_public_key(public_key_path)?;
    let data = fs::read(data_path)?;
    let signature = fs::read_to_string(sig_path)?;
    verify(&public_key, &data, signature.trim())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_key_generation_and_signing() {
        let (private_key, public_key) = generate_key_pair().unwrap();

        let data = b"Hello, World!";
        let signature = sign(&private_key, data).unwrap();

        assert!(verify(&public_key, data, &signature).unwrap());
        assert!(!verify(&public_key, b"Modified data", &signature).unwrap());
    }

    #[test]
    fn test_key_save_and_load() {
        let dir = tempdir().unwrap();
        let private_path = dir.path().join("private.pem");
        let public_path = dir.path().join("public.pem");

        let (private_key, public_key) = generate_key_pair().unwrap();
        save_private_key(&private_key, &private_path).unwrap();
        save_public_key(&public_key, &public_path).unwrap();

        let loaded_private = load_private_key(&private_path).unwrap();
        let loaded_public = load_public_key(&public_path).unwrap();

        let data = b"Test data";
        let signature = sign(&loaded_private, data).unwrap();
        assert!(verify(&loaded_public, data, &signature).unwrap());
    }
}
