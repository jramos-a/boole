

// function that takes a string a propositonal formula of
// reverse polish notation, evaluates and retuns result

// use std::env;
//used by default, gives warning

pub fn eval_formula(formula: &str) -> bool {
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
