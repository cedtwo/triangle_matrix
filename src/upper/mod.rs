//! Upper triangle traits.
mod base;

mod simple;
mod symmetric;

pub use simple::{SimpleUpperTri, SimpleUpperTriMut};
pub use symmetric::{SymmetricUpperTri, SymmetricUpperTriMut};
