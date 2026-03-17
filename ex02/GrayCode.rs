

// take an integer n, return gray code

fn gray_code(n: u32) -> u32 {
    n ^ (n >> 1)
}

fn main() {
    let n = 7;
    println!("Gray code for {} is {}", n, gray_code(n));
}
