use phantasm_ir::ir::Instruction;

// pretty much the view of each individual processor

/// Interpreter for phantasm IR
pub fn eval_instructions(instructions: Vec<Instruction>) {
    instructions.iter().for_each(|f| match f {
        Instruction::Load(a, b, c) => todo!(),
        Instruction::Store(a, b, c) => todo!(),
        Instruction::Arithmetic(a, b, c) => todo!(),
        Instruction::Branch(a, b) => todo!(),
        Instruction::Fence => todo!(),
        Instruction::Syscall => todo!(),
    });
}

#[test]
fn test_instructions() {
    println!("Ins")
}
