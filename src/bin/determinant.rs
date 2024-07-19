fn calculate_determinant(matrix: &Vec<i32>, size: usize) -> i32 {
    if size == 1 {
        return matrix[0];
    }

    let mut det = 0;
    let mut sign = -1;

    for ind in 0..size {
        let mut child_matrix = Vec::new();

        for i in 1..size { // iterate over rows
            for j in 0..size { // iterate over cells in each row
                if j == ind {
                    continue;
                }

                child_matrix.push(matrix[i*size+j]);
            }
        }

        sign *= -1;
        det += sign * matrix[ind] * calculate_determinant(&child_matrix, size-1);
    }

    return det;
}

fn main() {
    let matrix = vec![6, 1, 1, 4, -2, 5, 2, 8, 7];

    let size = matrix.len();
    let size_sqrt = (size as f64).sqrt() as usize;

    if size_sqrt.pow(2) != size {
        panic!("Expected a square matrix");
    }

    let determinant = calculate_determinant(&matrix, size_sqrt);

    println!("Determinant of the provided matrix is: {determinant}");
}

