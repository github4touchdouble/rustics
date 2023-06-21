use rand::{thread_rng, Rng};

#[derive(Clone)]
pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<Vec<f64>>,
}

impl Matrix {
    pub fn zeros(rows: usize, cols: usize) -> Matrix {
        // returns a matrix filled with values of 0.0
        Matrix {
            rows,
            cols,
            data: vec![vec![0.0; cols]; rows],
        }
    }

    pub fn random(rows: usize, cols: usize) -> Matrix {
        // returns a matrix filled with random values in range of -1.0 to 1.0
        let mut rng = thread_rng();
        let mut res = Matrix::zeros(rows, cols);

        for i in 0..rows {
            for j in 0..cols {
                res.data[i][j] = rng.gen::<f64>() * 2.0 - 1.0;
            }
        }

        res
    }

    pub fn from(data: Vec<Vec<f64>>) -> Matrix {
        // make matrix from vector of vector
        Matrix {
            rows: data.len(),
            cols: data[0].len(),
            data,
        }
    }

    pub fn multiply(&mut self, other: &Matrix) -> Matrix {
        // check if operation is even possible.
        if self.cols != other.rows {
            panic!("Operation can't be executed. Attempted to multiply matrices of incorrect dimensions.");
        }

        // standard matrix multiplication
        let mut res = Matrix::zeros(self.rows, other.cols);
        for i in 0..self.rows {
            for j in 0..other.cols {
                let mut sum = 0.0;
                for k in 0..self.cols {
                    sum += self.data[i][k] * other.data[k][j];
                }
                res.data[i][j] = sum;
            }
        }
        res
    }

    pub fn dot_multiply(&mut self, other: &Matrix) -> Matrix {
        // less complex matrix multiplication. works only for matrices with the same dimensions

        // check if operation is even possible
        if self.rows != other.rows || self.cols != other.cols {
            panic!("Operation can't be executed. Attempted dot multiplication of matrices of incorrect dimensions.");
        }

        // standard matrix dot multiplication
        let mut res = Matrix::zeros(self.rows, self.cols);
        for i in 0..self.rows {
            for j in 0..self.cols {
                res.data[i][j] = self.data[i][j] * other.data[i][j];
            }
        }
        res
    }

    pub fn add(&mut self, other: &Matrix) -> Matrix {
        // check if operation is even possible
        if self.rows != other.rows || self.cols != other.cols {
            panic!("Operation can't be executed. Attempted to add matrices of incorrect dimensions.");
        }

        // standard matrix addition
        let mut res = Matrix::zeros(self.rows, self.cols);
        for i in 0..self.rows {
            for j in 0..self.cols {
                res.data[i][j] = self.data[i][j] + other.data[i][j];
            }
        }
        res
    }

    pub fn subtract(&mut self, other: &Matrix) -> Matrix {
        // check if operation is even possible
        if self.rows != other.rows || self.cols != other.cols {
            panic!("Operation can't be executed. Attempted to subtract matrices of incorrect dimensions.");
        }

        // standard matrix subtraction
        let mut res = Matrix::zeros(self.rows, self.cols);
        for i in 0..self.rows {
            for j in 0..self.cols {
                res.data[i][j] = self.data[i][j] - other.data[i][j];
            }
        }
        res
    }

    pub fn map(&mut self, function: &dyn Fn(f64) -> f64) -> Matrix {
        //map a custom function on each matrix element and replace it with the result
        Matrix::from(
            (self.data)
                .clone()
                .into_iter()
                .map(|row| row.into_iter().map(|value| function(value)).collect())
                .collect(),
        )
    }

    pub fn transpose(&mut self) -> Matrix {
        // make a axb matrix to bxa matrix
        let mut res = Matrix::zeros(self.cols, self.rows);
        for i in 0..self.rows {
            for j in 0..self.cols {
                res.data[j][i] = self.data[i][j];
            }
        }
        res
    }
}