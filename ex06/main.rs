
mod cnf;

use cnf::conjunctive_normal_form;

fn main() {
    let formula = "AB&!"; // Example formula in RPN
    let cnf = conjunctive_normal_form(formula);
    println!("{}", cnf);
}
