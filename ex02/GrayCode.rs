

// take an integer n, return gray code

pub fn gray_code(n: u32) -> u32 {
    n ^ (n >> 1)
}
