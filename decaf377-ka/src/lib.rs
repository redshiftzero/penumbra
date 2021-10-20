use ark_ff::UniformRand;
use rand_core::{CryptoRng, RngCore};
use std::convert::TryInto;
use zeroize::Zeroize;

use decaf377;

/// A public key sent to the counterparty in the key agreement protocol.
///
/// This is a refinement type around `[u8; 32]` that marks the bytes as being a
/// public key.  Not all 32-byte arrays are valid public keys; invalid public
/// keys will error during key agreement.
#[derive(Clone)]
pub struct Public(pub [u8; 32]);

/// A secret key used to perform key agreement using the counterparty's public key.
#[derive(Clone, Zeroize)]
#[zeroize(drop)]
pub struct Secret(decaf377::Fr);

/// The shared secret derived at the end of the key agreement protocol.
#[derive(PartialEq, Eq, Clone, Zeroize)]
#[zeroize(drop)]
pub struct SharedSecret(pub [u8; 32]);

/// An error during key agreement.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Invalid public key")]
    InvalidPublic(Public),
    #[error("Public key bytes are incorrect length")]
    SliceLenError,
}

impl Secret {
    /// Generate a new secret key using `rng`.
    pub fn new<R: RngCore + CryptoRng>(mut rng: R) -> Self {
        Self(decaf377::Fr::rand(&mut rng))
    }

    /// Use the supplied field element as the secret key directly.
    ///
    /// # Warning
    ///
    /// This function exists to allow custom key derivation; it's the caller's
    /// responsibility to ensure that the input was generated securely.
    pub fn new_from_field(sk: decaf377::Fr) -> Self {
        Self(sk)
    }

    /// Derive a public key for this secret key, using the conventional
    /// `decaf377` generator.
    pub fn public(&self) -> Public {
        self.diversified_public(&decaf377::basepoint())
    }

    /// Derive a diversified public key for this secret key, using the provided
    /// `diversified_generator`.
    ///
    /// Since key agreement does not depend on the basepoint, only on the secret
    /// key and the public key, a single secret key can correspond to many
    /// different (unlinkable) public keys.
    pub fn diversified_public(&self, diversified_generator: &decaf377::Element) -> Public {
        Public((self.0 * diversified_generator).compress().into())
    }

    /// Perform key agreement with the provided public key.
    ///
    /// Fails if the provided public key is invalid.
    pub fn key_agreement_with(&self, other: &Public) -> Result<SharedSecret, Error> {
        let pk = decaf377::Encoding(other.0)
            .decompress()
            .map_err(|_| Error::InvalidPublic(other.clone()))?;

        Ok(SharedSecret((self.0 * pk).compress().into()))
    }
}

impl std::fmt::Debug for Public {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "decaf377_ka::Public({})",
            hex::encode(&self.0[..])
        ))
    }
}

impl std::fmt::Debug for Secret {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use decaf377::FieldExt;
        let bytes = self.0.to_bytes();
        f.write_fmt(format_args!(
            "decaf377_ka::Secret({})",
            hex::encode(&bytes[..])
        ))
    }
}

impl std::fmt::Debug for SharedSecret {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "decaf377_ka::SharedSecret({})",
            hex::encode(&self.0[..])
        ))
    }
}

impl std::convert::TryFrom<&[u8]> for Public {
    type Error = Error;

    fn try_from(slice: &[u8]) -> Result<Public, Error> {
        let bytes: [u8; 32] = slice.try_into().map_err(|_| Error::SliceLenError)?;
        Ok(Public(bytes))
    }
}