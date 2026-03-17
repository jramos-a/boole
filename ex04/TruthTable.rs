

// write a function that takes a string containing a propositional formula
// in RPN and write truth of table STDOUT

// can have up to 26 varaiables per letter

fn result(formula: &str, variables: &Vec<char>, values: &Vec<u8>) -> u8 {
    let mut stack = Vec::new();
    for c in formula.chars() {
        if c.is_ascii_alphabetic() {
            let index = variables.iter().position(|&v| v == c).unwrap();
            stack.push(values[index]);
        } else {
            let b = stack.pop().unwrap();
            let a = stack.pop().unwrap(); // will crash if '!' is the operator
            // parse the operators
            let res = match c {
                '&' => a & b,
                '|' => a | b,
                '^' => a ^ b,
                '!' => !b & 1,          // Bitwise NOT, then mask to keep it 0 or 1
                '>' => (!a & 1) | b,    // Logic: (NOT a) OR b
                '=' => !(a ^ b) & 1,    // Logic: NOT (a XOR b) results in 1 if they match
                _ => panic!("Invalid operator"),
            };
            stack.push(res);
        }
    }
    stack.pop().unwrap()
}

fn print_table(variables: Vec<char>, formula: &str) -> String {
    let mut table = String::new();
    // print header
    for var in &variables {
        table.push_str(&format!("| {} ", var));
    }
    table.push_str("| = |\n");
    // print separator
    for _ in 0..variables.len() {
        table.push_str("|---");
    }
    table.push_str("|---|\n");
    // print rows
    let num_rows = 1 << variables.len();
    for i in 0..num_rows {
        let mut row = String::new();
        for (j, _var) in variables.iter().enumerate() {
            let value = (i >> j) & 1;
            row.push_str(&format!("| {} ", value));
        }
        row.push_str("|");
        table.push_str(&row);
        let values: Vec<u8> = (0..variables.len()).map(|j| (i >> j) & 1).collect();
        let result = result(formula, &variables, &values);
        table.push_str(&format!(" {} |\n", result));
    }
    table
}

fn print_thruth_table(formula: &str) -> String {
    let mut variables = Vec::new();
    for c in formula.chars() {
        if c.is_ascii_alphabetic() && !variables.contains(&c) {
            variables.push(c);
        }
    }
    print_table(variables, formula)
}

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