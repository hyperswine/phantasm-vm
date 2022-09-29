// THE VM boot, starts the main processors and hardware and loads the bios firmware

pub fn launch_vm() {
    let vm = VM{ mpu: todo!(), ram: todo!() };

}

// Machine State

struct VM {
    mpu: MainProcessor8Core,
    ram: MainMemory500M,
}

// made to match the host as well as possible
// multi-core setup, hardware tree and memory map, etc

struct Processor {
    registers: Registers,
}

// assume host has an 8 core processor
type MainProcessor8Core = MainProcessorUnit<8>;

// Abstraction for the processor
struct MainProcessorUnit<const N: usize> {
    processors: [Processor; N],
}

// A 500M memory bank to start off with, recompile for more
type MainMemory500M = MainMemory<500_000_000>;

struct MainMemory<const NBytes: usize> {
    ram: [u8; NBytes],
}

// Full set of registers on a risc-y processor
struct Registers {
    integer: [u64; 32],
    float: [f32; 32],
    // later
    vector: [u128; 32],
}
