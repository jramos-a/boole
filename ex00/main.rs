
mod adder;

use adder::adder;

fn main() {
    let result = adder(5, 3);
    println!("{}", result);
}
