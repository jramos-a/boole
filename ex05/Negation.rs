

// string as input propositional formula in RPN, returns equivalent formula in NNF(Negation Normal Form)
// every negation operators must be allocated after a variable


// AB&! (equivalent: ¬(A ∧ B)) = A!b!| (equivalent: ¬A ∨ ¬B)
// result must contain only "!, &, |" even if th eoutput contains other operations


fn parse_input(formula &str) {
    
}

fn negation_normal_form(formula: &str) -> String {

}

fn main() {
    let args: Vec<string> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <formula>", args[0]);
        return;
    }
    let formula: &str = &args[1];
    let result = negation_normal_form(formula);
    println!("{}", result);
}
