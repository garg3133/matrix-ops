use matrix_ops::Matrix;

fn main() {
    let matrix1 = Matrix::generate(4, 2);
    let matrix2 = Matrix::new(2, 3, vec![1, 2, 3, 4, 5, 6]);

    let mul = matrix1.multiply(&matrix2);

    println!("Input matrix 1:\n{matrix1}\n");
    println!("Input matrix 2:\n{matrix2}\n");
    println!("Multiplication Result:\n{mul}\n");
}

