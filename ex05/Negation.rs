// Converts a propositional formula in RPN into its equivalent Negation Normal Form (NNF).
// NNF means: negations only apply directly to variables, and only operators !, &, | remain.
//
// Example:
//   AB&!  => !(A ∧ B) => A!B!|
//
// Input is assumed to be valid RPN.
//
// Only operators allowed in final result: !, &, |


/*
    Main transformation pipeline:

    - Reads RPN expression left-to-right
    - Uses a stack to build partial expressions
    - Eliminates implications and equivalences
    - Converts everything into base operators: !, &, |
    - Applies negation rules using `negate_expr`
 */
fn negation_normal_form(formula: &str) -> String {
    let mut stack: Vec<String> = Vec::new();

    for c in formula.chars() {
        match c {
            // Operand: push variable directly
            'A'..='Z' => stack.push(c.to_string()),

            // Negation: apply De Morgan / push negated form
            '!' => {
                let a = stack.pop().unwrap();
                stack.push(negate_expr(a));
            }

            // AND: combine two top elements
            '&' => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(format!("{}{}&", a, b));
            }

            // OR: combine two top elements
            '|' => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(format!("{}{}|", a, b));
            }

            // Implication: A > B  ≡  !A | B
            '>' => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(format!("{}!{}|", a, b));
            }

            // Equivalence: A = B ≡ (A & B) | (!A & !B)
            '=' => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(format!(
                    "{}{}&{}!{}!&|",
                    a, b, a, b
                ));
            }

            _ => {}
        }
    }

    // Final result of stack is full NNF expression in RPN
    stack.pop().unwrap()
}


/*
    Helper function to apply negation to an RPN expression.
    Applies logical negation to a full RPN expression string.

    This function ensures that:
    - negations are pushed down to variables
    - De Morgan laws are applied:
        !(A & B) = !A | !B<
        !(A | B) = !A & !B
    - double negation is removed: !!A = A

    It uses a stack-based evaluator over the RPN string.
 */
fn negate_expr(expr: String) -> String {
    let mut stack: Vec<String> = Vec::new();

    for c in expr.chars() {
        match c {
            // Variable becomes negational form: A => A!
            'A'..='Z' => stack.push(format!("{}!", c)),

            // Double handle negation simplification
            '!' => {
                let a = stack.pop().unwrap();

                // !!A = A
                if a.ends_with('!') {
                    stack.push(a[..a.len() - 1].to_string());
                } else {
                    stack.push(format!("{}!", a));
                }
            }

            // !(A & B) = !A | !B
            '&' => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(format!("{}{}|", a, b));
            }

            // !(A | B) = !A & !B
            '|' => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(format!("{}{}&", a, b));
            }

            _ => {}
        }
    }

    // Final negated expression in RPN
    stack.pop().unwrap()
}


fn main() {
    let result = negation_normal_form("AB&!");
    println!("{}", result);
}