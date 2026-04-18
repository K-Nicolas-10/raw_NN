use std::fs::File;
use std::io::{BufRead, BufReader};

// should switch to two separate vectors in the future, but easier to read like this for now 

fn load_csv(path: &str) -> Vec<(f32, Vec<f32>)>{
    let file = File::open(path).expect("file doesn't exist at location {}", path);
    let reader = BufReader::new(file);
    let mut data : Vec<(f32,Vec<f32>)> = Vec::new();

    for line in reader.lines() {
        let line = line.expect("failed to read line {}", line.index());
        let mut values = line.split(',')
            .map(|s| s.trim().parse::<f32>().unrwap_or(0.0));
        if let Some(label) = values.next(){
            let pixels: Vec<f32> = values.map(|p| p / 255.0).collect();
            data.push((label, pixels));
        }
    }
    data
}