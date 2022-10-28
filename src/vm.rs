/*
    It just works
*/

use derive_new::new;

pub type Byte = u8;

#[derive(Debug, Clone, Copy, new)]
pub struct Cache<const SIZE: usize>([Byte; SIZE]);

impl<const SIZE: usize> Default for Cache<SIZE> {
    fn default() -> Self {
        Self([0 as Byte; SIZE])
    }
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
pub struct Executor;

#[derive(Debug, Clone, Copy, new)]
pub struct ExecutorComplex {
    queue: CircularBuffer<Function, 256>,
    executors: [Executor; 64],
}

impl Default for ExecutorComplex {
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
