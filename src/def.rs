//! Triangle matrix type definition.

use std::ops::{Index, IndexMut};

/// A triangle matrix abstraction type.
pub trait Triangle
where
    Self::Inner: Index<usize>,
{
    /// The inner collection type
    type Inner;

    /// The length of either axis of the array.
    fn n(&self) -> usize;

    /// The inner collection.
    fn inner(&self) -> &Self::Inner;
}

/// A mutable triangle matrix abstraction type.
pub trait TriangleMut: Triangle
where
    Self::Inner: IndexMut<usize>,
{
    /// The inner collection.
    fn inner_mut(&mut self) -> &mut Self::Inner;
}
