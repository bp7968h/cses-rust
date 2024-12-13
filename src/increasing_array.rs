// You are given an array of n integers. 
// You want to modify the array so that it is increasing, i.e., every element is at least as large as the previous element.
// On each move, you may increase the value of any element by one. What is the minimum number of moves required?
//
// Input
// The first input line contains an integer n: the size of the array.
// Then, the second line contains n integers x1,x2,...,xn: the contents of the array.
//
// Output
// Print the minimum number of moves.
//
// Constraints
// 1 <= n <= 2 * 10^5
// 1 <= xi <= 10^9
//

fn _min_num_moves(arr: &mut [usize]) -> usize {
    let mut move_count = 0;
    
    for i in 1..(arr.len() - 1) {
    	if arr[i] < arr[i-1] {
    		let diff = arr[i-1] - arr[i];
    		arr[i] += diff;
    		move_count += diff;
    	} 
    	if arr[i+1] < arr[i] {
    		let diff = arr[i] - arr[i+1];
    		arr[i+1] += diff;
    		move_count += diff;
    	}
    }   
    move_count
}

#[cfg(test)]
mod tests {
    use super::_min_num_moves;

    #[test]
    fn min_five_move_req() {
        let mut input_arr: Vec<usize> = vec![3, 2, 5, 1, 7];

        let moves = _min_num_moves(&mut input_arr);

        assert_eq!(vec![3, 3, 5, 5, 7], input_arr);
        assert_eq!(moves, 5);
    }
}