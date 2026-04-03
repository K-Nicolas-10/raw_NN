pub struct Matrix{
    mat : Vec<f64>,
    rows : usize,
    cols: usize
}
impl Matrix{
    fn index (&self, row: usize, col: usize) -> usize{
        row * self.cols + col
    }
}
fn main() {
}
