// Consider an algorithm that takes as input a positive integer n.
// If n is even, the algorithm divides it by two, and if n is odd, the algorithm multiplies it by three and adds one.
// The algorithm repeats this, until n is one. 
// For example, the sequence for n=3 is as follows:
//  3 -> 10 -> 5 -> 16 -> 8 -> 4 -> 2 -> 1
//
// Your task is to simulate the execution of the algorithm for a given value of n.
// 
// Input
// The only input line contains an integer n.
//
// Output
// Print a line that contains all values of n during the algorithm.
//
// Constraints
// 1 <= n <= 10^6
//

fn weird_algo(start: usize) -> Vec<usize> {
  let mut curr_num = start;
  let mut weird_coll: Vec<usize> = Vec::new();
  loop {
    print!("{} ", curr_num);
    weird_coll.push(curr_num);
    if curr_num == 1 {
      break;
    }		
    
    if (curr_num & 1) == 1 {
    curr_num = (curr_num * 3) + 1;
    } else {
      curr_num /= 2;
    }
  }
  weird_coll
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn check_for_3() {
    let input = 3;
    let solution = weird_algo(input);

    assert_eq!(solution.len(), 8);
    assert_eq!(solution, vec![3,10,5,16,8,4,2,1]);
  }
}
