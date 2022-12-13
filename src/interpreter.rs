use phantasm_ir::spectre_ir::{*, self};

use crate::vm::{phantasm_system, Executor};
use Instruction0::*;
use Instruction1::*;
use Instruction2::*;

// pretty much the view of each individual executor

/// Interpreter for phantasm IR
pub fn eval_instructions(executor: &mut Executor, instructions: Vec<SpectreInstruction>) {
    // each time you execute an instruction, you add size of instruction
    // on instructions to do execution, usually stack pop or push or read from some address
    // on instructions for control flow
    // the executor combinatory

    instructions.iter().for_each(|instruction| {
        match instruction.0 {
            Instruction0::I => {}
            Instruction0::D => {}
        }
        match instruction.1 {
            Instruction1::Arithemtic(a, b, c) => (),
            Instruction1::Bitwise(a) => (),
            Instruction1::Spawn(addr) => (),
            Instruction1::Jump(addr) => (),
            Instruction1::Yield => (),
            Instruction1::Return => (),
        }
        match instruction.2 {
            Instruction2::Scalar => (),
            Instruction2::Vector => (),
        }
    });
}

#[test]
fn test_instructions() {
    println!("Testing Basic Instructions");
    let phantasm_system = phantasm_system();
    // precond: SP of executor0 is X
    let instructions_to_execute = vec![SpectreInstruction(I, Jump(spectre_ir::Jump::Address), Scalar)];
    // execute
    // postcond: SP of executor0 is the address specified (loaded in)
}
