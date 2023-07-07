use std::collections::VecDeque;
use std::fmt;

#[derive(PartialEq)]
enum Bit {
    One,
    Zero
}

impl fmt::Display for Bit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Bit::*;
        match self {
            One => write!(f, "1"),
            Zero => write!(f, "0")
        }
    }
}

struct Block {
    bit_array: Vec<Bit>,
    width: usize
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Bit::*;
        for j in 0..self.width {
            for i in 0..self.width {
                write!(f, "{} ", match self.bit_array.get(j * self.width + i) {
                    Some(bit) => &bit,
                    None => &Zero
                })?
            }
            writeln!(f, "")?
        }
        fmt::Result::Ok(())
    }
}

fn encode_bits(mut bits: VecDeque<Bit>) -> Vec<Block> {
    use Bit::*;
    fn format_block(block: &mut Block) {
        let mut xored: usize = 0;
        let mut set_bits: u32  = 0;
        for (i, bit) in block.bit_array.iter().enumerate() {
            if *bit == One {
                xored ^= i;
                set_bits += 1;
            }
        }
        let mut parity_bit: usize = 1;
        while parity_bit < block.width.pow(2) {
            if (parity_bit & xored) != 0 {
                block.bit_array[parity_bit] = One;
                set_bits += 1;
            }
            parity_bit <<= 1;
        }
        if set_bits % 2 == 1 {
            block.bit_array[0] = One;
        }
    }
    let mut blocks: Vec<Block> = Vec::new();
    while !bits.is_empty() {
        let mut block: Block = Block {
            bit_array: Vec::new(),
            width: 4
        };
        for i in 0..block.width.pow(2) {
            if bits.is_empty() { break; }
            if !(i == 0 || i.is_power_of_two()) {
                block.bit_array.push(bits.pop_front().unwrap());
            } else {
                block.bit_array.push(Zero);
            }
        }
        format_block(&mut block);
        blocks.push(block);
    }
    blocks
}

fn main() {
    use Bit::*;
    let blocks: Vec<Block> = encode_bits(VecDeque::from([
        Zero, Zero, Zero, Zero,
        One, One, One, One,
        Zero, Zero, Zero, One
    ]));
    for (i, block) in blocks.iter().enumerate() {
        println!("Block {}:\n{}", i, block);
    }
}