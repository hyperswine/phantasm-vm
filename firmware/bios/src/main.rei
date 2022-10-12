use elf
use riscv
use arm

// log is implemented on core:: for riscv and arm at specific serial addresses
// you can reimpl or load your own logger with a custom serial addr or another method

# NOTE: the main fn in a core executable is becomes _main:, not _start:
# "command line arguments" aren't supported
main: () {
    // same bs in bios.pasm but also define your data structures so the IR knows what to do/what to load and such
    // NOTE: I dunno if its possible to compare really long strings directly. You basically have to split it up to 8B at a time and compare each slice

    // setup dram
    DRAM.start()

    // basic settings
    SP.set(0x4000_0001)
    // note: I dunno if you can just move each part? eh dw about it just copy for now since arcboot prob wont be too big anyway
    // confirm M-mode
    EXECUTION_MODE.set(Machine)
    // confirm default settings for cpu and ram
    if CPU.settings() == DEFAULT_SETTINGS => log::info("CPU is set to default settings")
    else => log::info("CPU is not set to default... Was this intended?")

    // basically, do what you expect

    // 1. check if there is a bootable drive
    let bootable_drives = drives.filter(d => can_boot(d))
    if bootable_drives.empty() => return

    // try to boot from the first drive. Usually, assumed that it contains the default NeFS partition
    bootable_drives.find(d => boot(d))
}

// allow anonymous object to statically checked. Can only be dynamically checked with std?
// actually no, just use the CPU settings object
const DEFAULT_SETTINGS = Cpu::Settings {
    // on arm, there is 2 tables, check those too
    translation_table_on = false
    // etc... exceptions and stuff
    exceptions_on = false
    nmi_exceptions_on = false
}

const ARCBOOT_IMAGE_PATH = "/sys/kernel/arcboot"
const NEUTRONFS_UUID = 64xNEUTRONFS_0000...

boot: (drive: Drive) -> Status {
    // if drive isnt GPT, then no boot
    drive.is_gpt() ?: return Err()

    // find the first bootable NeFS partition
    let nefs_partitions = drive.partitions.filter(p => p.UUID == NEUTRONFS_UUID)

    // find /sys/kernel/arcboot
    nefs_partitions.for_each(p => {
        // the find fn finds the file on disk and the copy() copies the contents into memory pointed to by SP
        let arcboot_image = neutronfs::file::find(ARCBOOT_IMAGE_PATH)?.copy()
        load(arcboot_image)
        // break is special instead an iterative method
        break
    })
}

load: (raw_image: Bytes) {
    // use the ELF parser to parse each section
    let elf_image = elf::parse(raw_image)

    let arcboot_code = elf.image.code_section()

    // load arcboot data and other sections into RAM in a basic way
    // NOTE: arcboot should be compiled to a position independent image where its .data is relative to its code
    // and all its sections are loaded into physical memory isomorphically
    arcboot_code.sections.for_each(s => s.load_section())

    // let arcboot take control (and set execution mode to H-Mode)
    // @asm jump(arcboot.entry)
    // it all uses the same M mode calling convention on riscv or arm
    let relative_addr_of_entry_fn = arcboot_code.entry() as *u64

    // jump by transmuting as a fn()
    // NOTE: in rei, the fn annotation is not required as tuples and fns are both callable
    (relative_addr_of_entry_fn as ())()
}

// might be in elf?
// load_section: (section: Section) {}
