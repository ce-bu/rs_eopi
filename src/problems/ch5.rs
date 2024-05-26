///
/// # 5.3 - REVERSE BITS
///
/// Write a program that takes a 64-bit word and returns the 64-bit word consisting of the bits of the input word in reverse order. For example, if the input is alternating Is andOs,i.e.,(1010...10),theoutputshouldbealternatingOsand Is,i.e.,(0101...01).
///
/// # Notes
///
/// 76543210   10(6) 32(4) 54(2) 76(0) rotate left with (NumBlocks - BlockIndex - 1) * 2
/// 01234567
pub fn reverse_bits(n: u64) -> u64 {
    let mut tmp = n;
    let mut result: u64 = 0;

    const BLOCK_SIZE: u64 = 4;
    const NUM_BLOCKS: u64 = 64 / BLOCK_SIZE;
    const SWAPS: [u64; 16] = [
        0b0000, 0b1000, 0b0100, 0b1100, 0b0010, 0b1010, 0b0110, 0b1110, //
        0b0001, 0b1001, 0b0101, 0b1101, 0b0011, 0b1011, 0b0111, 0b1111,
    ];
    let mask = (1 << BLOCK_SIZE) - 1;

    for i in 0..NUM_BLOCKS {
        let block_value = SWAPS[(tmp & mask) as usize];
        let shift = (NUM_BLOCKS - i - 1) * BLOCK_SIZE;
        result = result | (block_value << shift);
        tmp = tmp >> BLOCK_SIZE;
    }
    result
}

///
/// 5.4 FIND A CLOSEST INTEGER WITH THE SAME WEIGHT
///
/// Define the weight of a nonnegative integer x to be the number of bits that are set to 1 in its binary representation. For example, since 92 in base-2 equals (1011100)2, the weight of 92 is 4.
/// Write a program which takes as input a nonnegative integer x and returns a number y which is not equal to x, but has the same weight as x and their difference, | y - x\, is as small as possible. You can assume x is not 0, or all Is. For example, if x = 6, you should return 5.
///
/// # Notes
///
///  x  = C + bM * 2^M + bN * 2^N (bM != bN)
///  y =  C + bN * 2^M + bM * 2^N (bM != bN)
/// |x - y| = 2^M - 2^N = 2^(M-N) => Minimuze M -N

pub fn closest_integer_same_weight(x: u64) -> u64 {
    for i in 0..63 {
        let b0 = (1 << i) & x;
        let b1 = (1 << (i + 1)) & x;
        if b0 != b1 {
            let y = x ^ ((1 << i) | (1 << (i + 1)));
            return y;
        }
    }
    x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_bits() {
        assert_eq!(reverse_bits(0), 0);
        assert_eq!(reverse_bits(0x0000_0000_0000_F000), 0x000F_0000_0000_0000);
    }

    #[test]
    fn test_closest_integer_same_weight() {
        assert_eq!(closest_integer_same_weight(6), 5);
    }
}
