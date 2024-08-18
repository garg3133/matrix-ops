use std::fs;
use matrix_ops::{Matrix, MatrixError};

fn main() {
    let file_content = fs::read_to_string("examples/matrix.txt")
              .expect(format!("Error while reading file 'examples/matrix.txt'").as_str());

    let matrix = match Matrix::try_from(file_content) {
        Ok(value) => value,
        Err(MatrixError::ConversionError(error)) => panic!("err: {error}")
    };

    let det = matrix.determinant();

    println!("Input:\n{matrix}\n");
    println!("Determinant:\n{det}\n");
}

