use std::fs;
use std::mem;
use std::path::Path;
use std::ptr;
use std::env;
use windows::Win32::System::Memory::{
    VirtualAlloc, VirtualProtect, MEM_COMMIT, MEM_RESERVE, PAGE_EXECUTE_READWRITE, PAGE_READWRITE
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Determine architecture
    let args: Vec<String> = env::args().collect();
    let bin_path = if args.len() > 1 {
        args[1].clone()
    } else {
        #[cfg(target_arch = "x86_64")]
        let bin_name = "stardust.x64.bin";
        
        #[cfg(target_arch = "x86")]
        let bin_name = "stardust.x86.bin";
        
        format!("../bin/{}", bin_name)
    };

    println!("Loading shellcode from: {}", bin_path);
    
    // Read shellcode from file
    let shellcode = fs::read(Path::new(&bin_path))?;
    let shellcode_len = shellcode.len();
    
    println!("Shellcode size: {} bytes", shellcode_len);
    
    unsafe {
        // Allocate memory for the shellcode
        let memory = VirtualAlloc(
            ptr::null(), 
            shellcode_len, 
            MEM_COMMIT | MEM_RESERVE, 
            PAGE_READWRITE
        );
        
        if memory.is_null() {
            return Err("Failed to allocate memory".into());
        }
        
        println!("Memory allocated at: {:p}", memory);
        
        // Copy shellcode to allocated memory
        ptr::copy_nonoverlapping(
            shellcode.as_ptr(), 
            memory.cast::<u8>(), 
            shellcode_len
        );
        
        // Make memory executable
        let mut old_protect = PAGE_READWRITE;
        VirtualProtect(
            memory, 
            shellcode_len, 
            PAGE_EXECUTE_READWRITE, 
            &mut old_protect
        )?;
        
        println!("Memory protection changed to PAGE_EXECUTE_READWRITE");
        
        // Create function pointer and execute shellcode
        let shellcode_fn: extern "C" fn(arg: *const u8) = mem::transmute(memory);
        
        println!("Executing shellcode...");
        shellcode_fn(ptr::null());
        
        println!("Shellcode execution completed");
    }
    
    Ok(())
}