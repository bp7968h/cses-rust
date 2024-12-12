// You are given a DNA sequence: a string consisting of characters A, C, G, and T. 
// Your task is to find the longest repetition in the sequence. 
//This is a maximum-length substring containing only one type of character.
//
// Input
// The only input line contains a string of n characters.
//
// Output
// Print one integer: the length of the longest repetition.
//
// Constraints
// 1 <= n <= 10^6
//

fn _longest_rep_seq(input: &str) -> usize {
	let input_chars: Vec<char> = input.chars().collect();
	let mut max_len = 0;
	let mut curr_len = 1;
	for i in 0..(input_chars.len() - 1) {
		if input_chars[i] == input_chars[i+1] {
			curr_len += 1;
		} else {
			curr_len = 1;
		}
        
        if curr_len > max_len {
			max_len = curr_len;
		}
	}
	max_len
}

#[cfg(test)]
mod tests {
  use super::_longest_rep_seq;

  #[test]
  fn longest_three_char() {
    let input = "ATTCGGGA";
    
    let max_len = _longest_rep_seq(input);
    assert_eq!(max_len, 3);
  }
}