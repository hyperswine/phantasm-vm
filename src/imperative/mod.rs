// THE VM boot, starts the main processors and hardware and loads the bios firmware

pub fn launch_vm() {
    let vm = VM::default();

    // pass control to VM
}

// Hardware Subsystems?
// allow host passthrough where possible to take inputs and interrupts from host hardware
// and change the state of the VM so that next instructions just work
// maybe make the VM listen to host interrupts via process_input()?? which queues the interrupts
// and must all be completed until executing the next instruction

// OR software emulated devices that somehow actively listen on mouse and keyboard from host

// Machine State
#[derive(Debug, Default)]
struct VM {
    mpu: MainProcessor8Core,
    ram: MainMemoryD,
}

// made to match the host as well as possible
// multi-core setup, hardware tree and memory map, etc
#[derive(Debug, Default, Clone, Copy)]
struct Processor {
    registers: Registers,
}

// assume host has an 8 core processor
#[derive(Debug)]
struct MainProcessor8Core(MainProcessorUnit<8>);

impl Default for MainProcessor8Core {
    fn default() -> Self {
        Self(MainProcessorUnit {
            processors: [Processor::default(); 8],
        })
    }
}

// Abstraction for the processor
#[derive(Debug)]
struct MainProcessorUnit<const N: usize> {
    processors: [Processor; N],
}

#[derive(Debug, Clone, Copy)]
struct Frame([u8; 4096]);

impl Default for Frame {
    fn default() -> Self {
        Self([0 as u8; 4096])
    }
}

// A good amount to start off with, recompile for more
#[derive(Debug)]
struct MainMemoryD(MainMemory<10_000, Frame>);

impl Default for MainMemoryD {
    fn default() -> Self {
        Self(MainMemory {
            ram: [Frame::default(); 10_000],
        })
    }
}

// maybe memory as in number of frames, and frame size
#[derive(Debug)]
struct MainMemory<const NFrames: usize, Frame> {
    ram: [Frame; NFrames],
}

// Full set of registers on a risc-y processor
#[derive(Debug, Default, Clone, Copy)]
struct Registers {
    pc: u64,
    integer: [u64; 32],
    float: [f32; 32],
    // simulate mmu and TLB and etc
    kernel_base_tt_addr: u64,
    user_base_tt_addr: u64,
    // later
    vector: [u128; 32],
}
