

// we need to create a function that takes as input two natural numbers and returns their sum

// this are the allowed operators: &, |,  ~, ^, <<, >>, and comparison.

pub fn adder(a: u32, b: u32) -> u32 {
    let mut carry: u32;
    let mut sum: u32;

    sum = a ^ b; // sum of bits where at least one of the bits is not set
    carry = (a & b) << 1; // carry of bits where both bits are set

    while carry != 0 {
        let temp_sum = sum;
        sum = temp_sum ^ carry; // add carry to the sum
        carry = (temp_sum & carry) << 1; // calculate new carry
    }

    return sum;
}
