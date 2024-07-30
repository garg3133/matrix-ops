use std::{env, fmt, fs};

struct Matrix {
    row: usize,
    col: usize,
    content: Vec<i64>
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut matrix_repr = String::new();
        for i in 0..self.row {
            let row_start = i*self.col;
            let row_end = (i+1)*self.col;

            let matrix_row: String = self.content[row_start..row_end]
                .iter()
                .map(|el| format!("{el:0>2}"))
                .collect::<Vec<String>>()
                .join(" ");

            matrix_repr.push_str(&matrix_row);
            matrix_repr.push('\n');
        }
        write!(f, "{}", matrix_repr)
    }
}

fn get_matrix_files_from_argv(mut argv_iter: impl Iterator<Item = String>) -> (String, String) {
    let file_name_1 = if let Some(value) = argv_iter.next() {
        value
    } else {
        help();
        panic!("No matrix files provided as arguments.");
    };

    let file_name_2 = if let Some(value) = argv_iter.next() {
        value
    } else {
        help();
        panic!("Only one matrix file provided as arguments");
    };

    (file_name_1, file_name_2)
}

fn get_matrix_from_file(file_name: &str) -> Matrix {
    let file_content = fs::read_to_string(&file_name)
            .expect(format!("Error while reading file '{file_name}'").as_str());

    let mut row_size = 0;
    let mut col_size = 0;
    let mut matrix: Vec<i64> = Vec::new();

    let lines = file_content.split('\n');
    for line in lines {
        let mut mat_row: Vec<i64> = line
            .split(' ')
            .filter(|s| *s != "")
            .map(|s| s.parse().expect(format!("Failed to parse to i32: '{s}'").as_str()))
            .collect();

        let row_len = mat_row.len();
        if row_len > 0 {
            matrix.append(&mut mat_row);

            row_size += 1;
            if col_size == 0 {
                col_size = row_len;
            } else if col_size != row_len {
                panic!("Error in row {} of matrix '{}'; Expected {} elements, found {}",
                    row_size, file_name, col_size, row_len);
            }
        }
    }

    Matrix {row: row_size, col: col_size, content: matrix}
}

fn multiply_and_add(vec1: &[i64], vec2: &[i64]) -> i64 {
    let mut res: i64 = 0;

    for i in 0..vec1.len() {
        res += vec1[i]*vec2[i];
    }

    res
}

fn multiply_matrices(matrix1: Matrix, matrix2: Matrix) -> Matrix {
    if matrix1.col != matrix2.row {
        panic!("Incompatible matrices; cannot be multiplied: no. of cols in first matrix should be equal to no. of rows in the second.");
    }

    let mut result: Vec<i64> = Vec::new();

    for row_from_1 in 0..matrix1.row {
        for col_from_2 in 0..matrix2.col {
            // get row from matrix1
            let start = row_from_1*matrix1.col;
            let end = (row_from_1+1)*matrix1.col;
            let vec1_slice = &matrix1.content[start..end];
                // .iter().cloned().collect();

            // get column from matrix2
            let mut vec2: Vec<i64> = Vec::new();
            for i in 0..matrix2.row {
                vec2.push(matrix2.content[i*matrix2.col + col_from_2]);
            }
            
            result.push(multiply_and_add(vec1_slice, &vec2));
        }
    }
    
    Matrix {row: matrix1.row, col: matrix2.col, content: result}
}

fn main() {
    let mut argv_iter = env::args();

    // ignore first argv (path to binary)
    argv_iter.next();

    let (file1, file2) = get_matrix_files_from_argv(argv_iter);

    let matrix1 = get_matrix_from_file(&file1);
    let matrix2 = get_matrix_from_file(&file2);

    println!("Matrix 1:\n{}", matrix1);
    println!("Matrix 2:\n{}", matrix2);

    let result = multiply_matrices(matrix1, matrix2);

    println!("Multiplication of the provided matrices give:\n\n{result}");
}

fn help() {
    println!("Usage:");
    println!("  cargo run --bin multiply matrix1.txt matrix2.txt");
}

