//! Base upper triangle indexing operations.
use crate::tri_num;

/// Get the index of an element.
pub(crate) fn get_element_index(i: usize, j: usize, n: usize) -> usize {
    tri_num(n) - tri_num(n - i) + j
}

/// Get the first index of a row.
pub(crate) fn get_row_start_index(i: usize, n: usize) -> usize {
    tri_num(n) - tri_num(n - i)
}

/// Get the first index of a column.
pub(crate) fn get_col_start_index(j: usize) -> usize {
    j
}

/// Get all indices of a row.
pub(crate) fn get_row_indices(i: usize, n: usize) -> impl Iterator<Item = usize> {
    get_row_start_index(i, n)..get_row_start_index(i + 1, n)
}

/// Get all indices of a column.
pub(crate) fn get_col_indices(j: usize, n: usize) -> impl Iterator<Item = usize> {
    (0..=j).map(move |row_index| get_row_start_index(row_index, n) + j - row_index)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_get_element_index() {
        let n = 4;

        assert_eq!(get_element_index(0, 0, n), 0);
        assert_eq!(get_element_index(0, 1, n), 1);
        assert_eq!(get_element_index(0, 2, n), 2);
        assert_eq!(get_element_index(0, 3, n), 3);
        assert_eq!(get_element_index(1, 0, n), 4);
        assert_eq!(get_element_index(1, 1, n), 5);
        assert_eq!(get_element_index(1, 2, n), 6);
        assert_eq!(get_element_index(2, 0, n), 7);
        assert_eq!(get_element_index(2, 1, n), 8);
        assert_eq!(get_element_index(3, 0, n), 9);
    }

    #[test]
    fn test_get_row_start() {
        #[rustfmt::skip]
        let n = 4;

        assert_eq!(get_row_start_index(0, n), 0);
        assert_eq!(get_row_start_index(1, n), 4);
        assert_eq!(get_row_start_index(2, n), 7);
        assert_eq!(get_row_start_index(3, n), 9);
    }

    #[test]
    fn test_get_col_start() {
        assert_eq!(get_col_start_index(0), 0);
        assert_eq!(get_col_start_index(1), 1);
        assert_eq!(get_col_start_index(2), 2);
        assert_eq!(get_col_start_index(3), 3);
    }

    #[test]
    fn test_get_row_indices() {
        let n = 4;

        assert_eq!(get_row_indices(0, n).collect::<Vec<_>>(), [0, 1, 2, 3]);
        assert_eq!(get_row_indices(1, n).collect::<Vec<_>>(), [4, 5, 6]);
        assert_eq!(get_row_indices(2, n).collect::<Vec<_>>(), [7, 8]);
        assert_eq!(get_row_indices(3, n).collect::<Vec<_>>(), [9]);
    }

    #[test]
    fn test_get_col_indices() {
        #[rustfmt::skip]
        let n = 4;

        assert_eq!(get_col_indices(0, n).collect::<Vec<_>>(), [0]);
        assert_eq!(get_col_indices(1, n).collect::<Vec<_>>(), [1, 4]);
        assert_eq!(get_col_indices(2, n).collect::<Vec<_>>(), [2, 5, 7]);
        assert_eq!(get_col_indices(3, n).collect::<Vec<_>>(), [3, 6, 8, 9]);
    }
}
