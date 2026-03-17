
# Description

this project uses bitwise operations to make mathematical operations.
we are not allowed to use the standard mathematical operators or libreries


# Bitwise Operators

/* Bitwise AND (&)
   - Compares each bit of two numbers and returns a new number where each bit is set to 1 only if both corresponding bits of the original numbers are 1. */
    5 & 3 = 1 (in binary: 0101 & 0011 = 0001)

/* Bitwise OR (|)
    - Compares each bit of two numbers and returns a new number where each bit is set to 1 if at least one of the corresponding bits of the original numbers is 1. */
     5 | 3 = 7 (in binary: 0101 | 0011 = 0111)

/* Bitwise XOR (^)
    - Compares each bit of two numbers and returns a new number where each bit is set to 1 only if the corresponding bits of the original numbers are different. */
     5 ^ 3 = 6 (in binary: 0101 ^ 0011 = 0110)

/* Bitwise NOT (~)
    - Inverts each bit of a number, changing 1s to 0s and 0s to 1s. */
     ~5 = -6 (in binary: ~0101 = 1010, which is -6 in two's complement representation)

/* Left Shift (<<)
    - Shifts the bits of a number to the left by a specified number of positions, filling the vacated bits with 0s. */
     5 << 1 = 10 (in binary: 0101 << 1 = 1010)
     if we only shift for 1 position, we can also write it as 5 * 2 = 10

/* Right Shift (>>)
    - Shifts the bits of a number to the right by a specified number of positions. For unsigned numbers, the vacated bits are filled with 0s. For signed numbers, the vacated bits are filled with the sign bit (the most significant bit). */
     10 >> 1 = 2 (in binary: 1010 >> 1 = 0101)
     if we only shift for 1 position, we can also write it as 10 / 2 = 5
