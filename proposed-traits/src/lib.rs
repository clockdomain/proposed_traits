#![no_std]
#![deny(unsafe_code)]

//! Compatibility layer for the original proposed-traits crate.
//! 
//! This crate re-exports all traits from the separate peripheral_traits,
//! crypto_traits, and messaging_traits crates for backward compatibility.
//! 
//! For new projects, consider using the specific crates directly:
//! - `peripheral_traits` for peripheral device abstractions
//! - `crypto_traits` for cryptographic algorithm abstractions  
//! - `messaging_traits` for inter-service messaging

// Re-export peripheral traits
pub use peripheral_traits::*;

// Re-export crypto traits
pub use crypto_traits::*;

// Re-export messaging traits - avoid common module conflict
pub use messaging_traits::{client, service};
pub mod common {
    pub use messaging_traits::common::*;
}
