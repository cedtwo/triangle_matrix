//! Triangle matrix operations.

/// Calculate the triangle number for `n`.
pub fn tri_num(n: usize) -> usize {
    (n * (n + 1)) / 2
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_tri_num() {
        // Sum all numbers from `0` to `n`
        let acc_num = |n: usize| (0..=n).reduce(|a, b| a + b).unwrap();

        assert_eq!(tri_num(1), acc_num(1));
        assert_eq!(tri_num(2), acc_num(2));
        assert_eq!(tri_num(3), acc_num(3));
        assert_eq!(tri_num(4), acc_num(4));
        assert_eq!(tri_num(5), acc_num(5));
    }
}
