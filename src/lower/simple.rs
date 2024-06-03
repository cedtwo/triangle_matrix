//! A simple upper triangle abstraction.
use std::ops::DerefMut;

use super::base;
use crate::{Triangle, TriangleMut};

/// A simple lower triangle collection.
///
/// Contains `tri_num(n)` elements with `n - 1` rows and columns to account for
/// the diagonal.
///
/// Any index outside of the lower triangle will cause a panic.
pub trait SimpleLowerTri<T>: Triangle<T> {
    /// Get a reference to an element.
    fn get_element<'a>(&'a self, i: usize, j: usize) -> &'a T {
        debug_assert!(i <= self.n() - 1);
        debug_assert!(j <= self.n() - 1);

        assert!(i != 0);
        assert!(j < i);

        let index = base::get_element_index(i - 1, j);
        &self.inner()[index]
    }

    /// Get an iterator of references to elements of a row.
    fn get_row<'a>(&'a self, i: usize) -> impl Iterator<Item = &'a T>
    where
        T: 'a,
    {
        SimpleLowerTri::get_row_indices(self, i).map(|el| &self.inner()[el])
    }

    /// Get an iterator of references to elements of a col.
    fn get_col<'a>(&'a self, i: usize) -> impl Iterator<Item = &'a T>
    where
        T: 'a,
    {
        SimpleLowerTri::get_col_indices(self, i).map(|el| &self.inner()[el])
    }

    /// Get the first index of a row.
    fn get_row_start_index(&self, i: usize) -> usize {
        debug_assert!(i <= self.n() - 1);

        assert!(i != 0);
        base::get_row_start_index(i - 1)
    }

    /// Get the first index of a column.
    fn get_col_start_index(&self, j: usize) -> usize {
        debug_assert!(j <= self.n() - 1);

        base::get_col_start_index(j)
    }

    /// Get all indices of a row.
    fn get_row_indices<'a, 'b>(&'a self, i: usize) -> impl Iterator<Item = usize> + 'b {
        debug_assert!(i <= self.n() - 1);

        assert!(i != 0);

        base::get_row_indices(i - 1)
    }

    /// Get all indices of a column.
    fn get_col_indices<'a, 'b>(&'a self, j: usize) -> impl Iterator<Item = usize> + 'b {
        debug_assert!(j <= self.n() - 1);

        base::get_col_indices(j, self.n() - 1)
    }

    /// Iterate all `(i, j)` indices of the triangle.
    fn iter_triangle_indices<'a, 'b>(&'a self) -> impl Iterator<Item = (usize, usize)> + 'b {
        base::iter_triangle_indices(self.n() - 1).map(|(i, j)| (i + 1, j))
    }
}

impl<T, U: Triangle<T>> SimpleLowerTri<T> for U {}

pub trait SimpleLowerTriMut<T>: Triangle<T> + TriangleMut<T>
where
    Self::Inner: DerefMut<Target = [T]>,
{
    /// Get a mutable reference to an element.
    fn get_element_mut<'a>(&'a mut self, i: usize, j: usize) -> &'a mut T {
        debug_assert!(i <= self.n() - 1);
        debug_assert!(j <= self.n() - 1);

        assert!(i != 0);
        assert!(j < i);

        let index = base::get_element_index(i - 1, j);
        &mut self.inner_mut().deref_mut()[index]
    }
}

impl<T, U: Triangle<T> + TriangleMut<T>> SimpleLowerTriMut<T> for U where
    Self::Inner: DerefMut<Target = [T]>
{
}

#[cfg(test)]
mod tests {

    mod lower_triangle {

        use crate::{SimpleLowerTri, SimpleLowerTriMut};
        use crate::{Triangle, TriangleMut};

        struct LoTriVec(usize, Vec<usize>);

        impl Triangle<usize> for LoTriVec {
            type Inner = Vec<usize>;

            fn n(&self) -> usize {
                self.0
            }

            fn inner(&self) -> &Vec<usize> {
                &self.1
            }
        }

        impl TriangleMut<usize> for LoTriVec {
            fn inner_mut(&mut self) -> &mut Vec<usize> {
                &mut self.1
            }
        }

        #[test]
        fn test_get_element() {
            #[rustfmt::skip]
            let v = vec![
                0,
                1, 2,
                3, 4, 5,
                6, 7, 8, 9,
            ];
            let n = 5;
            let m = LoTriVec(n, v);

            assert_eq!(*m.get_element(1, 0), 0);
            assert_eq!(*m.get_element(2, 0), 1);
            assert_eq!(*m.get_element(2, 1), 2);
            assert_eq!(*m.get_element(3, 0), 3);
            assert_eq!(*m.get_element(3, 1), 4);
            assert_eq!(*m.get_element(3, 2), 5);
            assert_eq!(*m.get_element(4, 0), 6);
            assert_eq!(*m.get_element(4, 1), 7);
            assert_eq!(*m.get_element(4, 2), 8);
            assert_eq!(*m.get_element(4, 3), 9);
        }
        #[test]

