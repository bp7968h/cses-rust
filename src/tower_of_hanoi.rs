// The Tower of Hanoi game consists of three stacks (left, middle and right) 
// and n round disks of different sizes. 
// Initially, the left stack has all the disks, in increasing order of size from top to bottom.
// 
// The goal is to move all the disks to the right stack using the middle stack. 
// On each move you can move the uppermost disk from a stack to another stack. 
// In addition, it is not allowed to place a larger disk on a smaller disk.
//
// Your task is to find a solution that minimizes the number of moves.
//
// Input
// The only input line has an integer n: the number of disks.
//
// Output
// First print an integer k: the minimum number of moves.
// After this, print k lines that describe the moves.
// Each line has two integers a and b: you move a disk from stack a to stack b.
//
// Constraints
// 1 <= n <= 16
//

fn _toh_moves(disk: u8, left: &mut Vec<u8>, middle: &mut Vec<u8>, right: &mut Vec<u8>, src: u8, aux: u8, dst: u8, moves: &mut Vec<(u8, u8)>) {
    match disk {
        1 => {
            if let Some(d) = left.pop() {
                right.push(d);
                moves.push((src, dst));
            }
            return;
        },
        _ => {
            _toh_moves(disk - 1, left, right, middle, src, dst, aux, moves);
            if let Some(d) = left.pop() {
                right.push(d);
                moves.push((src, dst));
            }
            _toh_moves(disk - 1, middle, left, right, aux, src, dst, moves);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::_toh_moves;
    
    #[test]
    fn moves_for_one_disk() {
        let num_disk: u8 = 1;
        let mut stack_1: Vec<u8> = (1..=num_disk).rev().collect();
        let mut stack_2: Vec<u8> = Vec::new();
        let mut stack_3: Vec<u8> = Vec::new();
        let mut moves: Vec<(u8, u8)> = Vec::new();
        
        _toh_moves(num_disk, &mut stack_1, &mut stack_2, &mut stack_3, 1, 2, 3, &mut moves);
        
        assert_eq!(1, moves.len());
        assert_eq!(moves, vec![(1,3)]);
    }
    
    #[test]
    fn moves_for_two_disk() {
        let num_disk: u8 = 2;
        let mut stack_1: Vec<u8> = (1..=num_disk).rev().collect();
        let mut stack_2: Vec<u8> = Vec::new();
        let mut stack_3: Vec<u8> = Vec::new();
        let mut moves: Vec<(u8, u8)> = Vec::new();
        
        _toh_moves(num_disk, &mut stack_1, &mut stack_2, &mut stack_3, 1, 2, 3, &mut moves);
        
        assert_eq!(3, moves.len());
        assert_eq!(moves, vec![(1,2), (1,3), (2,3)]);
    }
}
