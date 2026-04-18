use crate::matrix::Matrix;
use crate::activation::Activation;
#[derive(Clone)]
pub struct Layer{
    pub weights: Matrix,
    pub biases: Matrix,
    pub activation: Activation
}

impl Layer{
    //forward pass
    pub fn new(input_size: usize, output_size: usize, activation:Activation) -> Self{
       
        // output_size is neurons of the layer 
        // output_size is also the number of rows of the matrix because of the matrix
        // multiplication rule

        let m_weights: Matrix = Matrix::init_rand(output_size, input_size);
        let m_biases: Matrix = Matrix::init_zero(output_size, 1);

        Layer {
            weights: m_weights,
            biases: m_biases,
            activation: activation,
        } 

    }
    pub fn forward(&self, input:&Matrix) -> Matrix{

        let mut A : Matrix = &(&self.weights * input) + &self.biases;
        
        for i in 0.. A.rows*A.cols {
            A.mat[i] = self.activation.apply(A.mat[i]);
        }
        A
    }
}