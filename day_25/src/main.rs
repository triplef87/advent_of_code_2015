fn main() {
    let mut matrix: Vec<Vec<usize>> = vec![vec![0; 6062];6062];
    matrix[0][0] = 20151125;

    let mut row = 0;
    let mut col = 0;
    loop {
        let prev = matrix[row][col];

        if row == 0 {
            row = col + 1;
            col = 0;
        } else {
            row = row - 1;
            col = col + 1;
        }

        matrix[row][col] = (prev * 252533) % 33554393;
        if row == 2977 && col == 3082 {
            println!("{}", matrix[row][col]);
            break;
        }
    }

}
