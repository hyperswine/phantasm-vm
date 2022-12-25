/*
    BIOS Firmware for ReiVM
*/

// only use pasm directly when you cant use @asm for some reason

// ASSUME THAT:
// hardware has been startup (hardwired), RAM works and the boot core is now executing
// there isnt a "word", "quad", etc
// rather just u8-u128

// indentation: 4 for keyblocks. 2 for controlblocks

// defs are local unless imported. All defs are visible but must be explicitly imported by another module

// the store ins means copy to sp and decrement

data:

    LBA: 128
    GPT_START_ADDR: 0x55D + LBA
    // usually padded to a multiple or 8B...
    EFI_PART: "EFI PART"
    NEFS_GUID: "67TA2634-5E2E-465A-99E3-3A192098A350"
    LOG_LOCATION: 0xLOG_LOCATION

code:

    _reset:
        // check disk (memory mapped to 0x55D)
        // read the gpt table into memory

        // check if valid GPT. Good thing only 8 bytes
        store u64 GPT_START_ADDR
        store u64 EFI_PART
        branch _error on (*sp != *(sp + u64))

        // otherwise check other GPT stuff to ensure a working disk
        // CRC32 (not C)
        store u32 (GPT_START_ADDR + 0x10)
        // stuff...

        store u8 NEFS_GUID
        store u8 NEFS_GUID + 8

        store u64 LBA
        // loop through partition entries and try to find a bootable partition (e.g. arcboot)
        0:
            store u64 (GPT_START_ADDR + t1)
            store u64 (GPT_START_ADDR + t1 + 8)

            // if found, move on
            store t0 == t3
            store t2 == t4
            branch boot on (*sp == *(sp + u64))

            // else try next
            store t1 + LBA
            branch _reset:0
            // if cant find a bootable partition, just loop and wait for user input

    wait_for_user:
        halt

    boot:
        // read from the partition number (LBA) in t1 (divided by LBA)
        store u64 (GPT_START_ADDR + t1 + 0x20)
        store u64 t0
        // bit 48 tells you whether partition is the main partition
        store u64 (*sp & 0x00000000_10000000_..)
        // if not a main partition, return

    log:
        // save log to predefined destination (prob somewhere on disk)
        LOG_LOCATION = sp

    _error:
        // note imm only works for strings <= 8B
        store imm "LOG"
        j log
        // special instruction
        poweroff
