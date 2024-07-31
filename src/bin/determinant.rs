use std::{env, fs};

fn calculate_determinant(matrix: &Vec<i64>, size: usize) -> i64 {
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
    let mut argv_iter = env::args();

    // ignore first argv (path to binary)
    argv_iter.next();

    let file_name = if let Some(value) = argv_iter.next() {
        value
    } else {
        "matrix.txt".to_string()
    };

    let matrix = fs::read_to_string(&file_name)
            .expect(format!("Error while reading file '{file_name}'.\n\
                            Create a new matrix using command: \
                            'cargo run --bin create --out {file_name}'\n\
                            Original error").as_str());

    // convert matrix to i64 vector
    let matrix: Vec<i64> = matrix
        .replace("\n", " ")
        .split(' ')
        .filter(|s| *s != "")
        .map(|s| s.parse().expect(format!("Failed to parse to i32: '{s}'").as_str()))
        .collect();

    println!("Matrix provided: {matrix:?}");

    let size = matrix.len();
    let size_sqrt = (size as f64).sqrt() as usize;

    if size_sqrt.pow(2) != size {
        panic!("Expected a square matrix");
    }

    let determinant = calculate_determinant(&matrix, size_sqrt);

    println!("Determinant of the provided matrix is: {determinant}");
}

