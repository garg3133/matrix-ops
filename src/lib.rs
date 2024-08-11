use std::fmt;

pub struct Matrix {
    row: usize,
    col: usize,
    content: Vec<i32>
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut matrix_repr = String::new();

        let first_row = (0..self.col).map(|_| "  ".to_string()).collect::<Vec<String>>().join(" ");

        matrix_repr.push_str(format!("\n--{first_row}--\n").as_str());
        for i in 0..self.row {
            let row_start = i*self.col;
            let row_end = (i+1)*self.col;

            let matrix_row: String = self.content[row_start..row_end]
                .iter()
                .map(|el| format!("{el:0>2}"))
                .collect::<Vec<String>>()
                .join(" ");

            matrix_repr.push_str("| ");
            matrix_repr.push_str(&matrix_row);
            matrix_repr.push_str(" |\n");
        }
        matrix_repr.push_str(format!("--{first_row}--\n").as_str());

        write!(f, "{}", matrix_repr)
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
}
