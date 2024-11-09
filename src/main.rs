use std::fmt;


pub trait Dimension  {
    fn get_dim(&self) -> (usize, usize);
}

pub struct Matrix {
    pub(crate) rows: usize,
    pub(crate) cols: usize,
    pub(crate) data: Vec<f64>,
    pub(crate) stride: usize,
}

impl Matrix {
    pub fn new(rows: usize, cols: usize, data: Vec<f64>) -> Self {
        assert_eq!(rows * cols, data.len(), "Data length must match dimensions.");
        Self {
            rows,
            cols,
            data,
            stride: cols,
        }
    }
    pub fn get_dim(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }
}

impl fmt::Debug for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Matrix ({}x{}):\n", self.rows, self.cols)?;
        for r in 0..self.rows {
            write!(f, "[")?;
            for c in 0..self.cols {
                write!(f, " {}", self.data[r * self.stride + c])?;
                if c < self.cols - 1 {
                    write!(f, ",")?;
                }
            }
            writeln!(f, " ]")?;
        }
        Ok(())
    }
}

impl Dimension for Matrix {
    fn get_dim(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }
}

pub struct RowVector {
    pub(crate) vector: Matrix,
}

impl RowVector {
    pub fn new(size: usize, data: Vec<f64>) -> Self {
        assert_eq!(size, data.len(), "Data length must match the specified size.");
        Self {
            vector: Matrix::new(1, size, data),
        }
    }
}

impl fmt::Debug for RowVector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RowVector: {:?}", self.vector)
    }
}

impl Dimension for RowVector {
    fn get_dim(&self) -> (usize, usize) {
        self.vector.get_dim()
    }
}

pub struct ColVector {
    pub(crate) vector: Matrix,
}

impl ColVector {
    pub fn new(size: usize, data: Vec<f64>) -> Self {
        assert_eq!(size, data.len(), "Data length must match the specified size.");
        Self {
            vector: Matrix::new(size, 1, data),
        }
    }
}

impl fmt::Debug for ColVector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ColVector: {:?}", self.vector)
    }
}

impl Dimension for ColVector {
    fn get_dim(&self) -> (usize, usize) {
        self.vector.get_dim()
    }
}

fn main() {
    // Row Vector
    let row_vector = RowVector::new(3, vec![1.0, 2.0, 3.0]);
    println!("{:?}", row_vector); // Prints RowVector: Matrix (1x3): [ 1.0, 2.0, 3.0 ]
    let dim = row_vector.get_dim();
    println!("{} {}", dim.0, dim.1);

    // Column Vector
    let col_vector = ColVector::new(3, vec![4.0, 5.0, 6.0]);
    println!("{:?}", col_vector); // Prints ColVector: Matrix (3x1): [ 4.0 ], [ 5.0 ], [ 6.0 ]
    let dim = col_vector.get_dim();
    println!("{} {}", dim.0, dim.1);

    // Matrix
    let matrix = Matrix::new(2, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
    println!("{:?}", matrix); // Prints Matrix (2x3): [ 1.0, 2.0, 3.0 ], [ 4.0, 5.0, 6.0 ]
    let dim = matrix.get_dim();
    println!("{} {}", dim.0, dim.1);
}
