//! A `Scalar` represents an element of the finite field 
//! modulo `2^249 - 15145038707218910765482344729778085401`.
//! 
//! The `Scalar` type is an alias for one of the backend
//! implementations. 
//! 
//! `ConstantTimeEq` and `PartialEq` traits have been implemented 
//! here since they will be the samme across all of the different
//! backends.
//! 
//! Here it is also defined the `Ristretto255Scalar` type,
//! which is a type-alias for the curve25519-dalek Scalar Struct.
//! 

use crate::backend;

use subtle::Choice;
use subtle::ConstantTimeEq;


#[cfg(feature = "u64_backend")]
pub use backend::u64::scalar::*;
/// A `Scalar` represents an element of the field generated by
/// the prime of the sub-group: `2^249 - 15145038707218910765482344729778085401`.
///
/// This is a type alias for one of the Scalar types in the `backend`
/// module.
#[cfg(feature = "u64_backend")]
pub type Scalar = backend::u64::scalar::Scalar;

impl PartialEq for Scalar {
    fn eq(&self, other: &Scalar) -> bool {
        self.ct_eq(other).unwrap_u8() == 1u8
    }
}

impl ConstantTimeEq for Scalar {
    /// Test equality between two `Scalar`s.  Since the
    /// internal representation is not canonical, the field elements
    /// are normalized to wire format before comparison.
    fn ct_eq(&self, other: &Scalar) -> Choice {
        self.to_bytes().ct_eq(&other.to_bytes())
    }
}

impl Eq for Scalar {}

/// This is a type alias for the Scalar type in the `curve25519-dalek` lib.
pub type Ristretto255Scalar = curve25519_dalek::scalar::Scalar;
