pub mod vm;

use crate::vm::block::{Block, OpCode};

use colored::Colorize;

#[cfg(feature = "disassemble")]
use vm::block::BlockDbg;

fn main() {
    println!(
        "Hi, this is {} - a toy language you didn't wait for.",
        "neon".truecolor(240, 0, 255).bold()
    );

    let mut block = Block::new("ZeBlock");

    block.write_constant(1234.56, 2);
    block.write_constant(789.10, 4);

    block.write_op_code(OpCode::Return, 4);

    #[cfg(feature = "disassemble")]
    block.disassemble_block();
}
