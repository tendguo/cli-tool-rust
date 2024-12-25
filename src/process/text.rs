use std::collections::HashMap;

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use ed25519_dalek::{ed25519::signature::SignerMut, SigningKey, VerifyingKey};
use rand::rngs::OsRng;

use anyhow::{Ok, Result};

use crate::cli::TextSignFormat;

use super::process_genpassword;

trait Signer {
    fn sign(&self, message: &[u8]) -> anyhow::Result<Vec<u8>>;
}

trait Verifier {
    fn verify(&self, sig: &str, message: &[u8]) -> anyhow::Result<bool>;
}

pub struct Ed25519Signer {
    key: SigningKey,
}

pub struct Ed25519Verifier {
    key: VerifyingKey,
}

pub struct Blake3 {
    key: [u8; 32],
}

impl Signer for Blake3 {
    fn sign(&self, message: &[u8]) -> anyhow::Result<Vec<u8>> {
        let ret = blake3::keyed_hash(&self.key, message);
        println!("{:?}", ret);
        Ok(ret.as_bytes().to_vec())
    }
}

impl Signer for Ed25519Signer {
    fn sign(&self, message: &[u8]) -> anyhow::Result<Vec<u8>> {
        let signature = self.key.clone().sign(message);
        Ok(signature.to_bytes().to_vec())
    }
}

impl Verifier for Blake3 {
    fn verify(&self, sig: &str, message: &[u8]) -> anyhow::Result<bool> {
        let sig_decode = URL_SAFE_NO_PAD.decode(sig)?;
        let ret = blake3::keyed_hash(&self.key, message);
        Ok(ret.as_bytes().to_vec() == sig_decode)
    }
}

impl Verifier for Ed25519Verifier {
    fn verify(&self, sig: &str, message: &[u8]) -> anyhow::Result<bool> {
        let sig_decode = URL_SAFE_NO_PAD.decode(sig)?;
        let signature = &sig_decode[..64].try_into()?;
        let verify_result = self.key.verify_strict(message, signature).is_ok();
        Ok(verify_result)
    }
}

impl Blake3 {
    pub fn try_new(key: impl AsRef<[u8]>) -> Result<Self> {
        let key = key.as_ref();
        let key = (key[..32]).try_into()?;
        Ok(Self::new(key))
    }

    pub fn new(key: [u8; 32]) -> Self {
        Self { key }
    }

    fn generate() -> Result<HashMap<&'static str, Vec<u8>>> {
        let key = process_genpassword(32, true, true, true, true)?;
        let mut map = HashMap::new();
        map.insert("blake3.txt", key.as_bytes().to_vec());
        Ok(map)
    }
}

impl Ed25519Signer {
    pub fn try_new(key: impl AsRef<[u8]>) -> Result<Self> {
        let key = key.as_ref();
        let key = (&key[..32]).try_into()?;
        Ok(Self::new(key))
    }

    pub fn new(key: &[u8; 32]) -> Self {
        let key = SigningKey::from_bytes(key);
        Self { key }
    }

    fn generate() -> Result<HashMap<&'static str, Vec<u8>>> {
        let mut csprng = OsRng;
        let sk = SigningKey::generate(&mut csprng);
        let pk: VerifyingKey = (&sk).into();
        let mut map = HashMap::new();
        map.insert("ed25519.sk", sk.as_bytes().to_vec());
        map.insert("ed25519.pk", pk.as_bytes().to_vec());
        Ok(map)
    }
}

impl Ed25519Verifier {
    pub fn try_new(key: impl AsRef<[u8]>) -> Result<Self> {
        let key = key.as_ref();
        let key = (&key[..32]).try_into()?;
        let key = VerifyingKey::from_bytes(key)?;
        Ok(Self { key })
    }
}

pub fn process_generate_key(
    format: TextSignFormat,
) -> anyhow::Result<HashMap<&'static str, Vec<u8>>> {
    let key_map: HashMap<&str, Vec<u8>> = match format {
        TextSignFormat::Ed25519 => Ed25519Signer::generate()?,
        TextSignFormat::Blake3 => Blake3::generate()?,
    };

    Ok(key_map)
}

pub fn process_sign_text(format: TextSignFormat, key: &[u8], message: &[u8]) -> anyhow::Result<()> {
    let signer: Box<dyn Signer> = match format {
        TextSignFormat::Ed25519 => Box::new(Ed25519Signer::try_new(key)?),
        TextSignFormat::Blake3 => Box::new(Blake3::try_new(key)?),
    };
    let u = signer.sign(message)?;
    let encoded = URL_SAFE_NO_PAD.encode(u);
    println!("{:?}", encoded);
    Ok(())
}
pub fn process_sign_verify(
    format: TextSignFormat,
    key: &[u8],
    sig: &str,
    message: &[u8],
) -> anyhow::Result<bool> {
    let result: Box<dyn Verifier> = match format {
        TextSignFormat::Ed25519 => Box::new(Ed25519Verifier::try_new(key)?),
        TextSignFormat::Blake3 => Box::new(Blake3::try_new(key)?),
    };

    Ok(result.verify(sig, message)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::TextSignFormat;

    const KEY: &[u8] = include_bytes!("../../encryption/blake3.txt");
    const MESSAGE: &[u8] = include_bytes!("../../encryption/message.txt");

    #[test]
    fn it_works() -> Result<()> {
        let format = TextSignFormat::Blake3;
        let sig = "7EULPwVhxUYD1AbJHsnr2cxZ_5dsvhBRwXISDwzdkOk";
        let result = process_sign_verify(format, KEY, sig, MESSAGE)?;
        assert!(result);
        Ok(())
    }

    #[test]
    fn it_not_works() -> Result<()> {
        let format = TextSignFormat::Blake3;
        let sig = "Ey37iAVJo41c4fBCyCJpyXopC8l_I93MEdasdfasdfGwUxAI_8UY";
        let result = process_sign_verify(format, KEY, sig, MESSAGE)?;
        assert!(!result);
        Ok(())
    }
}
