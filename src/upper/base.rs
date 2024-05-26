//! Base upper triangle indexing operations.
use crate::{tri_num, Triangle};

/// Base trait for upper triangle indices. All upper triangle indexing operations
/// will be an offset of a value calculated here.
pub trait BaseUpperIndex: Triangle {
    /// Get the index of an element.
    fn get_element_index(&self, i: usize, j: usize, n: usize) -> usize {
        tri_num(n) - tri_num(n - i) + j
    }

    /// Get the first index of a row.
    fn get_row_start_index(&self, i: usize, n: usize) -> usize {
        tri_num(n) - tri_num(n - i)
    }

    /// Get the first index of a column.
    fn get_col_start_index(&self, j: usize, _: usize) -> usize {
        j
    }

    /// Get all indices of a row.
    fn get_row_indices(&self, i: usize, n: usize) -> impl Iterator<Item = usize> {
        self.get_row_start_index(i, n)..self.get_row_start_index(i + 1, n)
    }

    /// Get all indices of a column.
    fn get_col_indices(&self, j: usize, n: usize) -> impl Iterator<Item = usize> {
        (0..=j).map(move |row_index| self.get_row_start_index(row_index, n) + j - row_index)
    }
}

impl<T: Triangle> BaseUpperIndex for T {}

#[cfg(test)]
mod tests {

    use super::*;

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
        let m = UpTriVec(n, v);

        assert_eq!(m.get_element_index(0, 0, n), 0);
        assert_eq!(m.get_element_index(0, 1, n), 1);
        assert_eq!(m.get_element_index(0, 2, n), 2);
        assert_eq!(m.get_element_index(0, 3, n), 3);
        assert_eq!(m.get_element_index(1, 0, n), 4);
        assert_eq!(m.get_element_index(1, 1, n), 5);
        assert_eq!(m.get_element_index(1, 2, n), 6);
        assert_eq!(m.get_element_index(2, 0, n), 7);
        assert_eq!(m.get_element_index(2, 1, n), 8);
        assert_eq!(m.get_element_index(3, 0, n), 9);
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
        let m = UpTriVec(n, v);

        assert_eq!(m.get_row_start_index(0, n), 0);
        assert_eq!(m.get_row_start_index(1, n), 4);
        assert_eq!(m.get_row_start_index(2, n), 7);
        assert_eq!(m.get_row_start_index(3, n), 9);
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
        let m = UpTriVec(n, v);

        assert_eq!(m.get_col_start_index(0, n), 0);
        assert_eq!(m.get_col_start_index(1, n), 1);
        assert_eq!(m.get_col_start_index(2, n), 2);
        assert_eq!(m.get_col_start_index(3, n), 3);
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
        let m = UpTriVec(n, v);

        assert_eq!(m.get_row_indices(0, n).collect::<Vec<_>>(), [0, 1, 2, 3]);
        assert_eq!(m.get_row_indices(1, n).collect::<Vec<_>>(), [4, 5, 6]);
        assert_eq!(m.get_row_indices(2, n).collect::<Vec<_>>(), [7, 8]);
        assert_eq!(m.get_row_indices(3, n).collect::<Vec<_>>(), [9]);
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
        let m = UpTriVec(n, v);

        assert_eq!(m.get_col_indices(0, n).collect::<Vec<_>>(), [0]);
        assert_eq!(m.get_col_indices(1, n).collect::<Vec<_>>(), [1, 4]);
        assert_eq!(m.get_col_indices(2, n).collect::<Vec<_>>(), [2, 5, 7]);
        assert_eq!(m.get_col_indices(3, n).collect::<Vec<_>>(), [3, 6, 8, 9]);
    }
}
