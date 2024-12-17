// You have two coin piles containing a and b coins. 
// On each move, you can either:
//     - remove one coin from the left pile and two coins from the right pile
//     - two coins from the left pile and one coin from the right pile.
// Your task is to efficiently find out if you can empty both the piles.
//
// Input
// The first input line has an integer t: the number of tests.
// After this, there are t lines, each of which has two integers a and b: the numbers of coins in the piles.
//
// Output
//For each test, print "YES" if you can empty the piles and "NO" otherwise.
//
// Constraints
// 1 <= t <= 10^5
// 0 <= a, b <= 10^9
//

fn _can_empty_piles(piles: &[(usize, usize)]) -> Vec<bool> {
    let mut output = Vec::with_capacity(piles.len());
    for &(l,r) in piles {
        if ((l + r) % 3) != 0 || l > 2 * r || r > 2 * l {
	        output.push(false);
	        continue;
	    }
	    output.push(true)
    }
    output
}

#[cfg(test)]
mod tests {
    use super::_can_empty_piles;
    
    #[test]
    fn yes_no_yes() {
        let input = vec![(2,1), (2,2), (3,3)];
        let output = _can_empty_piles(&input);
        
        assert_eq!(input.len(), output.len());
        assert_eq!(output, vec![true, false, true]);
    }
}
