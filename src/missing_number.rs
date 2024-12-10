// You are given all numbers between 1,2,…,n1,2,…,n except one. Your task is to find the missing number.
//
// Input
// The first input line contains an integer n.
// The second line contains n-1 numbers. Each number is distinct and between 1 and n (inclusive).
// 
// Output
// Print the missing number.
//
// Constraints
// 2 <= n <= 2 * 10^5
//

fn missing_number(integer: usize, numbers: &[usize]) -> usize {
  let mut actual_sum: usize = (1..=integer).fold(0, |acc, n| acc ^ n);
  numbers.iter().for_each(|n| actual_sum ^= n );

  actual_sum
}

#[cfg(test)]
mod tests {
  use super::missing_number;

  #[test]
  fn missing_four() {
    let max_num: usize = 5;
    let numbers: [usize; 4] = [2,3,1,5];
    
    let missing_num = missing_number(max_num, &numbers);
    assert_eq!(missing_num, 4);
  }
}
