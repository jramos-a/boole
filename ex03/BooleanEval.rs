

// function that takes a string a propositonal formula of
// reverse polish notation, evaluates and retuns result

// use std::env;
//used by default, gives warning

fn eval_formula(formula: &str) -> bool {
    let mut stack: Vec<i32> = Vec::new();

    if formula.is_empty() {
        return false;
    }

    for c in formula.chars() {
        match c {
            '0' => stack.push(0),
            '1' => stack.push(1),
            '&' => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a & b);
            },
            '|' => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a | b);
            },
            '^' => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a ^ b);
            },
            _ => panic!("Invalid character in formula"),
        }
    }
    stack.pop().unwrap() == 1
}

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
