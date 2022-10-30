/*
    It just works
*/

use derive_new::new;
use phantasm_ir::spectre_ir::{Instructions, SpectreInstruction};
use std::{
    intrinsics::size_of,
    ptr::{read_volatile, write_volatile},
};

derive_alias! {
    #[derive(Defaults!)] = #[derive(Debug, Clone, Copy, new)];
    #[derive(Complete!)] = #[derive(Debug, Clone, Copy, Default, new)];
}

pub type Byte = u8;

#[derive(Debug, Clone, Copy, new)]
pub struct Cache<const SIZE: usize>([Byte; SIZE]);

impl<const SIZE: usize> Default for Cache<SIZE> {
    fn default() -> Self {
        Self([0 as Byte; SIZE])
    }
}

#[derive(Defaults!)]
pub struct MainMemory {
    memory: [Byte; 10_0000],
}

pub type Cache20MB = Cache<20_000_000>;
pub type Cache20KB = Cache<20_000>;

#[derive(Debug, Clone, Copy, new)]
pub struct ExecutorComplexes([ExecutorComplex; 128]);

impl Default for ExecutorComplexes {
    fn default() -> Self {
        Self([Default::default(); 128])
    }
}

#[derive(Debug, Clone, Copy, Default, new)]
pub struct MainProcessingUnit {
    global_cache: Cache20KB,
    executor_complexes: ExecutorComplexes,
}

#[derive(Debug, Clone, Copy, new)]
pub struct CircularBuffer<T: Default + Copy, const SIZE: usize>([T; SIZE]);

impl<T: Default + Copy, const SIZE: usize> Default for CircularBuffer<T, SIZE> {
    fn default() -> Self {
        Self([T::default(); SIZE])
    }
}

#[derive(Debug, Clone, Copy, Default, new)]
pub struct Function {
    asid: u16,
    fn_id: u16,
    curr_addr_instruction: u64,
    curr_addr_sp: u64,
}

#[derive(Debug, Clone, Copy, Default, new)]
pub struct WaitList(CircularBuffer<Function, 1024>);

#[derive(Debug, Clone, Copy, Default, new)]
pub enum ExecutorType {
    #[default]
    IExecutor,
    DExecutor,
}

pub type StackPointer = u64;
pub type InstructionPointer = u64;

#[derive(Debug, Clone, Copy, Default, new)]
pub struct Executor(ExecutorType, StackPointer, InstructionPointer);

impl Executor {
    pub fn next_instruction(&mut self) -> SpectreInstruction {
        self.2 += size_of::<SpectreInstruction>() as u64;
        // read the instruction at the point
        unsafe { read_volatile(self.2 as *const SpectreInstruction) }
    }

    pub fn push_stack<V>(&mut self, value: V) {
        // store value on the stack then decrement
        unsafe { write_volatile(self.1 as *mut V, value) }
        self.1 -= size_of::<V>() as u64;
    }

    pub fn pop_stack<V>(&mut self, mut v: V) {
        // instead of reading at SP, read at SP + sizeof V
        let read_location = self.1 + size_of::<V>() as u64;
        v = unsafe { read_volatile(read_location as *const V) };
        self.1 += size_of::<V>() as u64;
    }

    // read from arbitrary address from other thing for stuff?

    // jump to some arbitrary address
    pub fn jump(&mut self, addr: u64) {
        self.2 = addr;
    }
}

#[derive(Debug, Clone, Copy, Default, new)]
pub enum ExecutorComplex {
    #[default]
    IComplex,
    DComplex,
}

#[derive(Debug, Clone, Copy, Default, new)]
pub enum AcceleratorUnit {
    #[default]
    Sha256,
    Discretizer,
    Lookup,
}

#[derive(Debug, Clone, Copy, Default, new)]
pub struct AcceleratorQueue(CircularBuffer<Function, 128>);

#[derive(Debug, Clone, Copy, new)]
pub struct IComplex {
    queue: CircularBuffer<Function, 256>,
    executors: [Executor; 64],
    accelerator_queue: AcceleratorQueue,
    local_cache: Cache20KB,
    accelerators: [AcceleratorUnit; 32],
}

#[derive(Debug, Clone, Copy, new)]
pub struct DComplex {
    queue: CircularBuffer<Function, 256>,
    executors: [Executor; 64],
}

impl Default for IComplex {
    fn default() -> Self {
        Self {
            queue: Default::default(),
            executors: [Default::default(); 64],
            accelerator_queue: Default::default(),
            local_cache: Default::default(),
            accelerators: Default::default(),
        }
    }
}

/*
    An executor is an FSM with instruction addr, stack addr
*/

impl Default for DComplex {
    fn default() -> Self {
        Self {
            queue: Default::default(),
            executors: [Default::default(); 64],
        }
    }
}

#[test]
fn test_mpu() {
    let mpu = MainProcessingUnit::default();
}

/*
    Runner
*/

/// Start a VM process and run it with a set of instructions
pub fn run_vm(instructions: Instructions) {
    let mpu = MainProcessingUnit::default();

    // scheduler?

    // eval_instructions(instructions.0);
}
