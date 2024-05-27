use libmem::{
    enum_modules, enum_modules_ex, enum_processes, enum_segments, find_module, find_module_ex,
    find_process, find_segment, get_bits, get_process, get_process_ex, get_system_bits, get_thread,
    get_thread_ex, get_thread_process, is_process_alive, load_module, load_module_ex,
    unload_module, unload_module_ex,
};

pub fn main() {
    let processes = enum_processes().unwrap();
    println!("[*] Process Enumeration: ");
    println!(" - {}", processes.first().unwrap());
    println!("...");
    println!(" - {}", processes.last().unwrap());
    println!("Process Count: {}", processes.len());
    println!();

    let process = get_process().unwrap();
    println!("[*] Current Process: {}", process);

    let target_process = find_process("target").unwrap();
    println!("[*] Target Process: {}", target_process);

    println!(
        "[*] Target Process (got by PID): {}",
        get_process_ex(target_process.pid).unwrap()
    );

    println!(
        "[*] Is Target Process Alive? {}",
        is_process_alive(&target_process)
    );

    println!();

    println!("[*] Process Bits: {}", get_bits());
    println!("[*] System Bits: {}", get_system_bits());

    println!("================================");

    let thread = get_thread().unwrap();
    println!("[*] Current Thread: {}", thread);

    let target_thread = get_thread_ex(&target_process).unwrap();
    println!("[*] Target Thread: {}", target_thread);

    let thread_process = get_thread_process(&target_thread).unwrap();
    println!("[*] Target Thread Process: {}", thread_process);

    println!("================================");

    let modules = enum_modules().unwrap();
    println!("[*] Module Enumeration: ");
    println!(" - {}", modules.first().unwrap());
    println!("...");
    println!(" - {}", modules.last().unwrap());
    println!("Module Count: {}", modules.len());
    println!();

    let modules = enum_modules_ex(&target_process).unwrap();
    println!("[*] Target Module Enumeration: ");
    println!(" - {}", modules.first().unwrap());
    println!("...");
    println!(" - {}", modules.last().unwrap());
    println!("Module Count: {}", modules.len());
    println!();

    let module = find_module(&process.name).unwrap();
    println!("[*] Process Module: {}", module);

    let target_module = find_module_ex(&target_process, &target_process.name).unwrap();
    println!("[*] Target Process Module: {}", target_module);

    let libpath = format!(
        "{}/../../build/tests/libtest.so",
        std::env::current_dir().unwrap().display()
    );
    println!("[*] Library Path: {}", libpath);
    let loaded_module = load_module(&libpath).unwrap();
    println!("[*] Module Loaded: {}", loaded_module);
    unload_module(&loaded_module).unwrap();
    println!("[*] Unloaded Module");

    let target_loaded_module = load_module_ex(&target_process, &libpath).unwrap();
    println!("[*] Module Loaded in Target: {}", target_loaded_module);
    unload_module_ex(&target_process, &target_loaded_module).unwrap();
    println!("[*] Unloaded Module from Target Process");

    println!("================================");
}
