//! A simple upper triangle abstraction.
use std::ops::{Index, IndexMut};

use super::base;
use crate::{Triangle, TriangleMut};

/// A simple upper triangle collection.
///
/// Contains `tri_num(n)` elements with `n - 1` rows and columns to account for
/// the diagonal.
///
/// Any index outside of the upper triangle will cause a panic.
pub trait SimpleUpperTri: Triangle {
    /// Get a reference to an element.
    fn get_element<'a>(&'a self, i: usize, j: usize) -> &'a <Self::Inner as Index<usize>>::Output {
        debug_assert!(i <= self.n() - 1);
        debug_assert!(j <= self.n() - 1);

        assert!(j != 0);
        assert!(i < j);

        let index = base::get_element_index(i, j - (i + 1), self.n() - 1);
        &self.inner()[index]
    }

    /// Get an iterator of references to elements of a row.
    fn get_row<'a>(
        &'a self,
        i: usize,
    ) -> impl Iterator<Item = &'a <Self::Inner as Index<usize>>::Output> {
        SimpleUpperTri::get_row_indices(self, i).map(|el| &self.inner()[el])
    }

    /// Get an iterator of references to elements of a col.
    fn get_col<'a>(
        &'a self,
        i: usize,
    ) -> impl Iterator<Item = &'a <Self::Inner as Index<usize>>::Output> {
        SimpleUpperTri::get_col_indices(self, i).map(|el| &self.inner()[el])
    }

    /// Get the first index of a row.
    fn get_row_start_index(&self, i: usize) -> usize {
        debug_assert!(i <= self.n() - 1);

        base::get_row_start_index(i, self.n() - 1)
    }

    /// Get the first index of a column.
    fn get_col_start_index(&self, j: usize) -> usize {
        debug_assert!(j <= self.n() - 1);

        assert!(j != 0);

        base::get_col_start_index(j - 1)
    }

    /// Get all indices of a row.
    fn get_row_indices(&self, i: usize) -> impl Iterator<Item = usize> {
        debug_assert!(i <= self.n() - 1);

        base::get_row_indices(i, self.n() - 1)
    }

    /// Get all indices of a column.
    fn get_col_indices(&self, j: usize) -> impl Iterator<Item = usize> {
        debug_assert!(j <= self.n() - 1);

        assert!(j != 0);

        base::get_col_indices(j - 1, self.n() - 1)
    }
}

impl<T: Triangle> SimpleUpperTri for T {}

pub trait SimpleUpperTriMut: Triangle + TriangleMut
where
    Self::Inner: IndexMut<usize>,
{
    /// Get a mutable reference to an element.
    fn get_element_mut<'a>(
        &'a mut self,
        i: usize,
        j: usize,
    ) -> &'a mut <Self::Inner as Index<usize>>::Output {
        debug_assert!(i <= self.n() - 1);
        debug_assert!(j <= self.n() - 1);

        assert!(i < j);

        let index = base::get_element_index(i, j - (i + 1), self.n() - 1);
        &mut self.inner_mut()[index]
    }
}

impl<T: Triangle + TriangleMut> SimpleUpperTriMut for T where Self::Inner: IndexMut<usize> {}

#[cfg(test)]
mod tests {

    mod upper_triangle {

        use crate::{SimpleUpperTri, SimpleUpperTriMut};
        use crate::{Triangle, TriangleMut};

        struct UpTriVec(usize, Vec<usize>);

        impl Triangle for UpTriVec {
            type Inner = Vec<usize>;

            fn n(&self) -> usize {
                self.0
            }

            fn inner(&self) -> &Vec<usize> {
                &self.1
            }
        }

        impl TriangleMut for UpTriVec {
            fn inner_mut(&mut self) -> &mut Vec<usize> {
                &mut self.1
            }
        }

        #[test]
        fn test_get_element() {
            #[rustfmt::skip]
            let v = vec![
                0, 1, 2, 3,
                   4, 5, 6,
                      7, 8,
                         9,
            ];
            let n = 5;
            let m = UpTriVec(n, v);

            assert_eq!(*m.get_element(0, 1), 0);
            assert_eq!(*m.get_element(0, 2), 1);
            assert_eq!(*m.get_element(0, 3), 2);
            assert_eq!(*m.get_element(0, 4), 3);

            assert_eq!(*m.get_element(1, 2), 4);
            assert_eq!(*m.get_element(1, 3), 5);
            assert_eq!(*m.get_element(1, 4), 6);

            assert_eq!(*m.get_element(2, 3), 7);
            assert_eq!(*m.get_element(2, 4), 8);

            assert_eq!(*m.get_element(3, 4), 9);
        }

