
// input = string containing propositional formula in RPN -> tells if satisfable or not

// can use CNF and NNF


#[path = "../ex05/Negation.rs"]
mod negation;

use self::negation::negation_normal_form;


fn eval(formula: &str, values: &[bool; 26]) -> bool {
    let mut stack: Vec<bool> = Vec::new();

    for c in formula.chars() {
        match c {
            'A'..='Z' => {
                let idx = (c as u8 - b'A') as usize;
                stack.push(values[idx]);
            }

            '!' => {
                let a = stack.pop().unwrap();
                stack.push(!a);
            }

            '&' => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a && b);
            }

            '|' => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a || b);
            }

            _ => {}
        }
    }

    stack.pop().unwrap()
}

pub fn sat(formula: &str) -> bool {
    // Convert to NNF
    let formula = negation_normal_form(formula);

    // Collect different variables
    let mut used = [false; 26];
    let mut vars = Vec::new();

    for c in formula.chars() {
        if ('A'..='Z').contains(&c) {
            let idx = (c as u8 - b'A') as usize;

            if !used[idx] {
                used[idx] = true;
                vars.push(idx);
            }
        }
    }

    let n = vars.len();

    // Try every possible assignment
    for mask in 0..(1usize << n) {
        let mut values = [false; 26];

        for (i, &idx) in vars.iter().enumerate() {
            values[idx] = (mask & (1 << i)) != 0;
        }

        if eval(&formula, &values) {
            return true;
        }
    }

    false
}
