use std::{fmt, ops::{Add, Index, IndexMut, Mul}};
use rand::Rng;
#[derive(Clone)]
pub struct Matrix{
    pub mat : Vec<f64>,
    pub rows : usize,
    pub cols: usize
}

impl fmt::Debug for Matrix{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        writeln!(f, "Matrix({}{}): ", self.rows, self.cols)?;
        for r in 0..self.rows{
            write!(f, "[" )?;
            for c in 0..self.cols{
                let val = self[(r,c)];
                write!(f, "{:>8.4} ", val)?; // formatting with 8 positions so they don't offset, basically padding
            }
            writeln!(f, "]")?;
        }
        Ok(())
    }
}
impl Index<(usize, usize)> for Matrix{
    type Output = f64;
    fn index (&self, (row,col) : (usize, usize)) -> &f64{
        &self.mat[row * self.cols + col]
    }
}

impl IndexMut<(usize, usize)> for Matrix{
    fn index_mut(&mut self, (row , col) : (usize, usize)) -> &mut Self::Output{
        &mut self.mat[row * self.cols + col]
    }
}
// should also implement for Matrix owned in the future 
impl Add<&Matrix> for &Matrix{
    type Output = Matrix;
    fn add(self, other: &Matrix) -> Matrix{
        //assert_eq!()
        let mut added_matrix :Vec <f64> = Vec::with_capacity(self.rows*self.cols);

        for i in 0..self.rows * self.cols{
            added_matrix.push(self.mat[i] + other.mat[i]);
        }

        Matrix { 
            mat: added_matrix, 
            rows: self.rows, 
            cols: self.cols 
        }
    }
}
impl Mul<&Matrix> for &Matrix{
    type Output = Matrix;
    fn mul(self, other: &Matrix) -> Matrix{
        assert_eq!(self.cols, other.rows, 
            "Dimensions of rows and columns do not match");
        let m_m_r = self.rows;
        let m_m_c = other.cols;    
        let mut result = Matrix {
            mat: vec![0.0; m_m_r * m_m_c],
            rows: m_m_r,
            cols: m_m_c,
        };

        for i in 0..self.rows {
            for k in 0..self.cols { 
                let temp_a = self[(i, k)];
                for j in 0..other.cols {
                    result[(i, j)] += temp_a * other[(k, j)];
                }
            }
        }
        result
    }
}
impl Matrix { 
    pub fn init_zero (nr_row : usize, nr_col : usize) -> Matrix{
        let matrix_zeros : Vec<f64> = vec![0.0; nr_row * nr_col];
        Matrix{
            mat: matrix_zeros,
            rows: nr_row,
            cols : nr_col
        }
    }
    pub fn init_rand (nr_row: usize, nr_col : usize) -> Matrix{
        let mut matrix_rand : Vec<f64> = Vec::with_capacity(nr_row*nr_col);
        let mut rng = rand::thread_rng();
        for _i in 0..nr_row*nr_col{
            matrix_rand.push(rng.gen_range(-1.0..1.0))
        }
        Matrix { 
            mat: matrix_rand,
            rows: nr_row,
            cols: nr_col
        }
    }
    pub fn transpose (&self) -> Matrix{
        let mut result = Matrix{
            mat: vec![0.0; self.cols * self.rows],
            rows: self.cols,
            cols:self.rows
        };
        for r in 0..self.rows{
            for c in 0..self.cols{
                result [(c,r)] = self[(r,c)];
            }
        }
        result
    }
}