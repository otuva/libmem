use libmem::*;

fn separator() {
    println!("====================");
}

fn main() {
    println!("[*] libmem-rs tests");

    separator();

    println!("[*] Process Enumeration");

    let mut counter = 0;
    for proc in LM_EnumProcesses() {
        println!("{}", proc);
        counter += 1;
        if counter >= 5 {
            break;
        }
    }

    separator();

    let cur_proc = LM_GetProcess().unwrap();
    println!("[*] Current Process: {}", cur_proc);

    separator();

    let proc = LM_FindProcess("firefox-esr").unwrap();
    println!("[*] Remote Process: {}", proc);

    separator();

    let is_alive = LM_IsProcessAlive(&proc);
    println!("[*] Is the remote process alive? {}", if is_alive { "yes" } else { "no" });

    separator();

    let sysbits = LM_GetSystemBits();
    println!("[*] System Bits: {}", sysbits);

    separator();

    print!("[*] Current Process Threads: ");
    for tid in LM_EnumThreadIds() {
        print!("{} ", tid);
    }
    println!();

    separator();

    print!("[*] Remote Process Threads: ");
    for tid in LM_EnumThreadIdsEx(&proc) {
        print!("{} ", tid);
    }
    println!();

    separator();

    println!("[*] Current Thread ID: {}", LM_GetThreadId());
    println!("[*] Remote Process Thread ID: {}", LM_GetThreadIdEx(&proc));

    separator();

    println!("[*] Current Process - Module Enumeration");
    let mut counter = 0;
    for module in LM_EnumModules() {
        println!("{}", module);

        counter += 1;
        if counter >= 5 {
            break;
        }
    }

    separator();

    println!("[*] Remote Process - Module Enumeration");
    let mut counter = 0;
    for module in LM_EnumModulesEx(&proc) {
        println!("{}", module);

        counter += 1;
        if counter >= 5 {
            break;
        }
    }

    separator();

    let cur_module = LM_FindModule(&cur_proc.get_name()).unwrap();
    println!("[*] Current Process Module: {}", cur_module);

    separator();

    let module = LM_FindModuleEx(&proc, "firefox-esr").unwrap();
    println!("[*] Remote Process Module: {}", module);

    separator();

    let module = LM_LoadModule("/usr/local/lib/libtest.so").unwrap();
    println!("[*] Module Loaded into Current Process: {}", module);

    separator();

    // TODO: Test LM_LoadModuleEx
    
    // separator();

    // Needs internal fixing
    // LM_UnloadModule(&module).unwrap();
    // println!("[*] Unloaded Module from Current Process: {}", module);

    // separator();

    // TODO: Test LM_UnloadModuleEx

    // separator();
}
