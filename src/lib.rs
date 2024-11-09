// ============================================================
//                           Matrix Library                     
// ------------------------------------------------------------
//   - Matrix:       Represents a 2D array of values              
//   - Row Vector:   A matrix with a single row                   
//   - Column Vector: A matrix with a single column               
// ============================================================




use std::fmt;

pub trait Dimension  {
    fn get_dim(&self) -> (usize, usize);
}
pub trait Copy {
    fn copy(&self) -> Self;
}
pub trait Access {
    // Get element at the specified row and column
    fn get(&self, row: usize, col: usize) -> Result<f64, &'static str>;

    // Set element at the specified row and column
    fn set(&mut self, row: usize, col: usize, value: f64);
}

// ============================================================
//                           Matrix Library                     
// ------------------------------------------------------------
//   - Matrix:       Represents a 2D array of values                  
// ============================================================
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

impl Copy for Matrix {
    fn copy(&self) -> Self {
        // Perform a deep copy of the matrix: clone the data and copy other fields.
        Matrix {
            rows: self.rows,
            cols: self.cols,
            data: self.data.clone(), // Vec::clone() creates a copy of the vector
            stride: self.stride,
        }
    }
}

// Implement the Access trait for Matrix
impl Access for Matrix {
    fn get(&self, row: usize, col: usize) -> Result<f64, &'static str> {
        if row < self.rows && col < self.cols {
            let index = row * self.stride + col;
            Ok(self.data[index])
        } else {
            Err("there is  not a element with this index")
        }
    }

    fn set(&mut self, row: usize, col: usize, value: f64) {
        if row < self.rows && col < self.cols {
            let index = row * self.stride + col;
            self.data[index] = value;
        }
    }
}

// ============================================================
//                           Matrix Library                     
// ------------------------------------------------------------
//   - Row Vector:   A matrix with a single row                   
// ============================================================

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

impl Copy for RowVector {
    fn copy(&self) -> Self {
        RowVector {
            vector: self.vector.copy(), // Delegate the copy to the underlying matrix
        }
    }
}
// Implement Access trait for RowVector
impl Access for RowVector {
    fn get(&self, row: usize, col: usize) -> Result<f64, &'static str> {
        if row == 0 && col < self.vector.cols {
            Ok(self.vector.data[col])
        } else {
            Err("there is not  a element with this index")
        }
    }

    fn set(&mut self, row: usize, col: usize, value: f64) {
        if row == 0 && col < self.vector.cols {
            self.vector.data[col] = value;
        }
    }
}

// ============================================================
//                           Matrix Library                     
// ------------------------------------------------------------       //   - Column Vector: A matrix with a single column               
// ============================================================

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

impl Copy for ColVector {
    fn copy(&self) -> Self {
        ColVector {
            vector: self.vector.copy(), // Delegate the copy to the underlying matrix
        }
    }
}

// Implement Access trait for ColVector
impl Access for ColVector {
    fn get(&self, row: usize, col: usize) -> Result<f64, &'static str> {
        if col == 0 && row < self.vector.rows {
            Ok(self.vector.data[row])
        } else {
            Err("there is not a element with this index")
        }
    }

    fn set(&mut self, row: usize, col: usize, value: f64) {
        if col == 0 && row < self.vector.rows {
            self.vector.data[row] = value;
        }
    }
}
