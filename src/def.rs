//! Triangle matrix type definition.

use std::ops::{Deref, DerefMut, Index, IndexMut};

/// Upper triangle marker type.
pub enum UpperTriangle {}

/// Lower triange marker type.
pub enum LowerTriangle {}

/// A triangle matrix abstraction type.
///
/// This should be implemented on a type where the target of `Deref` and `DerefMut`
/// is the inner collection, `C`.
pub trait TriangleType<Ty>: Deref + DerefMut
where
    Self::Target: Index<usize> + IndexMut<usize>,
{
    /// The length of either axis of the array.
    fn n(&self) -> usize;
}
