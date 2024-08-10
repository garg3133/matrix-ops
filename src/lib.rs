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
}
