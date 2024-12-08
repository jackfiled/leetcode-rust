/**
 * [782] Transform to Chessboard
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn moves_to_chessboard(board: Vec<Vec<i32>>) -> i32 {
        let n = board.len();

        let (mut row_mask, mut col_mask) = (0, 0);

        for i in 0..n {
            row_mask |= board[0][i] << i;
            col_mask |= board[i][0] << i;
        }

        let reverse_row_mask = ((1 << n) - 1) ^ row_mask;
        let reverse_col_mask = ((1 << n) - 1) ^ col_mask;

        let (mut row_count, mut col_count) = (0, 0);

        for i in 0..n {
            let (mut current_row_mask, mut current_col_mask) = (0, 0);

            for j in 0..n {
                current_row_mask |= board[i][j] << j;
                current_col_mask |= board[j][i] << j;
            }

            if current_row_mask != row_mask && current_row_mask != reverse_row_mask {
                return -1;
            }
            if current_row_mask == row_mask {
                row_count += 1;
            }

            if current_col_mask != col_mask && current_col_mask != reverse_col_mask {
                return -1;
            }
            if current_col_mask == col_mask {
                col_count += 1;
            }
        }

        let row_moves = Self::get_moves(row_mask, row_count, n as i32);
        let col_moves = Self::get_moves(col_mask, col_count, n as i32);
        if row_moves == -1 || col_moves == -1 {
            -1
        } else {
            row_moves + col_moves
        }
    }

    fn get_moves(mask: i32, count: i32, n: i32) -> i32 {
        let one_count = mask.count_ones() as i32;

        if n % 2 == 0 {
            if one_count != n >> 1 || count != n >> 1 {
                return -1;
            }

            let count0 = n / 2 - (mask & 0xAAAAAAAAu32 as i32).count_ones() as i32;
            let count1 = n / 2 - (mask & 0x55555555).count_ones() as i32;

            count0.min(count1)
        } else {
            if n.abs_diff(2 * one_count) != 1 || n.abs_diff(2 * count) != 1 {
                return -1;
            }

            if one_count == n / 2 {
                n / 2 - (mask & 0xAAAAAAAAu32 as i32).count_ones() as i32
            } else {
                (n + 1) / 2 - (mask & 0x55555555).count_ones() as i32
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_782() {
        assert_eq!(
            2,
            Solution::moves_to_chessboard(vec![
                vec![0, 1, 1, 0],
                vec![0, 1, 1, 0],
                vec![1, 0, 0, 1],
                vec![1, 0, 0, 1]
            ])
        );
    }
}
