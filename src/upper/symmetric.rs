//! A symmetric upper triangle matrix abstraction.
use std::ops::{Index, IndexMut};

use super::base;
use crate::{Triangle, TriangleMut};

/// A symmetric upper triangle collection.
///
/// Contains `tri_num(n)` elements with `n - 1` rows and columns to account for
/// the diagonal.
///
/// For all indices `i` and `j` where `i != j`, all pairs of `(i, j)` are equal to
/// the pair `(j, i)`.
pub trait SymmetricUpperTri: Triangle {
    /// Get a reference to an element.
    fn get_element<'a>(&'a self, i: usize, j: usize) -> &'a <Self::Inner as Index<usize>>::Output {
        debug_assert!(i <= self.n() - 1);
        debug_assert!(j <= self.n() - 1);

        let index = if i < j {
            base::get_element_index(i, j - (i + 1), self.n() - 1)
        } else {
            base::get_element_index(j, i - (j + 1), self.n() - 1)
        };

        &self.inner()[index]
    }

    /// Get an iterator of references to elements of a row.
    fn get_row<'a>(
        &'a self,
        i: usize,
    ) -> impl Iterator<Item = &'a <Self::Inner as Index<usize>>::Output> {
        SymmetricUpperTri::get_row_indices(self, i).map(|el| &self.inner()[el])
    }

    /// Get an iterator of references to elements of a col.
    fn get_col<'a>(
        &'a self,
        i: usize,
    ) -> impl Iterator<Item = &'a <Self::Inner as Index<usize>>::Output> {
        SymmetricUpperTri::get_col_indices(self, i).map(|el| &self.inner()[el])
    }

    /// Get all indices of a row.
    fn get_row_indices(&self, i: usize) -> Box<dyn Iterator<Item = usize> + '_> {
        debug_assert!(i <= self.n() - 1);

        if i == 0 {
            Box::new(base::get_row_indices(i, self.n() - 1))
        } else if i == self.n() - 1 {
            Box::new(base::get_col_indices(i - 1, self.n() - 1))
        } else {
            Box::new(
                base::get_col_indices(i - 1, self.n() - 1)
                    .chain(base::get_row_indices(i, self.n() - 1)),
            )
        }
    }

    /// Get all indices of a column.
    fn get_col_indices(&self, j: usize) -> impl Iterator<Item = usize> {
        SymmetricUpperTri::get_row_indices(self, j)
    }
}

impl<T: Triangle> SymmetricUpperTri for T {}

pub trait SymmetricUpperTriMut: Triangle + TriangleMut
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

        let index = if i < j {
            base::get_element_index(i, j - (i + 1), self.n() - 1)
        } else {
            base::get_element_index(j, i - (j + 1), self.n() - 1)
        };

        &mut self.inner_mut()[index]
    }
}

impl<T: Triangle + TriangleMut> SymmetricUpperTriMut for T where Self::Inner: IndexMut<usize> {}

#[cfg(test)]
mod tests {

    use super::{SymmetricUpperTri, SymmetricUpperTriMut};
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
        assert_eq!(*m.get_element(1, 0), 0);
        assert_eq!(*m.get_element(1, 2), 4);
        assert_eq!(*m.get_element(1, 3), 5);
        assert_eq!(*m.get_element(1, 4), 6);
        assert_eq!(*m.get_element(2, 0), 1);
        assert_eq!(*m.get_element(2, 1), 4);
        assert_eq!(*m.get_element(2, 3), 7);
        assert_eq!(*m.get_element(2, 4), 8);
        assert_eq!(*m.get_element(3, 0), 2);
        assert_eq!(*m.get_element(3, 1), 5);
        assert_eq!(*m.get_element(3, 2), 7);
        assert_eq!(*m.get_element(3, 4), 9);
        assert_eq!(*m.get_element(4, 0), 3);
        assert_eq!(*m.get_element(4, 1), 6);
        assert_eq!(*m.get_element(4, 2), 8);
        assert_eq!(*m.get_element(4, 3), 9);
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
        *m.get_element_mut(3, 1) = 10;
        *m.get_element_mut(2, 3) = 11;

        assert_eq!(*m.get_element(1, 3), 10);
        assert_eq!(*m.get_element(3, 2), 11);
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
        assert_eq!(m.get_row_indices(1).collect::<Vec<_>>(), [0, 4, 5, 6]);
        assert_eq!(m.get_row_indices(2).collect::<Vec<_>>(), [1, 4, 7, 8]);
        assert_eq!(m.get_row_indices(3).collect::<Vec<_>>(), [2, 5, 7, 9]);
        assert_eq!(m.get_row_indices(4).collect::<Vec<_>>(), [3, 6, 8, 9]);
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

        assert_eq!(m.get_col_indices(0).collect::<Vec<_>>(), [0, 1, 2, 3]);
        assert_eq!(m.get_col_indices(1).collect::<Vec<_>>(), [0, 4, 5, 6]);
        assert_eq!(m.get_col_indices(2).collect::<Vec<_>>(), [1, 4, 7, 8]);
        assert_eq!(m.get_col_indices(3).collect::<Vec<_>>(), [2, 5, 7, 9]);
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
        assert_eq!(m.get_row(1).cloned().collect::<Vec<_>>(), [0, 4, 5, 6]);
        assert_eq!(m.get_row(2).cloned().collect::<Vec<_>>(), [1, 4, 7, 8]);
        assert_eq!(m.get_row(3).cloned().collect::<Vec<_>>(), [2, 5, 7, 9]);
        assert_eq!(m.get_row(4).cloned().collect::<Vec<_>>(), [3, 6, 8, 9]);
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

        assert_eq!(m.get_col(0).cloned().collect::<Vec<_>>(), [0, 1, 2, 3]);
        assert_eq!(m.get_col(1).cloned().collect::<Vec<_>>(), [0, 4, 5, 6]);
        assert_eq!(m.get_col(2).cloned().collect::<Vec<_>>(), [1, 4, 7, 8]);
        assert_eq!(m.get_col(3).cloned().collect::<Vec<_>>(), [2, 5, 7, 9]);
        assert_eq!(m.get_col(4).cloned().collect::<Vec<_>>(), [3, 6, 8, 9]);
    }
}
