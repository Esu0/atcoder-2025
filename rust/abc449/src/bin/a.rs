#[allow(unused_imports)]
use proconio::{input, marker};

fn main() {
    input! {
        d: f64
    }
    
    println!("{:.15}", d * d * std::f64::consts::PI / 4.);
}
