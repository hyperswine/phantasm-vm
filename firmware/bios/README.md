# Rei VM BIOS

A BIOS Software for ReiVM that compiles to Phantasm IR.

The BIOS is very small. It is meant to fit on the 64-512KB EEPROM and hardly flashed.
The BIOS firmware uses a subset of neutronfs driver code to implement a quick search for an arcboot image on a neutronfs partition on a bootable drive. It does so iteratively through each drive that is plugged in...
And for each neutronfs partition.

The BIOS firmware is less than 100 lines of code with 3 major functions. Which compiles to probably around a few KB in release mode with `config = low-memory-footprint`.
