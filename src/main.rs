use project::{Matrix, Access};
fn main() {
    let mut _matrix = Matrix::new(2, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 0.0]);
    _matrix.set(1, 1, 2.5);
    println!("{:?}", _matrix);
}
