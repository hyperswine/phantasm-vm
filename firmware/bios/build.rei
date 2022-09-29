require = {
    core = { version = "0.1" }
}

target = {
    // to target IR (.pasm), reic <files> --target phantasm_ir
    // extra settings. NOTE: maybe allow the VM to link against this directly? Or include this
    phantasm_ir = {}
}
