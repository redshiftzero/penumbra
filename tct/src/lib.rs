//! The tiered commitment tree for Penumbra.
//!
//! ```ascii,no_run
//! Eternity┃           ╱╲ ◀───────────── Anchor           
//!     Tree┃          ╱││╲               = Eternity Root    
//!         ┃         * ** *           ╮                   
//!         ┃      *   *  *   *        │ 8 levels          
//!         ┃   *     *    *     *     ╯                   
//!         ┃  ╱╲    ╱╲    ╱╲    ╱╲                        
//!         ┃ ╱││╲  ╱││╲  ╱││╲  ╱││╲ ◀─── Eternity Leaf      
//!                         ▲             = Epoch Root     
//!                      ┌──┘                              
//!                      │                                 
//!                      │                                 
//!    Epoch┃           ╱╲ ◀───────────── Epoch Root       
//!     Tree┃          ╱││╲                                
//!         ┃         * ** *           ╮                   
//!         ┃      *   *  *   *        │ 8 levels          
//!         ┃   *     *    *     *     ╯                   
//!         ┃  ╱╲    ╱╲    ╱╲    ╱╲                        
//!         ┃ ╱││╲  ╱││╲  ╱││╲  ╱││╲ ◀─── Epoch Leaf       
//!                  ▲                    = Block Root     
//!                  └───┐                                 
//!                      │                                 
//!                      │                                 
//!    Block┃           ╱╲ ◀───────────── Block Root       
//!     Tree┃          ╱││╲                                
//!         ┃         * ** *           ╮                   
//!         ┃      *   *  *   *        │ 8 levels          
//!         ┃   *     *    *     *     ╯                   
//!         ┃  ╱╲    ╱╲    ╱╲    ╱╲                        
//!         ┃ ╱││╲  ╱││╲  ╱││╲  ╱││╲ ◀─── Block Leaf       
//!                                       = Note Commitment
//! ```

// Cargo doc complains if the recursion limit isn't higher, even though cargo build succeeds:
#![recursion_limit = "256"]
#![warn(missing_docs)]

#[macro_use]
extern crate derivative;

use std::fmt::Debug;

use serde::{Deserialize, Serialize};

pub mod internal;
mod serialize;

use internal::{
    active::{Active, Focus, Insert, Item, Tier},
    complete::{Complete, ForgetOwned},
    hash::GetHash,
    hash::Hash,
    height::Height,
    index,
    path::AuthPath,
    proof,
};

/// A commitment to be stored in a [`Block`].
///
/// This is an element of the base field of the curve used by the Poseidon hash function
/// instantiated for BLS12-377.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Commitment(#[serde(with = "crate::serialize::fq")] pub poseidon377::Fq);

impl From<Commitment> for poseidon377::Fq {
    fn from(commitment: Commitment) -> Self {
        commitment.0
    }
}

impl From<poseidon377::Fq> for Commitment {
    fn from(commitment: poseidon377::Fq) -> Self {
        Commitment(commitment)
    }
}

mod eternity;
pub use eternity::{
    epoch::{block::Block, Epoch},
    error, Eternity, Proof, Root, VerifyError,
};

pub mod epoch {
    //! [`Epoch`]s within [`Eternity`](super::Eternity)s, and their [`Root`]s and [`Proof`]s of inclusion.
    pub use crate::eternity::epoch::*;
}

pub mod block {
    //! [`Block`]s within [`Epoch`](super::Epoch)s, and their [`Root`]s and [`Proof`]s of inclusion.
    pub use crate::eternity::epoch::block::*;
}

/// When inserting a [`Commitment`] into an [`Eternity`], [`Epoch`], or [`Block`], should we
/// [`Keep`] it to allow it to be witnessed later, or [`Forget`] about it after updating the root
/// hash?
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Witness {
    /// Keep this commitment so it can be witnessed later.
    Keep,
    /// Forget this commitment so it does not take up space, but it cannot be witnessed later.
    Forget,
}

/// When inserting a [`Commitment`] into an [`Eternity`], [`Epoch`], or [`Block`], this flag
/// indicates that we should immediately forget about it to save space, because we will not want to
/// witness its presence later.
pub use Witness::Forget;

/// When inserting a [`Commitment`] into an [`Eternity`], [`Epoch`], or [`Block`], this flag
/// indicates that we should keep this commitment to allow it to be witnessed later.
pub use Witness::Keep;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_eternity_size() {
        static_assertions::assert_eq_size!(Eternity, [u8; 96]);
    }

    #[test]
    fn check_eternity_proof_size() {
        static_assertions::assert_eq_size!(Proof, [u8; 2344]);
    }
}