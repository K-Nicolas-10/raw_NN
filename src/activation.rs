#[derive(Clone)]
pub enum Activation{
    Sigmoid,
    ReLU
}
impl Activation{
    pub fn apply(&self, x: f64) -> f64{
        match self {
            Self::Sigmoid => Self::sigmoid(x),
            Self::ReLU => Self::relu(x)
        }
    }
    pub fn derivative(&self, x:f64) -> f64{
        match self{
            Self::Sigmoid => Self::sigmoid_der(x),
            Self::ReLU => Self::relu_der(x)
        }
    }
    fn relu(x: f64) -> f64{
        if x > 0.0 {x}
        else {0.0}
    }
    fn sigmoid(x: f64) -> f64{
        1.0/(1.0+ (-x).exp())
    }
    fn relu_der(x:f64)->f64{
        if x > 0.0 {1.0}
        else {0.0}
    }
    fn sigmoid_der(x:f64)->f64{
        Self::sigmoid(x) * (1.0 - Self::sigmoid(x))
    }
    
}