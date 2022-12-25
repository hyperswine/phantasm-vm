/*
    High Level VM
    Features:
    - function pools akin to thread pools
    - inf executors akin to hardware cores
    - multi process, stack based. Allows configuration with ASID tagging of functions
*/

use derive_new::new;
use std::{
    intrinsics::size_of,
    ptr::{read_volatile, write_volatile},
};

derive_alias! {
    #[derive(Defaults!)] = #[derive(Debug, Clone, Copy, new)];
    #[derive(Complete!)] = #[derive(Debug, Clone, Copy, Default, new)];
}

pub type Byte = u8;

pub type UniversalCache = Memory;
pub type LocalCache = Memory;

pub type Memory = Vec<Byte>;
pub type MainMemory = Memory;

pub fn new_main_memory_2_gigabytes() -> MainMemory {
    vec![0; 2_000_000_000]
}

pub struct PhantasmSystem {
    main_memory: MainMemory,
}

/*
    Utility
*/

pub trait HighLevelPrint {
    fn print_details(&self);
}
