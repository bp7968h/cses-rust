// Given a string, your task is to reorder its letters in such a way that it becomes a palindrome 
// (i.e., it reads the same forwards and backwards).
//
// Input
// The only input line has a string of length n consisting of characters A–Z.
//
// Output
// Print a palindrome consisting of the characters of the original string. 
// You may print any valid solution. If there are no solutions, print "NO SOLUTION".
// 
// Constraints
// 1 <= n <= 10^6
//
use std::collections::HashMap;

fn _becomes_palindrome(input: &str) -> String {
    let mut char_store: HashMap<char, usize> = HashMap::new();
    for c in input.chars() {
        char_store.entry(c).and_modify(|e| *e += 1).or_insert(1);
    }
    
    let mut odd_chars: String = String::new();
    let mut half_palindrome = String::new();
    for (&key, &value) in char_store.iter() {
        if (value & 1) == 1 {
            odd_chars.push(key);
        }
        
        for _ in 0..value/2 {
            half_palindrome.push(key);
        }
    }

    // there can only be one odd char in the string
    if odd_chars.len() > 1 {
        return String::from("NO SOLUTION");
    }
    
    format!("{}{}{}", half_palindrome, odd_chars, &half_palindrome.chars().rev().collect::<String>())
}

#[cfg(test)]
mod tests {
    use super::_becomes_palindrome;
    
    #[test]
    fn yes_AAAACACBA() {
        let input: &str = "AAAACACBA";
        let solutions: [&str; 2] = ["AAACBCAAA", "CAAABAAAC"];
        
        let output = _becomes_palindrome(input);
        
        assert_eq!(input.len(), output.len());
        assert!(solutions.contains(&output.as_str()));
    }
    
    #[test]
    fn no_solution_ABABABABAB() {
        let input: &str = "ABABABABAB";
        
        let output = _becomes_palindrome(input);
        
        assert_eq!(output.as_str(), "NO SOLUTION");
    }
}
