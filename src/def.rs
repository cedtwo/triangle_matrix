//! Triangle matrix type definition.

use std::ops::{Deref, DerefMut};

/// A triangle matrix abstraction type.
pub trait Triangle<T> {
    /// The inner collection type
    type Inner: Deref<Target = [T]>;

    /// The length of either axis of the array.
    fn n(&self) -> usize;

    /// The inner collection.
    fn inner(&self) -> &Self::Inner;
}

/// A mutable triangle matrix abstraction type.
pub trait TriangleMut<T>: Triangle<T>
where
    Self::Inner: DerefMut<Target = [T]>,
{
    /// The inner collection.
    fn inner_mut(&mut self) -> &mut Self::Inner;
}
