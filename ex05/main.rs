
mod negation;

use negation::negation_normal_form;

fn main() {
    let result = negation_normal_form("AB&!");
    println!("{}", result);
}
