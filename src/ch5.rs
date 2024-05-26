/// # 5.3 - REVERSE BITS
///
/// Write a program that takes a 64-bit word and returns the 64-bit word consisting of the bits of the input word in reverse order. For example, if the input is alternating Is andOs,i.e.,(1010...10),theoutputshouldbealternatingOsand Is,i.e.,(0101...01).
/// # Notes
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_bits() {
        assert_eq!(reverse_bits(0), 0);
        assert_eq!(reverse_bits(0x0000_0000_0000_F000), 0x000F_0000_0000_0000);
    }
}
