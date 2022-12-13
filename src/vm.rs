/*
    It just works
*/

use derive_new::new;
use phantasm_ir::spectre_ir::SpectreInstruction;
use std::{
    intrinsics::size_of,
    ptr::{read_volatile, write_volatile},
};

derive_alias! {
    #[derive(Defaults!)] = #[derive(Debug, Clone, Copy, new)];
    #[derive(Complete!)] = #[derive(Debug, Clone, Copy, Default, new)];
}

pub type Byte = u8;

pub trait HighLevelPrint {
    fn print_details(&self);
}

/*
    Could try to emulate the speed of each cache, but nah
    Wait cant we just use the main memory for everything then? Yea cause its just SP
    Well it might still help depending on how... for now just keep it like this...
*/

/*
    Full System
*/

pub struct PhantasmSystem {
    mpu: MainProcessingUnit,
    main_memory: MainMemory
}

pub fn phantasm_system() -> PhantasmSystem {
    PhantasmSystem { mpu: new_mpu(), main_memory: new_main_memory_2_gigabytes() }
}

pub fn new_local_cache_20_kilobytes() -> LocalCache {
    vec![0; 20_000]
}

pub fn new_universal_cache_20_megabytes() -> UniversalCache {
    vec![0; 20_000_000]
}

pub type UniversalCache = Memory;
pub type LocalCache = Memory;

pub type Memory = Vec<Byte>;
pub type MainMemory = Memory;

pub fn new_main_memory_2_gigabytes() -> MainMemory {
    vec![0; 2_000_000_000]
}

#[derive(Debug, Clone, new)]
pub struct MainProcessingUnit {
    global_cache: UniversalCache,
    executor_complexes: Vec<ExecutorComplex>,
}

impl HighLevelPrint for MainProcessingUnit {
    fn print_details(&self) {
        println!("MPU STATs");
        println!("global_cache length = {:?}", self.global_cache.len());
        println!(
            "executor_complexes count = {}",
            self.executor_complexes.len()
        );
        // status active
        self.executor_complexes
            .iter()
            .for_each(|c| println!("status: Active"));
    }
}

/// Create an MPU with 8 executor complexes, 4 I and 4 D
pub fn new_mpu() -> MainProcessingUnit {
    // half instruction, half data, alternating
    // let executor_complexes = vec![ExecutorComplex];
    let mut complexes = Vec::<ExecutorComplex>::new();
    for i in 0..8 {
        let complex = if i % 2 == 0 {
            ExecutorComplex::IComplex
        } else {
            ExecutorComplex::DComplex
        };
        complexes.push(complex)
    }
    MainProcessingUnit {
        global_cache: new_universal_cache_20_megabytes(),
        executor_complexes: complexes,
    }
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
    HExecutor,
}

pub type StackPointer = u64;
pub type InstructionPointer = u64;

/// A general purpose executor that executes a certain instruction, either type I or D
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

#[derive(Debug, Clone, new)]
pub struct IComplex {
    queue: CircularBuffer<Function, 256>,
    executors: [Executor; 64],
    accelerator_queue: AcceleratorQueue,
    local_cache: LocalCache,
    accelerators: [AcceleratorUnit; 32],
}

#[derive(Debug, Clone, Copy, new)]
pub struct DComplex {
    queue: CircularBuffer<Function, 256>,
    executors: [Executor; 64],
    accelerators: [AcceleratorUnit; 32],
}

impl Default for IComplex {
    fn default() -> Self {
        Self {
            queue: Default::default(),
            executors: [Default::default(); 64],
            accelerator_queue: Default::default(),
            local_cache: new_local_cache_20_kilobytes(),
            accelerators: Default::default(),
        }
    }
}

impl Default for DComplex {
    fn default() -> Self {
        Self {
            queue: Default::default(),
            executors: [Default::default(); 64],
            accelerators: [Default::default(); 32],
        }
    }
}

/*
    Testing
*/

#[test]
fn test_mpu() {
    let mpu = new_mpu();
    mpu.print_details();
}
