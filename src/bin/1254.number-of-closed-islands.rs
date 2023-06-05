/*
 * @lc app=leetcode id=1254 lang=rust
 *
 * [1254] Number of Closed Islands
 */

// @lc code=start

impl Solution {
    pub fn closed_island(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut count = 0;
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if (grid[i][j] == 0) {
                    grid[i][j] = 1;
                    if !recSearch(i, j, !(i>0 && j>0 && i<grid.len()-1 && j <grid[i].len()-1), &mut grid) {
                        count += 1;
                    }
                }
            }
        }
        count
    }
}
pub fn recSearch(x: usize, y: usize, mut isEdge: bool, grid: &mut Vec<Vec<i32>>) -> bool {
    if (x + 1 < grid.len() && grid[x + 1][y] == 0) {
        grid[x + 1][y] = 1;
        isEdge |= recSearch(x + 1, y, isEdge, grid);
    }
    if (x != 0 && grid[x - 1][y] == 0) {
        grid[x - 1][y] = 1;
        isEdge |= recSearch(x - 1, y, isEdge, grid);
    }
    if (y + 1 < grid[x].len() && grid[x][y + 1] == 0) {
        grid[x][y + 1] = 1;
        isEdge |= recSearch(x, y + 1, isEdge, grid);
    }
    if (y != 0 && grid[x][y - 1] == 0) {
        grid[x][y - 1] = 1;
        isEdge |= recSearch(x, y - 1, isEdge, grid);
    }

    if (x + 1 >= grid.len() || x == 0 || y + 1 >= grid[x].len() || y == 0) {
        isEdge |= true;
    }
    return isEdge;
}
// @lc code=end
