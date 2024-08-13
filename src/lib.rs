use std::fmt;
use rand::Rng;

pub struct Matrix {
    row: usize,
    col: usize,
    content: Vec<i32>
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut matrix: Vec<String> = Vec::new();
        for i in 0..self.row {
            let row_start = i*self.col;
            let row_end = (i+1)*self.col;

            let matrix_row: String = self.content[row_start..row_end]
                .iter()
                .map(|el| format!("{el:0>2}"))
                .collect::<Vec<String>>()
                .join(" ");

            matrix.push(matrix_row);
        }
        write!(f, "{}", matrix.join("\n"))
    }
}

impl Matrix {
    pub fn row(&self) -> usize {
        self.row
    }

    pub fn col(&self) -> usize {
        self.col
    }

    pub fn is_square(&self) -> bool {
        self.row == self.col
    }

    pub fn new(row: usize, col: usize, content: Vec<i32>) -> Self {
        if content.len() != row*col {
            panic!("Invalid matrix! Expected {row}*{col} matrix cell elements, got: {content:?}");
        }

        Matrix {row, col, content}
    }

    pub fn generate(row: usize, col: usize) -> Matrix {
        let mut final_matrix: Vec<i32> = Vec::new();

        let mut thread_rng = rand::thread_rng();

        (0..row).for_each(|_| {
            (0..col).for_each(|_| {
                let num = thread_rng.gen_range(1..100);
                final_matrix.push(num);
            });
        });

        Matrix {row, col, content: final_matrix}
    }

    pub fn multiply(&self, matrix2: &Matrix) -> Matrix {
        if self.col != matrix2.row {
            panic!("Incompatible matrices; cannot be multiplied: no. of cols in first matrix should be equal to no. of rows in the second.");
        }

        let mut result: Vec<i32> = Vec::new();

        for row_from_1 in 0..self.row {
            for col_from_2 in 0..matrix2.col {
                // get row from self
                let start = row_from_1*self.col;
                let end = (row_from_1+1)*self.col;
                let vec1_slice = &self.content[start..end];

                // get column from matrix2
                let mut vec2: Vec<i32> = Vec::new();
                for i in 0..matrix2.row {
                    vec2.push(matrix2.content[i*matrix2.col + col_from_2]);
                }
                
                let res = vec1_slice.iter().enumerate().map(|(i, &v1)| v1*vec2[i]).sum();
                result.push(res);
            }
        }
        
        Matrix {row: self.row, col: matrix2.col, content: result}
    }

    fn calculate_determinant(matrix: &Matrix) -> i64 {
        let size = matrix.row;

        if size == 1 {
            return matrix.content[0] as i64;
        }

        let mut det: i64 = 0;
        let mut sign = -1;

        for ind in 0..size {
            let mut child_matrix = Vec::new();

            for i in 1..size { // iterate over rows
                for j in 0..size { // iterate over cells in each row
                    if j == ind {
                        continue;
                    }

                    child_matrix.push(matrix.content[i*size+j]);
                }
            }

            let child_matrix = Matrix::new(size-1, size-1, child_matrix);

            sign *= -1;
            det += sign * matrix.content[ind] as i64 * Self::calculate_determinant(&child_matrix);
        }

        det
    }

    pub fn determinant(&self) -> i64 {
        if self.row != self.col {
            panic!("Expected a square matrix.");
        }

        Self::calculate_determinant(self)
    }
}

