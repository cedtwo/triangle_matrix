//! Base lower triangle indexing operations.
use crate::ops::tri_num;

/// Get the index of an element.
pub fn get_element_index(i: usize, j: usize) -> usize {
    tri_num(i) + j
}

/// Get the first index of a row.
pub fn get_row_start_index(i: usize) -> usize {
    tri_num(i)
}

/// Get the first index of a column.
pub fn get_col_start_index(j: usize) -> usize {
    tri_num(j) + j
}

/// Get all indices of a row.
pub fn get_row_indices(i: usize) -> impl Iterator<Item = usize> {
    get_row_start_index(i)..get_row_start_index(i + 1)
}

/// Get all indices of a column.
pub fn get_col_indices(j: usize, n: usize) -> impl Iterator<Item = usize> {
    (0..n - j).map(move |row_index| get_row_start_index(row_index + j) + j)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_get_element_index() {
        assert_eq!(get_element_index(0, 0), 0);
        assert_eq!(get_element_index(1, 0), 1);
        assert_eq!(get_element_index(1, 1), 2);
        assert_eq!(get_element_index(2, 0), 3);
        assert_eq!(get_element_index(2, 1), 4);
        assert_eq!(get_element_index(2, 2), 5);
        assert_eq!(get_element_index(3, 0), 6);
        assert_eq!(get_element_index(3, 1), 7);
        assert_eq!(get_element_index(3, 2), 8);
        assert_eq!(get_element_index(3, 3), 9);
    }

    #[test]
    fn test_get_row_start() {
        assert_eq!(get_row_start_index(0), 0);
        assert_eq!(get_row_start_index(1), 1);
        assert_eq!(get_row_start_index(2), 3);
        assert_eq!(get_row_start_index(3), 6);
    }

    #[test]
    fn test_get_col_start() {
        assert_eq!(get_col_start_index(0), 0);
        assert_eq!(get_col_start_index(1), 2);
        assert_eq!(get_col_start_index(2), 5);
        assert_eq!(get_col_start_index(3), 9);
    }

    #[test]
    fn test_get_row_indices() {
        assert_eq!(get_row_indices(0).collect::<Vec<_>>(), [0]);
        assert_eq!(get_row_indices(1).collect::<Vec<_>>(), [1, 2]);
        assert_eq!(get_row_indices(2).collect::<Vec<_>>(), [3, 4, 5]);
        assert_eq!(get_row_indices(3).collect::<Vec<_>>(), [6, 7, 8, 9]);
    }

    #[test]
    fn test_get_col_indices() {
        let n = 4;

        assert_eq!(get_col_indices(0, n).collect::<Vec<_>>(), [0, 1, 3, 6]);
        assert_eq!(get_col_indices(1, n).collect::<Vec<_>>(), [2, 4, 7]);
        assert_eq!(get_col_indices(2, n).collect::<Vec<_>>(), [5, 8]);
        assert_eq!(get_col_indices(3, n).collect::<Vec<_>>(), [9]);
    }
}
