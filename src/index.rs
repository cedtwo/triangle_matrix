//! Triangle matrix indexing operations.

use std::ops::{Deref, Index, IndexMut};

use crate::{tri_num, LowerTriangle, TriangleType, UpperTriangle};

/// Triangle matrix indexing.
///
/// The indices `i` and `j` refer to the rows and columns respectively. All methods
/// either return an element, or the one dimensional `usize` index of the inner
/// collection.
pub trait TriangleIndex<Ty>: TriangleType<Ty>
where
    Self::Target: Index<usize> + IndexMut<usize>,
{
    /// Get the inner index for the element at the `i`, `j` indices.
    fn get_element_index(&self, i: usize, j: usize) -> usize;

    /// Get a reference to the element at the `i`, `j` indices.
    fn get_element<'a>(
        &'a self,
        i: usize,
        j: usize,
    ) -> &'a <<Self as Deref>::Target as Index<usize>>::Output {
        let index = self.get_element_index(i, j);
        &self[index]
    }

    /// Get a mutable reference to the element at the `i`, `j` indices.
    fn get_element_mut<'a>(
        &'a mut self,
        i: usize,
        j: usize,
    ) -> &'a mut <<Self as Deref>::Target as Index<usize>>::Output {
        let index = self.get_element_index(i, j);
        &mut self[index]
    }

    /// Get the inner index for the first element of row, `i`.
    fn get_row_start_index(&self, i: usize) -> usize;

    /// Get the inner index for the first element of column, `j`.
    fn get_col_start_index(&self, j: usize) -> usize;

    /// Get the inner indices for elements of the row, `i`.
    fn get_row_indices(&self, i: usize) -> impl Iterator<Item = usize> {
        self.get_row_start_index(i)..self.get_row_start_index(i + 1)
    }

    /// Get the inner indices for elements of the column, `j`.
    fn get_col_indices(&self, i: usize) -> impl Iterator<Item = usize>;
}

/// Upper triangle implementation.
impl<T: TriangleType<UpperTriangle>> TriangleIndex<UpperTriangle> for T
where
    Self::Target: Index<usize> + IndexMut<usize>,
{
    fn get_element_index(&self, i: usize, j: usize) -> usize {
        tri_num(self.n()) - tri_num(self.n() - i) + j
    }

    fn get_row_start_index(&self, i: usize) -> usize {
        tri_num(self.n()) - tri_num(self.n() - i)
    }

    fn get_col_start_index(&self, j: usize) -> usize {
        j
    }

    fn get_col_indices(&self, i: usize) -> impl Iterator<Item = usize> {
        (0..=i).map(move |row_index| self.get_row_start_index(row_index) + i - row_index)
    }
}

/// Lower triangle implementation.
impl<T: TriangleType<LowerTriangle>> TriangleIndex<LowerTriangle> for T
where
    Self::Target: Index<usize> + IndexMut<usize>,
{
    fn get_element_index(&self, i: usize, j: usize) -> usize {
        tri_num(i) + j
    }

    fn get_row_start_index(&self, i: usize) -> usize {
        tri_num(i)
    }

    fn get_col_start_index(&self, j: usize) -> usize {
        tri_num(j) + j
    }

    fn get_col_indices(&self, i: usize) -> impl Iterator<Item = usize> {
        (0..self.n() - i).map(move |row_index| self.get_row_start_index(row_index + i) + i)
    }
}

#[cfg(test)]
mod tests {

    use std::marker::PhantomData;
    use std::ops::DerefMut;

    use super::*;

    /// A triangle matrix containing the axis length, `n` and a `Vec` collection
    /// of `usize` elements.
    struct VecTri<Ty>(usize, Vec<usize>, PhantomData<Ty>);

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

    impl<Ty> TriangleType<Ty> for VecTri<Ty> {
        fn n(&self) -> usize {
            self.0
        }
    }

    mod upper_triangle {

        use super::*;

        type UpVecTri = VecTri<UpperTriangle>;

