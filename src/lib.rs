//! # triangle_matrix
//!
//! Upper and lower triangle matrix indexing operations.
//!
//! Provides indexing operations for one dimensional collections using
//! `Index<usize>`. Requires implementing [`Triangle`] for any `Index<usize>`
//! type, and optionally [`TriangleMut`] for any `IndexMut<usize>` type.
//! Usage requires delegation to the collection, and the axis length, `n`.
//! ```
//! # use crate::triangle_matrix::{Triangle, TriangleMut};
//! #
//! // A vector represented as a triangle matrix.
//! struct TriVec(usize, Vec<usize>);
//!
//! impl Triangle for TriVec {
//!     type Inner = Vec<usize>;
//!
//!     fn n(&self) -> usize {
//!         self.0
//!     }
//!
//!     fn inner(&self) -> &Self::Inner {
//!         &self.1
//!     }
//! }
//!
//! impl TriangleMut for TriVec {
//!     fn inner_mut(&mut self) -> &mut Self::Inner {
//!         &mut self.1
//!     }
//! }
//! ```
//! ## Triangle matrix types
//!
//! For all types, the indices `i` and `j` refer to rows and columns respectively.
//! The indices of a diagonal element (`i == j`) are out of bounds for all types.
//!
//! ### Simple Upper Triangle ([`SimpleUpperTri`], [`SimpleUpperTriMut`])
//!
//! Indexing operations for an upper triangle matrix with no diagonal elements.
//! Allows getting the element or one dimensional index for any `i` and `j` indices
//! where `i < j`. Does not allow indexing into rows or columns outside of the
//! triangle.
//!
//! ```
//! # use crate::triangle_matrix::{Triangle, TriangleMut};
//! # use crate::triangle_matrix::ops::tri_num;
//! #
//! use crate::triangle_matrix::SimpleUpperTri;
//!
//! #  struct TriVec(usize, Vec<usize>);
//! #
//! #  impl Triangle for TriVec {
//! #      type Inner = Vec<usize>;
//! #
//! #      fn n(&self) -> usize {
//! #          self.0
//! #      }
//! #
//! #      fn inner(&self) -> &Self::Inner {
//! #          &self.1
//! #      }
//! #  }
//! #
//! #  impl TriangleMut for TriVec {
//! #      fn inner_mut(&mut self) -> &mut Self::Inner {
//! #          &mut self.1
//! #      }
//! #  }
//! let n = 5;
//! let m = TriVec(n, Vec::from_iter(0..tri_num(n - 1)));
//!
//! // Get the elements of rows 0..4.
//! assert_eq!(m.get_row(0).cloned().collect::<Vec<_>>(), [   0, 1, 2, 3]);
//! assert_eq!(m.get_row(1).cloned().collect::<Vec<_>>(), [      4, 5, 6]);
//! assert_eq!(m.get_row(2).cloned().collect::<Vec<_>>(), [         7, 8]);
//! assert_eq!(m.get_row(3).cloned().collect::<Vec<_>>(), [            9]);
//!
//! assert_eq!(m.get_col_indices(2).collect::<Vec<_>>(), [1, 4]);
//! assert_eq!(m.get_col_indices(4).collect::<Vec<_>>(), [3, 6, 8, 9]);
//!
//! assert_eq!(*m.get_element(0, 3), 2);
//! assert_eq!(*m.get_element(1, 3), 5);
//! assert_eq!(*m.get_element(2, 3), 7);
//! ```
//!
//! ### Symmetric Upper Triangle ([`SymmetricUpperTri`], [`SymmetricUpperTriMut`])
//!
//! Indexing operations for a symmetric upper triangle matrix with no diagonal
//! elements. Allows getting the element or one dimensional index for any `i` and
//! `j` indices where any pair of `(i, j)` indices is equal to the pair, `(j, i)`.
//! Does not allow indexing into rows or columns outside of the triangle.
//!
//! ```
//! # use crate::triangle_matrix::{Triangle, TriangleMut};
//! # use crate::triangle_matrix::ops::tri_num;
//! #
//! use crate::triangle_matrix::SymmetricUpperTri;
//!
//! #  struct TriVec(usize, Vec<usize>);
//! #
//! #  impl Triangle for TriVec {
//! #      type Inner = Vec<usize>;
//! #
//! #      fn n(&self) -> usize {
//! #          self.0
//! #      }
//! #
//! #      fn inner(&self) -> &Self::Inner {
//! #          &self.1
//! #      }
//! #  }
//! #
//! #  impl TriangleMut for TriVec {
//! #      fn inner_mut(&mut self) -> &mut Self::Inner {
//! #          &mut self.1
//! #      }
//! #  }
//! let n = 5;
//! let m = TriVec(n, Vec::from_iter(0..tri_num(n - 1)));
//!
//! // Get the elements of rows 0..5.
//! assert_eq!(m.get_row(0).cloned().collect::<Vec<_>>(), [   0, 1, 2, 3]);
//! assert_eq!(m.get_row(1).cloned().collect::<Vec<_>>(), [0,    4, 5, 6]);
//! assert_eq!(m.get_row(2).cloned().collect::<Vec<_>>(), [1, 4,    7, 8]);
//! assert_eq!(m.get_row(3).cloned().collect::<Vec<_>>(), [2, 5, 7,    9]);
//! assert_eq!(m.get_row(4).cloned().collect::<Vec<_>>(), [3, 6, 8, 9   ]);
//!
//! assert_eq!(m.get_col_indices(2).collect::<Vec<_>>(), [1, 4,     7, 8]);
//! assert_eq!(m.get_col_indices(4).collect::<Vec<_>>(), [3, 6, 8, 9    ]);
//!
//! assert_eq!(*m.get_element(0, 3), 2);
//! assert_eq!(*m.get_element(3, 1), 5);
//! assert_eq!(*m.get_element(3, 2), 7);
//! ```
//!
//! ### Simple Lower Triangle ([`SimpleLowerTri`], [`SimpleLowerTriMut`])
//!
//! Indexing operations for a lower triangle matrix with no diagonal elements.
//! Allows getting the element or one dimensional index for any `i` and `j` indices
//! where `j < i`. Does not allow indexing into rows or columns outside of the
//! triangle.
//!
//! ```
//! # use crate::triangle_matrix::{Triangle, TriangleMut};
//! # use crate::triangle_matrix::ops::tri_num;
//! #
//! use crate::triangle_matrix::SimpleLowerTri;
//!
//! #  struct TriVec(usize, Vec<usize>);
//! #
//! #  impl Triangle for TriVec {
//! #      type Inner = Vec<usize>;
//! #
//! #      fn n(&self) -> usize {
//! #          self.0
//! #      }
//! #
//! #      fn inner(&self) -> &Self::Inner {
//! #          &self.1
//! #      }
//! #  }
//! #
//! #  impl TriangleMut for TriVec {
//! #      fn inner_mut(&mut self) -> &mut Self::Inner {
//! #          &mut self.1
//! #      }
//! #  }
//! let n = 5;
//! let m = TriVec(n, Vec::from_iter(0..tri_num(n - 1)));
//!
//! // Get the elements of rows 1..5.
//! assert_eq!(m.get_row(1).cloned().collect::<Vec<_>>(), [0            ]);
//! assert_eq!(m.get_row(2).cloned().collect::<Vec<_>>(), [1, 2         ]);
//! assert_eq!(m.get_row(3).cloned().collect::<Vec<_>>(), [3, 4, 5      ]);
//! assert_eq!(m.get_row(4).cloned().collect::<Vec<_>>(), [6, 7, 8, 9   ]);
//!
//! assert_eq!(m.get_col_indices(1).collect::<Vec<_>>(), [2, 4, 7]);
//! assert_eq!(m.get_col_indices(3).collect::<Vec<_>>(), [9]);
//!
//! assert_eq!(*m.get_element(2, 1), 2);
//! assert_eq!(*m.get_element(3, 1), 4);
//! assert_eq!(*m.get_element(4, 1), 7);
//! ```
//!
//! ### Symmetric Lower Triangle ([`SymmetricLowerTri`], [`SymmetricLowerTriMut`])
//!
//! Indexing operations for a symmetric lower triangle matrix with no diagonal
//! elements. Allows getting the element or one dimensional index for any `i` and
//! `j` indices where any pair of `(i, j)` indices is equal to the pair, `(j, i)`.
//! Does not allow indexing into rows or columns outside of the triangle.
//!
//! ```
//! # use crate::triangle_matrix::{Triangle, TriangleMut};
//! # use crate::triangle_matrix::ops::tri_num;
//! #
//! use crate::triangle_matrix::SymmetricLowerTri;
//!
//! #  struct TriVec(usize, Vec<usize>);
//! #
//! #  impl Triangle for TriVec {
//! #      type Inner = Vec<usize>;
//! #
//! #      fn n(&self) -> usize {
//! #          self.0
//! #      }
//! #
//! #      fn inner(&self) -> &Self::Inner {
//! #          &self.1
//! #      }
//! #  }
//! #
//! #  impl TriangleMut for TriVec {
//! #      fn inner_mut(&mut self) -> &mut Self::Inner {
//! #          &mut self.1
//! #      }
//! #  }
//! let n = 5;
//! let m = TriVec(n, Vec::from_iter(0..tri_num(n - 1)));
//!
//! // Get the elements of rows 0..5.
//! assert_eq!(m.get_row(0).cloned().collect::<Vec<_>>(), [   0, 1, 3, 6]);
//! assert_eq!(m.get_row(1).cloned().collect::<Vec<_>>(), [0,    2, 4, 7]);
//! assert_eq!(m.get_row(2).cloned().collect::<Vec<_>>(), [1, 2,    5, 8]);
//! assert_eq!(m.get_row(3).cloned().collect::<Vec<_>>(), [3, 4, 5,    9]);
//! assert_eq!(m.get_row(4).cloned().collect::<Vec<_>>(), [6, 7, 8, 9   ]);
//!
//! assert_eq!(m.get_col_indices(2).collect::<Vec<_>>(), [1, 2,     5, 8]);
//! assert_eq!(m.get_col_indices(4).collect::<Vec<_>>(), [6, 7, 8, 9    ]);
//!
//! assert_eq!(*m.get_element(0, 3), 3);
//! assert_eq!(*m.get_element(3, 1), 4);
//! assert_eq!(*m.get_element(3, 2), 5);
//! ```
mod def;

pub mod lower;
pub mod upper;

pub mod ops;

pub use def::{Triangle, TriangleMut};
pub use ops::tri_num;

pub use lower::{SimpleLowerTri, SimpleLowerTriMut, SymmetricLowerTri, SymmetricLowerTriMut};
pub use upper::{SimpleUpperTri, SimpleUpperTriMut, SymmetricUpperTri, SymmetricUpperTriMut};
