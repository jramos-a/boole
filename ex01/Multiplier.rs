
#[path = "../ex00/Adder.rs"]
mod adder;

pub fn multiplier(a: u32, b: u32) -> u32 {
    let mut result: u32 = 0;
    let mut multiplier: u32 = b;
    let mut multiplicand: u32 = a;

    while multiplier > 0 {
        if (multiplier & 1) == 1 {
            result = adder::adder(result, multiplicand); // add multiplicand to result if the least significant bit of multiplier is 1
        }
        multiplicand <<= 1; // shift multiplicand left by 1 (multiply by 2)
        multiplier >>= 1; // shift multiplier right by 1 (divide by 2)
    }

    return result;
}

