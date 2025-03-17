//! Memory manipulation functions

// Zero memory (Rust's equivalent of RtlSecureZeroMemory)
pub unsafe fn zero(memory: *mut u8, length: u32) {
    for i in 0..length {
        *memory.offset(i as isize) = 0;
    }
}

// Memory copy function 
pub unsafe fn copy(destination: *mut u8, source: *const u8, length: u32) -> *mut u8 {
    for i in 0..length {
        *destination.offset(i as isize) = *source.offset(i as isize);
    }
    destination
}

// Memory compare function
pub unsafe fn compare(memory1: *const u8, memory2: *const u8, length: usize) -> u32 {
    let mut a = memory1;
    let mut b = memory2;
    let mut len = length;
    
    while len > 0 {
        let val1 = *a;
        let val2 = *b;
        
        if val1 != val2 {
            return (val1 as i32 - val2 as i32) as u32;
        }
        
        a = a.offset(1);
        b = b.offset(1);
        len -= 1;
    }
    
    0
}

// Get string from RIP-relative addressing
pub unsafe fn symbol<T>(s: *const u8) -> T {
    let rip_data = external_rip_data();
    let offset = (s as usize).wrapping_sub(rip_data_fn_addr() as usize);
    let absolute_addr = rip_data.wrapping_sub(offset);
    
    core::mem::transmute_copy(&absolute_addr)
}

// Use the external RipStart/RipData functions from lib.rs
extern "C" {
    // These are now public functions
    pub fn RipStart() -> usize;
    pub fn RipData() -> usize;
}

// Helper functions to get addresses
fn rip_data_fn_addr() -> usize {
    RipData as usize
}

fn external_rip_data() -> usize {
    unsafe { RipData() }
}

// Macro to define the RangeHeadList loop, improved version
#[macro_export]
macro_rules! range_head_list {
    ($head_list:expr, $type:ty, |$current:ident| $body:block) => {
        {
            let head_ptr = $head_list as *const LIST_ENTRY;
            let mut $current = (*head_ptr).Flink as $type;
            
            while $current as *const _ != head_ptr as *const _ {
                $body
                $current = (*$current).InLoadOrderLinks.Flink as $type;
            }
        }
    };
}