        #[test]
        fn test_get_element_mut() {
            #[rustfmt::skip]
            let v = vec![
                0, 1, 2, 3,
                   4, 5, 6,
                      7, 8,
                         9,
            ];
            let n = 5;
            let mut m = UpTriVec(n, v);
            *m.get_element_mut(1, 2) = 10;
            *m.get_element_mut(1, 3) = 11;

            assert_eq!(*m.get_element(1, 2), 10);
            assert_eq!(*m.get_element(1, 3), 11);
        }

        #[test]
        fn test_get_row_start() {
            #[rustfmt::skip]
            let v = vec![
                0, 1, 2, 3,
                   4, 5, 6,
                      7, 8,
                         9,
            ];
            let n = 5;
            let m = UpTriVec(n, v);

            assert_eq!(m.get_row_start_index(0), 0);
            assert_eq!(m.get_row_start_index(1), 4);
            assert_eq!(m.get_row_start_index(2), 7);
            assert_eq!(m.get_row_start_index(3), 9);
        }

        #[test]
        fn test_get_col_start() {
            #[rustfmt::skip]
            let v = vec![
                0, 1, 2, 3,
                   4, 5, 6,
                      7, 8,
                         9,
            ];
            let n = 5;
            let m = UpTriVec(n, v);

            assert_eq!(m.get_col_start_index(1), 0);
            assert_eq!(m.get_col_start_index(2), 1);
            assert_eq!(m.get_col_start_index(3), 2);
            assert_eq!(m.get_col_start_index(4), 3);
        }

        #[test]
        fn test_get_row_indices() {
            #[rustfmt::skip]
            let v = vec![
                0, 1, 2, 3,
                   4, 5, 6,
                      7, 8,
                         9,
            ];
            let n = 5;
            let m = UpTriVec(n, v);

            assert_eq!(m.get_row_indices(0).collect::<Vec<_>>(), [0, 1, 2, 3]);
            assert_eq!(m.get_row_indices(1).collect::<Vec<_>>(), [4, 5, 6]);
            assert_eq!(m.get_row_indices(2).collect::<Vec<_>>(), [7, 8]);
            assert_eq!(m.get_row_indices(3).collect::<Vec<_>>(), [9]);
        }

        #[test]
        fn test_get_col_indices() {
            #[rustfmt::skip]
            let v = vec![
                0, 1, 2, 3,
                   4, 5, 6,
                      7, 8,
                         9,
            ];
            let n = 5;
            let m = UpTriVec(n, v);

            assert_eq!(m.get_col_indices(1).collect::<Vec<_>>(), [0]);
            assert_eq!(m.get_col_indices(2).collect::<Vec<_>>(), [1, 4]);
            assert_eq!(m.get_col_indices(3).collect::<Vec<_>>(), [2, 5, 7]);
            assert_eq!(m.get_col_indices(4).collect::<Vec<_>>(), [3, 6, 8, 9]);
        }

        #[test]
        fn test_get_row() {
            #[rustfmt::skip]
            let v = vec![
                0, 1, 2, 3,
                   4, 5, 6,
                      7, 8,
                         9,
            ];
            let n = 5;
            let m = UpTriVec(n, v);

            assert_eq!(m.get_row(0).cloned().collect::<Vec<_>>(), [0, 1, 2, 3]);
            assert_eq!(m.get_row(1).cloned().collect::<Vec<_>>(), [4, 5, 6]);
            assert_eq!(m.get_row(2).cloned().collect::<Vec<_>>(), [7, 8]);
            assert_eq!(m.get_row(3).cloned().collect::<Vec<_>>(), [9]);
        }

        #[test]
        fn test_get_col() {
            #[rustfmt::skip]
            let v = vec![
                0, 1, 2, 3,
                   4, 5, 6,
                      7, 8,
                         9,
            ];
            let n = 5;
            let m = UpTriVec(n, v);

            assert_eq!(m.get_col(1).cloned().collect::<Vec<_>>(), [0]);
            assert_eq!(m.get_col(2).cloned().collect::<Vec<_>>(), [1, 4]);
            assert_eq!(m.get_col(3).cloned().collect::<Vec<_>>(), [2, 5, 7]);
            assert_eq!(m.get_col(4).cloned().collect::<Vec<_>>(), [3, 6, 8, 9]);
        }
    }
}
