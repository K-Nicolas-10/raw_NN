mod matrix;
mod layer;
mod activation;
mod network;
use matrix::{Matrix};
use layer::{Layer};
use activation::{Activation};
fn main() {

    let layer = Layer::new(3,2, Activation::Sigmoid);
    println!("{:?}", layer.weights);
    println!("{:?}", layer.biases);
    let mut input = Matrix::init_zero(3,1);
    input.mat = vec![0.5, -1.2, 3.0];
    let output = layer.forward(&input);
    println!("{:?}", output);

}

