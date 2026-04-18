use crate::layer::Layer;
use crate::matrix::Matrix;
pub struct Network{
    layers: Vec<Layer>,
}

impl Network{
    pub fn new() ->  Self{
        Network { 
            layers: Vec::new() 
        }
    }

    pub fn add_layer(&mut self, layer : Layer){
        self.layers.push(layer);
    }

    pub fn forward(&self, input: Matrix) -> Matrix {
        // clone used for overwriting so we can keep taking the precedent output for forwarding.
        // i.e. the input for the first layer really is the input but for the other ones
        // the input is the ouput.
        let mut current_input = input.clone();
        
        for layer in &self.layers{
            current_input = layer.forward(&current_input);
        }
        current_input
    }
}