        fn test_get_element_mut() {
            #[rustfmt::skip]
            let v = vec![
                0,
                1, 2,
                3, 4, 5,
                6, 7, 8, 9,
            ];
            let n = 5;
            let mut m = LoTriVec(n, v);
            *m.get_element_mut(3, 1) = 10;
            *m.get_element_mut(3, 2) = 11;

            assert_eq!(*m.get_element(3, 1), 10);
            assert_eq!(*m.get_element(3, 2), 11);
        }

        #[test]
        fn test_get_row_start() {
            #[rustfmt::skip]
            let v = vec![
                0,
                1, 2,
                3, 4, 5,
                6, 7, 8, 9,
            ];
            let n = 5;
            let m = LoTriVec(n, v);

            assert_eq!(m.get_row_start_index(1), 0);
            assert_eq!(m.get_row_start_index(2), 1);
            assert_eq!(m.get_row_start_index(3), 3);
            assert_eq!(m.get_row_start_index(4), 6);
        }

        #[test]
        fn test_get_col_start() {
            #[rustfmt::skip]
            let v = vec![
                0,
                1, 2,
                3, 4, 5,
                6, 7, 8, 9,
            ];
            let n = 5;
            let m = LoTriVec(n, v);

            assert_eq!(m.get_col_start_index(0), 0);
            assert_eq!(m.get_col_start_index(1), 2);
            assert_eq!(m.get_col_start_index(2), 5);
            assert_eq!(m.get_col_start_index(3), 9);
        }

        #[test]
        fn test_get_row_indices() {
            #[rustfmt::skip]
            let v = vec![
                0,
                1, 2,
                3, 4, 5,
                6, 7, 8, 9,
            ];
            let n = 5;
            let m = LoTriVec(n, v);

            assert_eq!(m.get_row_indices(1).collect::<Vec<_>>(), [0]);
            assert_eq!(m.get_row_indices(2).collect::<Vec<_>>(), [1, 2]);
            assert_eq!(m.get_row_indices(3).collect::<Vec<_>>(), [3, 4, 5]);
            assert_eq!(m.get_row_indices(4).collect::<Vec<_>>(), [6, 7, 8, 9]);
        }

        #[test]
        fn test_get_col_indices() {
            #[rustfmt::skip]
            let v = vec![
                0,
                1, 2,
                3, 4, 5,
                6, 7, 8, 9,
            ];
            let n = 5;
            let m = LoTriVec(n, v);

            assert_eq!(m.get_col_indices(0).collect::<Vec<_>>(), [0, 1, 3, 6]);
            assert_eq!(m.get_col_indices(1).collect::<Vec<_>>(), [2, 4, 7]);
            assert_eq!(m.get_col_indices(2).collect::<Vec<_>>(), [5, 8]);
            assert_eq!(m.get_col_indices(3).collect::<Vec<_>>(), [9]);
        }

        #[test]
        fn test_get_row() {
            #[rustfmt::skip]
            let v = vec![
                0,
                1, 2,
                3, 4, 5,
                6, 7, 8, 9,
            ];
            let n = 5;
            let m = LoTriVec(n, v);

            assert_eq!(m.get_row(1).cloned().collect::<Vec<_>>(), [0]);
            assert_eq!(m.get_row(2).cloned().collect::<Vec<_>>(), [1, 2]);
            assert_eq!(m.get_row(3).cloned().collect::<Vec<_>>(), [3, 4, 5]);
            assert_eq!(m.get_row(4).cloned().collect::<Vec<_>>(), [6, 7, 8, 9]);
        }

        #[test]
        fn test_get_col() {
            #[rustfmt::skip]
            let v = vec![
                0,
                1, 2,
                3, 4, 5,
                6, 7, 8, 9,
            ];
            let n = 5;
            let m = LoTriVec(n, v);

            assert_eq!(m.get_col(0).cloned().collect::<Vec<_>>(), [0, 1, 3, 6]);
            assert_eq!(m.get_col(1).cloned().collect::<Vec<_>>(), [2, 4, 7]);
            assert_eq!(m.get_col(2).cloned().collect::<Vec<_>>(), [5, 8]);
            assert_eq!(m.get_col(3).cloned().collect::<Vec<_>>(), [9]);
        }

        #[test]
        fn test_iter_triangle_indices() {
            let n = 5;
            let m = LoTriVec(n, Vec::new());

            #[rustfmt::skip]
            assert_eq!(m.iter_triangle_indices().collect::<Vec<_>>(), [
                (1, 0),
                (2, 0), (2, 1),
                (3, 0), (3, 1), (3, 2),
                (4, 0), (4, 1), (4, 2), (4, 3)
            ]);
        }
    }
}
