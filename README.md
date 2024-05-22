# Triangle Matrix

A simple library for representing flat data structures as a triangle matrix. 

Provides the `TriangleIndex` trait that provides an abstraction of triangle
matrix indexing operations for a one dimensional collection. All operations
are delegated to the inner collection using `Deref` and `DerefMut`. Requires
delegating the length of an axis, `n`, using the `TriangleType` trait.

# Example
```rust
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

use triangle_matrix::*;

// Create a wrapper storing the length of an axis and the collection.
struct VecTri<Ty>(usize, Vec<usize>, PhantomData<Ty>);

// Implement `Deref` and `DerefMut`, delegating `Deref::Target` to the vector.
impl<Ty> Deref for VecTri<Ty> {
    type Target = Vec<usize>;
    fn deref(&self) -> &Self::Target {
        &self.1
    }
}

impl<Ty> DerefMut for VecTri<Ty> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.1
    }
}

// Delegate `n` to the `usize` field and specify `Ty` as the triangle type.
impl<Ty> TriangleType<Ty> for VecTri<Ty> {
    fn n(&self) -> usize {
        self.0
    }
}

// An abstraction of an `4 * 4` triangle matrix where the elements are the usize indices.
let n = 4;
let v = Vec::from_iter(0..tri_num(n));

// Represent the vector as an upper triangle matrix.
let mut m: VecTri<UpperTriangle> = VecTri(n, v, PhantomData);
assert_eq!(m.get_row_indices(0).collect::<Vec<_>>(), [0, 1, 2, 3]);
assert_eq!(m.get_row_indices(1).collect::<Vec<_>>(),    [4, 5, 6]);
assert_eq!(m.get_row_indices(2).collect::<Vec<_>>(),       [7, 8]);
assert_eq!(m.get_row_indices(3).collect::<Vec<_>>(),          [9]);

assert_eq!(m.get_element_index(1, 2), 6);
assert_eq!(*m.get_element(1, 2), 6);

assert_eq!(m.get_col_indices(3).collect::<Vec<_>>(), [3, 6, 8, 9]);

*m.get_element_mut(1, 2) = 10;
*m.get_element_mut(2, 1) = 11;

// Thanks to `Deref`, we can index into the underlying vector.
assert_eq!(m.get_col_indices(3).map(|i| m[i]).collect::<Vec<_>>(), [3, 10, 11, 9]);
```
