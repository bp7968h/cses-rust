// You are given a list of n integers, and your task is to calculate the number of distinct values in the list.
//
// Input
// The first input line has an integer n: the number of values.
// The second line has n integers x_1,x_2,...,x_n.
//
// Output
// Print one integers: the number of distinct values.
//
// Constraints 
// 1 <= n <= 2 * 10^5
// 1 <= x_i <= 10^9
//

use std::collections::HashSet;

fn _find_distinct_numbers(input: &[usize]) -> usize {
    let mut set = HashSet::new();

    for item in input {
        let _ = set.insert(item);
    }

    set.len()
}

#[cfg(test)]
mod tests {
    use super::_find_distinct_numbers;

    #[test]
    fn one_distinct_numbers() {
        let input = vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1];

        let d_numbers = _find_distinct_numbers(&input);

        assert_eq!(d_numbers, 1);
    }

    #[test]
    fn two_distinct_numbers() {
        let input = vec![2,3,2,2,3];

        let d_numbers = _find_distinct_numbers(&input);

        assert_eq!(d_numbers, 2);
    }

    #[test]
    fn ten_distinct_numbers() {
        let input = vec![7, 4, 10, 9, 6, 1, 8, 2, 5, 3];

        let d_numbers = _find_distinct_numbers(&input);

        assert_eq!(d_numbers, 10);
    }
}