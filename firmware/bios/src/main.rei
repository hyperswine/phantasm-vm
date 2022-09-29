# NOTE: the main fn in a core executable is becomes _main:, not _start:
# "command line arguments" aren't supported
main: () {
    // same bs in bios.pasm but also define your data structures so the IR knows what to do/what to load and such
    // NOTE: I dunno if its possible to compare really long strings directly. You basically have to split it up to 8B at a time and compare each slice
}
