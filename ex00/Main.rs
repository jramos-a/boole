
pub mod adder;

use crate::adder::adder;

fn main() {
    let result = adder(5, 3);
    println!("{}", result);
}