        #[test]
        fn test_get_element_index() {
            #[rustfmt::skip]
            let v = vec![
                0, 1, 2, 3,
                   4, 5, 6,
                      7, 8,
                         9,
            ];
            let n = 4;
            let m: UpVecTri = VecTri(n, v, PhantomData);

            assert_eq!(m.get_element_index(0, 0), 0);
            assert_eq!(m.get_element_index(0, 1), 1);
            assert_eq!(m.get_element_index(0, 2), 2);
            assert_eq!(m.get_element_index(0, 3), 3);
            assert_eq!(m.get_element_index(1, 0), 4);
            assert_eq!(m.get_element_index(1, 1), 5);
            assert_eq!(m.get_element_index(1, 2), 6);
            assert_eq!(m.get_element_index(2, 0), 7);
            assert_eq!(m.get_element_index(2, 1), 8);
            assert_eq!(m.get_element_index(3, 0), 9);
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
            let n = 4;
            let m: UpVecTri = VecTri(n, v, PhantomData);

            assert_eq!(*m.get_element(0, 0), 0);
            assert_eq!(*m.get_element(0, 1), 1);
            assert_eq!(*m.get_element(0, 2), 2);
            assert_eq!(*m.get_element(0, 3), 3);
            assert_eq!(*m.get_element(1, 0), 4);
            assert_eq!(*m.get_element(1, 1), 5);
            assert_eq!(*m.get_element(1, 2), 6);
            assert_eq!(*m.get_element(2, 0), 7);
            assert_eq!(*m.get_element(2, 1), 8);
            assert_eq!(*m.get_element(3, 0), 9);
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
            let n = 4;
            let mut m: UpVecTri = VecTri(n, v, PhantomData);
            *m.get_element_mut(1, 1) = 10;

            assert_eq!(*m.get_element(1, 1), 10);
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
            let n = 4;
            let m: UpVecTri = VecTri(n, v, PhantomData);

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
            let n = 4;
            let m: UpVecTri = VecTri(n, v, PhantomData);

            assert_eq!(m.get_col_start_index(0), 0);
            assert_eq!(m.get_col_start_index(1), 1);
            assert_eq!(m.get_col_start_index(2), 2);
            assert_eq!(m.get_col_start_index(3), 3);
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
            let n = 4;
            let m: UpVecTri = VecTri(n, v, PhantomData);

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
            let n = 4;
            let m: UpVecTri = VecTri(n, v, PhantomData);

            assert_eq!(m.get_col_indices(0).collect::<Vec<_>>(), [0]);
            assert_eq!(m.get_col_indices(1).collect::<Vec<_>>(), [1, 4]);
            assert_eq!(m.get_col_indices(2).collect::<Vec<_>>(), [2, 5, 7]);
            assert_eq!(m.get_col_indices(3).collect::<Vec<_>>(), [3, 6, 8, 9]);
        }
    }

    mod lower_triangle {

        use super::*;

        type LowVecTri = VecTri<LowerTriangle>;

        #[test]
        fn test_get_element_index() {
            #[rustfmt::skip]
            let v = vec![
                0,
                1, 2,
                3, 4, 5,
                6, 7, 8, 9,
            ];
            let n = 4;
            let m: LowVecTri = VecTri(n, v, PhantomData);

            assert_eq!(m.get_element_index(0, 0), 0);
            assert_eq!(m.get_element_index(1, 0), 1);
            assert_eq!(m.get_element_index(1, 1), 2);
            assert_eq!(m.get_element_index(2, 0), 3);
            assert_eq!(m.get_element_index(2, 1), 4);
            assert_eq!(m.get_element_index(2, 2), 5);
            assert_eq!(m.get_element_index(3, 0), 6);
            assert_eq!(m.get_element_index(3, 1), 7);
            assert_eq!(m.get_element_index(3, 2), 8);
            assert_eq!(m.get_element_index(3, 3), 9);
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
            let n = 4;
            let m: LowVecTri = VecTri(n, v, PhantomData);

            assert_eq!(*m.get_element(0, 0), 0);
            assert_eq!(*m.get_element(1, 0), 1);
            assert_eq!(*m.get_element(1, 1), 2);
            assert_eq!(*m.get_element(2, 0), 3);
            assert_eq!(*m.get_element(2, 1), 4);
            assert_eq!(*m.get_element(2, 2), 5);
            assert_eq!(*m.get_element(3, 0), 6);
            assert_eq!(*m.get_element(3, 1), 7);
            assert_eq!(*m.get_element(3, 2), 8);
            assert_eq!(*m.get_element(3, 3), 9);
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
            let n = 4;
            let mut m: LowVecTri = VecTri(n, v, PhantomData);
            *m.get_element_mut(1, 1) = 10;

            assert_eq!(*m.get_element(1, 1), 10);
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
            let n = 4;
            let m: LowVecTri = VecTri(n, v, PhantomData);

            assert_eq!(m.get_row_start_index(0), 0);
            assert_eq!(m.get_row_start_index(1), 1);
            assert_eq!(m.get_row_start_index(2), 3);
            assert_eq!(m.get_row_start_index(3), 6);
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
            let n = 4;
            let m: LowVecTri = VecTri(n, v, PhantomData);

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
            let n = 4;
            let m: LowVecTri = VecTri(n, v, PhantomData);

            assert_eq!(m.get_row_indices(0).collect::<Vec<_>>(), [0]);
            assert_eq!(m.get_row_indices(1).collect::<Vec<_>>(), [1, 2]);
            assert_eq!(m.get_row_indices(2).collect::<Vec<_>>(), [3, 4, 5]);
            assert_eq!(m.get_row_indices(3).collect::<Vec<_>>(), [6, 7, 8, 9]);
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
            let n = 4;
            let m: LowVecTri = VecTri(n, v, PhantomData);

            assert_eq!(m.get_col_indices(0).collect::<Vec<_>>(), [0, 1, 3, 6]);
            assert_eq!(m.get_col_indices(1).collect::<Vec<_>>(), [2, 4, 7]);
            assert_eq!(m.get_col_indices(2).collect::<Vec<_>>(), [5, 8]);
            assert_eq!(m.get_col_indices(3).collect::<Vec<_>>(), [9]);
        }
    }
}
