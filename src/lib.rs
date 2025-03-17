#![no_std]
#![no_main]
#![allow(non_snake_case)]

// Import modules
mod constants;
mod hash;
mod instance;
mod memory;
mod resolve;
mod windows;

// Re-export key components but exclude memory to avoid shadowing
pub use constants::*;
pub use hash::*;
pub use instance::*;
// Don't re-export memory to avoid shadowing the assembly functions
pub use resolve::*;
pub use windows::*;

use core::ffi::c_void;
use core::panic::PanicInfo;

// External assembly functions (defined in our assembly files)
// These are now properly declared only here
#[link(name = "asm")]
extern "C" {
    pub fn RipStart() -> usize;
    pub fn RipData() -> usize;
}

// Shellcode entry point - matches the assembly entry point
#[no_mangle]
pub unsafe extern "C" fn entry(args: *mut c_void) {
    // Create instance and start execution
    let instance = instance::Instance::new();
    instance.start(args);
}

// Panic handler that does nothing
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Exit handler for #![no_std]
#[no_mangle]
pub unsafe extern "C" fn __aeabi_unwind_cpp_pr0() {
    loop {}
}