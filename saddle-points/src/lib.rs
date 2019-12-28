// It's called a "saddle point" because it is greater than or equal to every element
// in its row and less than or equal to every element in its column.
pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut saddle_points = Vec::new();
    for row in 0..input.len() {
        for col in 0..input[row].len() {
            let max_in_row = input[row].iter().max().unwrap();
            let min_in_col = input.iter().map(|v| v[col]).min().unwrap();
            if input[row][col] == *max_in_row && input[row][col] == min_in_col {
                saddle_points.push((row, col));
            }
        }
    }
    saddle_points
}
