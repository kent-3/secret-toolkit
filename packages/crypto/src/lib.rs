//! # Crypto
//! This crate contains common cryptography tools used in the development of Secret Contracts
//! running on the Secret Network.
//!
//! Note: It has a deep dependency tree and increases compilation times significantly.
//! 
//! Add the following to your `cargo.toml` file:
//! 
//! ```toml
//! [dependencies]
//! secret-toolkit = { version = "0.3.0", features = ["crypto"] }
//! secret-toolkit-crypto = { version = "0.3.0", features = ["hash", "rand", "ecc-secp256k1"] }
//! ```
//! 
//! ## Example usage:
//! ```rust
//! use secret_toolkit::{
//!     crypto::secp256k1::{PrivateKey, PublicKey, Signature},
//!     crypto::{sha_256, Prng},
//! };
//!
//! let entropy: String = "secret".to_owned();
//! let prng_seed: Vec<u8> = sha_256(base64::encode(entropy.clone()).as_bytes()).to_vec();
//!
//! let mut rng = Prng::new(prng_seed, entropy.as_bytes());
//!
//! let private_key: PrivateKey = PrivateKey::parse(&rng.randbytes());
//! let public_key: PublicKey = private_key.pubkey();
//!
//! let message: &[u8] = b"message";
//! let signature: Signature = private_key.sign(message, deps.api);
//! ```
//! 
//! ### Cargo Features
//! - `["hash"]` - Provides an easy-to-use `sha256` function. Uses [sha2](https://crates.io/crates/sha2).
//! - `["rand"]` - Used to generate pseudo-random numbers. Uses [rand_chacha] and [rand_core].
//! - `["ecc-secp256k1"]` - Contains types and methods for working with secp256k1 keys and signatures, 
//! as well as standard constants for key sizes. Uses [secp256k1](https://crates.io/crates/secp256k1).
//!
#[cfg(feature = "hash")]
mod hash;
#[cfg(feature = "rand")]
mod rng;
#[cfg(feature = "ecc-secp256k1")]
pub mod secp256k1;

#[cfg(feature = "hash")]
pub use hash::{sha_256, SHA256_HASH_SIZE};

#[cfg(feature = "rand")]
pub use rng::Prng;
