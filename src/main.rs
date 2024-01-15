fn snail(matrix: &[Vec<i32>]) -> Vec<i32> {
    if matrix.len() == 0 {
        return Vec::new();
    }

    // get grid dimensions
    let mut columns = matrix[0].len(); // x
    let mut rows = matrix.len(); // y
    let mut origin = 0;

    let target_length = columns * rows;
    let mut final_arr = Vec::new();

    let mut attempt = 0;
    while final_arr.len() < target_length {
        if attempt > 0 {
            columns = columns - 1;
            rows = rows - 1;
            origin = origin + 1;
        }

        // Top bar
        for x in origin..columns {
            let point = matrix[origin][x];
            final_arr.push(point);
        }

        // Right side bar
        for y in origin + 1..rows {
            let point = matrix[y][columns - 1];
            final_arr.push(point);
        }

        // Bottom bar
        for x in (origin..columns - 1).rev() {
            let point = matrix[rows - 1][x];
            final_arr.push(point);
        }

        // Left bar
        for y in (origin + 1..columns - 1).rev() {
            let point = matrix[y][origin];
            final_arr.push(point);
        }

        attempt = attempt + 1;
    }

    final_arr
}

fn main() {
    // let array = [vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]];
    let array = [
        vec![1, 2, 3, 4, 5],
        vec![6, 7, 8, 9, 10],
        vec![11, 12, 13, 14, 15],
        vec![16, 17, 18, 19, 20],
        vec![21, 22, 23, 24, 25],
    ];
    dbg!(snail(&array));
}
