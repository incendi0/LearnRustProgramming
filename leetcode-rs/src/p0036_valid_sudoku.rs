struct Solution {}

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        const LEN: usize = 9;
        let mut row = [[0; LEN]; LEN];
        let mut col = [[0; LEN]; LEN];
        let mut blk = [[[0; LEN]; LEN / 3]; LEN / 3];
        for i in 0..9 {
            for j in 0..9 {
                if board[i][j] != '.' {
                    let d = board[i][j] as usize - '1' as usize;
                    row[i][d] += 1;
                    col[j][d] += 1;
                    blk[i / 3][j / 3][d] += 1;
                    if row[i][d] > 1 || col[j][d] > 1 || blk[i / 3][j / 3][d] > 1 {
                        return false;
                    }
                }
            }
        }
        true
    }
}
