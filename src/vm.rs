use derive_new::new;

/*
    High Level VM
    Features:
    - function pools akin to thread pools
    - inf executors akin to hardware cores
    - multi process, stack based. Allows configuration with ASID tagging of functions
*/

pub type Byte = u8;
pub type Memory = Vec<Byte>;

pub fn new_main_memory_2_gigabytes() -> Memory {
    vec![0; 2_000_000_000]
}

pub struct PhantasmSystem {
    main_memory: Memory,
}

/*
    Utility
*/

derive_alias! {
    #[derive(Defaults!)] = #[derive(Debug, Clone, Copy, new)];
    #[derive(Complete!)] = #[derive(Debug, Clone, Copy, Default, new)];
}

// could also just impl Display for PhantasmSystem
pub trait MachineStateDisplay {
    fn print_details(&self);
}
