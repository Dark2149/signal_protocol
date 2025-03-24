use curve25519_dalek::constants::RISTRETTO_BASEPOINT_POINT;
use curve25519_dalek::scalar::Scalar;
use curve25519_dalek::ristretto::RistrettoPoint;
use rand_core::{CryptoRng, RngCore}

#[cfg(feature = "std")]
use std::vec::Vec;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

/// Представляет пару ключей
pub struct KeyPair {
    pub private: PrivateKey,
    pub public: PublicKey,
}

pub struct PrivateKey(Scalar);

pub struct PublicKey(RistrettoPoint);

impl KeyPair {
    pub fn generate<R: CryptoRng + RngCore>(rng: &mut R) -> Self {
        let private = PrivateKey::generate(rng);
        let public = PublicKey::from(&private);
        Self {private, public}
    }
}