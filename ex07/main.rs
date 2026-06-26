
mod sat;

use sat::sat;

fn main() {
    let result = sat("AB|C&!");
    println!("Result: {}", result);
}
