
mod booleaneval;

use booleaneval::eval_formula;

fn main() {
    // the way to take arguments
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <formula>", args[0]);
        return;
    }
    let formula = &args[1];
    let result = eval_formula(formula);
    println!("{}", result);
}
