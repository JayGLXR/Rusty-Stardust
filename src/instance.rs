use crate::constants::{END_OFFSET, MB_OK};
use crate::hash_str;
use crate::memory::symbol;
use crate::resolve;
use crate::windows::*;
use core::ffi::c_void;

// Type aliases for Windows API functions
type FnLoadLibraryA = unsafe extern "system" fn(lpLibFileName: PSTR) -> HMODULE;
type FnGetProcAddress = unsafe extern "system" fn(hModule: HMODULE, lpProcName: PSTR) -> PVOID;
type FnMessageBoxA = unsafe extern "system" fn(hWnd: PVOID, lpText: PSTR, lpCaption: PSTR, uType: u32) -> i32;

#[cfg(feature = "debug")]
type FnDbgPrint = unsafe extern "C" fn(Format: PCH, ...) -> i32;

// Keep resolve_import macro but mark it as #[allow(unused_macros)] since it might be needed later
#[allow(unused_macros)]
macro_rules! resolve_import {
    ($instance:expr, $module:ident) => {
        {
            let base_addr = $instance.$module.handle;
            if base_addr != 0 {
                $(
                    // For each API in the module struct, resolve it using its hash
                    $instance.$module.$api = core::mem::transmute(
                        resolve::_api(base_addr, $instance.$module.$api as usize)
                    );
                )*
            }
        }
    };
}

// Debug print macro
#[cfg(feature = "debug")]
#[macro_export]
macro_rules! dbg_printf {
    ($instance:expr, $fmt:expr, $($arg:expr),*) => {
        {
            unsafe {
                if !$instance.ntdll.DbgPrint.is_null() {
                    // Format like the original: [DEBUG::function::line] message
                    let prefix = symbol::<PCH>(concat!("[DEBUG::", file!(), "::", line!(), "] ").as_ptr());
                    let msg = symbol::<PCH>($fmt.as_ptr());
                    (*$instance.ntdll.DbgPrint)(prefix, msg, $($arg),*);
                }
            }
        }
    };
}

#[cfg(not(feature = "debug"))]
#[macro_export]
macro_rules! dbg_printf {
    ($instance:expr, $fmt:expr, $($arg:expr),*) => {
        {}
    };
}

// Main instance structure
pub struct Instance {
    // Base address and size
    pub base: BaseInfo,
    
    // Modules
    pub ntdll: NtdllModule,
    pub kernel32: Kernel32Module,
}

// Base information
pub struct BaseInfo {
    pub address: usize,
    pub length: usize,
}

// Ntdll module
pub struct NtdllModule {
    pub handle: usize,
    
    #[cfg(feature = "debug")]
    pub DbgPrint: *mut FnDbgPrint,
}

// Kernel32 module
pub struct Kernel32Module {
    pub handle: usize,
    pub LoadLibraryA: *mut FnLoadLibraryA,
    pub GetProcAddress: *mut FnGetProcAddress,
}

impl Instance {
    // Initialize the instance
    pub fn new() -> Self {
        unsafe {
            // Create instance with empty values
            let mut instance = Instance {
                base: BaseInfo {
                    address: 0,
                    length: 0,
                },
                
                ntdll: NtdllModule {
                    handle: 0,
                    #[cfg(feature = "debug")]
                    DbgPrint: core::ptr::null_mut(),
                },
                
                kernel32: Kernel32Module {
                    handle: 0,
                    LoadLibraryA: core::ptr::null_mut(),
                    GetProcAddress: core::ptr::null_mut(),
                },
            };
            
            // Calculate shellcode base address + size
            instance.base.address = crate::memory::RipStart();
            instance.base.length = (crate::memory::RipData() - instance.base.address) + END_OFFSET;
            
            // Resolve modules from PEB
            instance.ntdll.handle = resolve::module(hash_str!("ntdll.dll"));
            if instance.ntdll.handle == 0 {
                return instance;
            }
            
            instance.kernel32.handle = resolve::module(hash_str!("kernel32.dll"));
            if instance.kernel32.handle == 0 {
                return instance;
            }
            
            // Resolve imports
            #[cfg(feature = "debug")]
            {
                instance.ntdll.DbgPrint = core::mem::transmute(
                    resolve::_api(instance.ntdll.handle, hash_str!("DbgPrint") as usize)
                );
            }
            
            instance.kernel32.LoadLibraryA = core::mem::transmute(
                resolve::_api(instance.kernel32.handle, hash_str!("LoadLibraryA") as usize)
            );
            
            instance.kernel32.GetProcAddress = core::mem::transmute(
                resolve::_api(instance.kernel32.handle, hash_str!("GetProcAddress") as usize)
            );
            
            instance
        }
    }
    
    // Start the shellcode execution
    pub unsafe fn start(&self, _arg: *mut c_void) {
        // Load user32.dll
        let user32 = if !self.kernel32.LoadLibraryA.is_null() {
            unsafe { (*self.kernel32.LoadLibraryA)(symbol("user32.dll".as_ptr())) }
        } else {
            core::ptr::null_mut()
        };
        
        #[cfg(feature = "debug")]
        {
            if !user32.is_null() {
                dbg_printf!(self, "oh wow look we loaded user32.dll -> %p\n", user32);
            } else {
                dbg_printf!(self, "okay something went wrong. failed to load user32 :/\n",);
            }
            
            let peb = NtCurrentPeb();
            dbg_printf!(self, "running from %ls (Pid: %d)\n", 
                (*(*peb).ProcessParameters).ImagePathName.Buffer,
                (*NtCurrentTeb()).ClientId.UniqueProcess);
            
            dbg_printf!(self, "shellcode @ %p [%d bytes]\n", self.base.address, self.base.length);
        }
        
        // Resolve MessageBoxA and display message
        if !user32.is_null() {
            let msgbox: *mut FnMessageBoxA = core::mem::transmute(
                resolve::_api(user32 as usize, hash_str!("MessageBoxA") as usize)
            );
            
            if !msgbox.is_null() {
                unsafe {
                    (*msgbox)(
                        core::ptr::null_mut(),
                        symbol("Hello world".as_ptr()),
                        symbol("caption".as_ptr()),
                        MB_OK
                    );
                }
            }
        }
    }
}