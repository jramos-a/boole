
mod graycode;

use graycode::gray_code;

fn main() {
    let n = 7;
    println!("Gray code for {} is {}", n, gray_code(n));
}
