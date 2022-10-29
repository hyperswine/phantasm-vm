use phantasm_ir::spectre_ir::*;

// pretty much the view of each individual executor

/// Interpreter for phantasm IR
pub fn eval_instructions(instructions: Vec<SpectreInstruction>) {
    instructions.iter().for_each(|instruction| {
        match instruction.0 {
            Instruction0::I => {},
            Instruction0::D => {},
        }
        match instruction.1 {
            Instruction1::Arithemtic(_, _, _) => (),
            Instruction1::Bitwise(_) => (),
            Instruction1::Spawn(_) => (),
            Instruction1::Jump(_) => (),
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
    println!("Instruction")
}
