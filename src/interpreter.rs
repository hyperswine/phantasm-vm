use phantasm_ir::spectre_ir::*;

// pretty much the view of each individual processor

/// Interpreter for phantasm IR
pub fn eval_instructions(instructions: Vec<SpectreInstruction>) {
    instructions.iter().for_each(|f| match f {
        SpectreInstruction::Arithemtic(a, b, c) => todo!(),
        SpectreInstruction::Bitwise(b) => todo!(),
        SpectreInstruction::Jump(a) => todo!(),
        SpectreInstruction::Yield => todo!(),
        SpectreInstruction::Return => todo!(),
    });
}

#[test]
fn test_instructions() {
    println!("Ins")
}
