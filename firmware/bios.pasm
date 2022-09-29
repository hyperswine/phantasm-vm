#*
    BIOS Firmware for ReiVM
*#

// only use pasm directly when you cant use @asm for some reason

// ASSUME THAT:
// hardware has been startup (hardwired), RAM works and the boot core is now executing
// there isnt a "word", "quad", etc
// rather just u8-u128

// defs are local unless imported. All defs are visible but must be explicitly imported by another module

@data
def LBA = 128
def GPT_START_ADDR = 0x55D + LBA
def EFI_PART = "EFI PART"
def NEFS_GUID = "67TA2634-5E2E-465A-99E3-3A192098A350"

@code
_start:
// check disk (memory mapped to 0x55D)
// read the gpt table into memory

// check if valid GPT. Good thing only 8 bytes
t0 = u64 GPT_START_ADDR
t1 = u64 EFI_PART
j _error on t0 != t1

// otherwise check other GPT stuff to ensure a working disk
// CRC32 (not C)
t0 = u32 (GPT_START_ADDR + 0x10)
...

t3 = u8 NEFS_GUID
t4 = u8 NEFS_GUID + 8

t1 = u64 LBA
// loop through partition entries and try to find a bootable partition (e.g. arcboot)
0:
t0 = u64 (GPT_START_ADDR + t1)
t2 = u64 (GPT_START_ADDR + t1 + 8)

// if found, move on
t3 = t0 == t3
t4 = t2 == t4
branch boot on t3 == t4

// else try next
t1 = t1 + LBA
j local 0
// if cant find a bootable partition, just loop and wait for user input

wait_for_user:
halt

boot:
// read from the partition number (LBA) in t1 (divided by LBA)
t0 = u64 (GPT_START_ADDR + t1 + 0x20)
t0 = u64 t0
// bit 48 tells you whether partition is the main partition
t0 = t0 & 0x00000000_10000000_...
// if not a main partition, return

_error:
// note imm only works for strings <= 8B
a0 = imm "LOG"
j log
poweroff
