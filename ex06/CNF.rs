

// input string -> RPN (already NNF) -> return CNF (Conjunctive Normal Form)

#[path = "../ex05/Negation.rs"]
mod negation;

use self::negation::negation_normal_form;

#[derive(Clone, Debug)]
enum Expr {
    Var(char),
    Not(char),
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
}

// Parse RPN into expression tree
fn parse_rpn(formula: &str) -> Expr {
    let mut stack: Vec<Expr> = Vec::new();

    for c in formula.chars() {
        match c {
            'A'..='Z' => stack.push(Expr::Var(c)),

            '!' => {
                let expr = stack.pop().unwrap();
                match expr {
                    Expr::Var(v) => stack.push(Expr::Not(v)),
                    _ => panic!("Invalid NNF: NOT must apply to variable only"),
                }
            }

            '&' => {
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();
                stack.push(Expr::And(Box::new(left), Box::new(right)));
            }

            '|' => {
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();
                stack.push(Expr::Or(Box::new(left), Box::new(right)));
            }

            _ => {}
        }
    }

    stack.pop().unwrap()
}

// Distribute OR over AND (CNF transformation)
fn distribute(expr: Expr) -> Expr {
    match expr {
        Expr::And(a, b) => Expr::And(
            Box::new(distribute(*a)),
            Box::new(distribute(*b)),
        ),

        Expr::Or(a, b) => {
            let left = distribute(*a);
            let right = distribute(*b);

            match (left, right) {
                // (A & B) | C => (A | C) & (B | C)
                (Expr::And(a1, a2), r) => Expr::And(
                    Box::new(distribute(Expr::Or(a1, Box::new(r.clone())))),
                    Box::new(distribute(Expr::Or(a2, Box::new(r)))),
                ),

                // A | (B & C) => (A | B) & (A | C)
                (l, Expr::And(b1, b2)) => Expr::And(
                    Box::new(distribute(Expr::Or(Box::new(l.clone()), b1))),
                    Box::new(distribute(Expr::Or(Box::new(l), b2))),
                ),

                (l, r) => Expr::Or(Box::new(l), Box::new(r)),
            }
        }

        other => other,
    }
}

// Flattens nested AND expressions into a list of operands
// This fixes the only NNF results issue.
fn flatten_and(expr: Expr, out: &mut Vec<Expr>) {
    match expr {
        Expr::And(a, b) => {
            flatten_and(*a, out);
            flatten_and(*b, out);
        }
        other => out.push(other),
    }
}

// Convert expression back to RPN
fn to_rpn(expr: &Expr) -> String {
    match expr {
        Expr::Var(c) => c.to_string(),
        Expr::Not(c) => format!("{}!", c),

        Expr::And(_, _) => {
            let mut parts = Vec::new();
            flatten_and(expr.clone(), &mut parts);

            let mut out = String::new();

            // borrow instead of move
            for p in &parts {
                out.push_str(&to_rpn(p));
            }

            for _ in 1..parts.len() {
                out.push('&');
            }

            out
        }

        Expr::Or(a, b) => {
            format!("{}{}|", to_rpn(a), to_rpn(b))
        }
    }
}

pub fn conjunctive_normal_form(formula: &str) -> String {
    let nnf = negation_normal_form(formula);

    let tree = parse_rpn(&nnf);

    let cnf = distribute(tree);

    to_rpn(&cnf)
}
