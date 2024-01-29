struct Matrix {
    data: Vec<Vec<f64>>
}

/// A class that makes dealing with matrices much easier. It is mainly for solving systems of equations
impl Matrix {

    /// Solves the series of equations by solving the 'data' variable of the matrix
    pub fn solve(&mut self) -> Result<(), &'static str> {
        return self.solve_helper(0, 0);
    }

    /// The helper method for the solve function
    fn solve_helper(&mut self, row: int32, column: int32) -> Result<(), &'static str>{
        if (row >= self.data.len() || column >= matrix[0].len()) {
            self.correct_floats();
            return Ok(())
        }

        if (self.data[row][column] != 0) {
            self.multiply_row(row, 1 / self.data[row][column]);

            for i in 0..self.data.len() {
                if (i == row) {
                    continue;
                }

                self.add_to_row(i, -matrix[i][column], row);
            }

            self.solve_helper(row + 1, column + 1);
        } else {
            if (gaussian_arrange(row, column)) {
                self.solve_helper(row, column);
            } else {
                return Err("Illegal argument");
            }

        }
    }

    /// Adds
    pub fn add_to_row(&mut self, add_to: int32, scalar: float64, add_from: int32) {
        for i in 0..self.data[0].len() {
            self.data[add_to][i] += scalar * self.data[add_from][i];
        }
    }

    pub fn correct_floats(&mut self) {
        let rounding_factor = powi(10, -5);
        for i in 0..self.data.len() {
            for j in 0..self.data[i].len() {
                if (math.abs(1 - self.data[i][j]) <= rounding_factor) {
                    self.data[i][j] = 1.0;
                }
                if (math.abs(self.data[i][j]) <= rounding_factor) {
                    self.data[i][j] = 0.0;
                }
            }
        }
    }

    pub fn gaussian_arrange(&mut self, row: int32, column: int32) -> bool{
        if (matrix[row][column] != 0) {
            return true;
        }

        for r in row..self.data.len() {
            if (matrix[r][column] != 0) {
                switch_row(row, r);
                return true;
            }
        }

        return false;
    }

    pub fn switch_row(&mut self, row1: int32, row2: int32) {
        let temp = self.data[row1];
        self.data[row1] = self.data[row2];
        self.data[row2] = temp;
    }

}