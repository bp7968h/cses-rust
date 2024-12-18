// A Gray code is a list of all 2^n bit strings of length n, 
// where any two successive strings differ in exactly one bit 
// (i.e., their Hamming distance is one).
// Your task is to create a Gray code for a given length n.
//
// Input
// The only input line has an integer n.
//
// Output
// Print 2^n lines that describe the Gray code. You can print any valid solution.
//
// Constraints
// 1 <= n <= 16
//

fn _generate_gray_code(str_len: &usize) -> Vec<String> {
    let mut gray_codes = vec!["0".to_owned(), "1".to_owned()];

    for _ in 1..*str_len {
        let mut new_codes: Vec<String> = Vec::new();

        for code in &gray_codes {
            new_codes.push(format!("0{}", code));
        }

        for code in gray_codes.iter().rev() {
            new_codes.push(format!("1{}", code));
        }
        gray_codes = new_codes;
    }
    gray_codes
}

#[cfg(test)]
mod tests {
    use super::_generate_gray_code;
    
    #[test]
    fn for_2() {
        let codes = _generate_gray_code(&2);
        
        assert_eq!(codes.len(), 4);
        assert_eq!(vec!["00", "01", "11", "10"], codes);
    }
    
    #[test]
    fn for_3() {
        let codes = _generate_gray_code(&3);
        
        assert_eq!(codes.len(), 8);
        assert_eq!(vec!["000", "001", "011", "010", "110", "111", "101", "100"], codes);
    }
}
