use super::error::prelude::*;

use crate::sha2::{Digest, Sha256};

pub type DefaultHash = Sha256;
pub const HASHBYTES: usize = 32;
pub const EMPTY_HASH_BYTES: [u8; HASHBYTES] = [
    227, 176, 196, 66, 152, 252, 28, 20, 154, 251, 244, 200, 153, 111, 185, 36, 39, 174, 65, 228,
    100, 155, 147, 76, 164, 149, 153, 27, 120, 82, 184, 85,
];

pub trait TreeHash {
    fn hash(input: &[u8]) -> LedgerResult<Vec<u8>>;
    fn hash_leaf<T>(leaf: &T) -> LedgerResult<Vec<u8>>
    where
        T: Hashable;
    fn hash_nodes<T>(left: &T, right: &T) -> LedgerResult<Vec<u8>>
    where
        T: Hashable;
}

impl<H: Digest> TreeHash for H {
    fn hash(input: &[u8]) -> LedgerResult<Vec<u8>> {
        let mut ctx = Self::new();
        ctx.input(input);
        Ok(ctx.result().to_vec())
    }

    fn hash_leaf<T>(leaf: &T) -> LedgerResult<Vec<u8>>
    where
        T: Hashable,
    {
        let mut ctx = Self::new();
        ctx.input(&[0x00]);
        leaf.update_context(&mut ctx)?;
        Ok(ctx.result().to_vec())
    }

    fn hash_nodes<T>(left: &T, right: &T) -> LedgerResult<Vec<u8>>
    where
        T: Hashable,
    {
        let mut ctx = Self::new();
        ctx.input(&[0x01]);
        left.update_context(&mut ctx)?;
        right.update_context(&mut ctx)?;
        Ok(ctx.result().to_vec())
    }
}

/// The type of values stored in a `MerkleTree` must implement
/// this trait, in order for them to be able to be fed
/// to a Ring `Context` when computing the hash of a leaf.
///
/// A default instance for types that already implements
/// `AsRef<[u8]>` is provided.
///
/// ## Example
///
/// Here is an example of how to implement `Hashable` for a type
/// that does not (or cannot) implement `AsRef<[u8]>`:
///
/// ```ignore
/// impl Hashable for PublicKey {
///     fn update_context(&self, context: &mut Hasher) -> Result<(), CommonError> {
///         let bytes: Vec<u8> = self.to_bytes();
///         Ok(context.update(&bytes)?)
///     }
/// }
/// ```
pub trait Hashable {
    /// Update the given `context` with `self`.
    ///
    /// See `openssl::hash::Hasher::update` for more information.
    fn update_context<D: Digest>(&self, context: &mut D) -> LedgerResult<()>;
}

impl<T: AsRef<[u8]>> Hashable for T {
    fn update_context<D: Digest>(&self, context: &mut D) -> LedgerResult<()> {
        Ok(context.input(self.as_ref()))
    }
}
