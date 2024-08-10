pub struct Matrix {
    row: usize,
    col: usize,
    content: Vec<i32>
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
