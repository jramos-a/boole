
mod truthtable;

use truthtable::print_thruth_table;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <formula>", args[0]);
        return;
    }
    let formula: &str = &args[1];
    let result = print_thruth_table(formula);
    println!("{}", result);
}