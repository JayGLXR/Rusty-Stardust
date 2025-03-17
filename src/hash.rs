use crate::constants::{FNV1A_BASIS, FNV1A_PRIME};

// Runtime hash function for strings
pub unsafe fn hash_string(string: *const u8) -> u32 {
    let mut hash = FNV1A_BASIS;
    let mut ptr = string;
    
    while *ptr != 0 {
        let mut byte = *ptr;
        
        // Convert to uppercase if lowercase
        if byte >= b'a' {
            byte -= 0x20;
        }
        
        hash ^= byte as u32;
        hash = hash.wrapping_mul(FNV1A_PRIME);
        
        ptr = ptr.add(1);
    }
    
    hash
}

// Runtime hash function for wide strings
pub unsafe fn hash_string_wide(string: *const u16) -> u32 {
    let mut hash = FNV1A_BASIS;
    let mut ptr = string;
    
    while *ptr != 0 {
        let mut byte = (*ptr & 0xFF) as u8;
        
        // Convert to uppercase if lowercase
        if byte >= b'a' {
            byte -= 0x20;
        }
        
        hash ^= byte as u32;
        hash = hash.wrapping_mul(FNV1A_PRIME);
        
        ptr = ptr.add(1);
    }
    
    hash
}

// Compile-time hash function for const evaluation
#[allow(dead_code)]
pub const fn hash_const(s: &str) -> u32 {
    let bytes = s.as_bytes();
    let mut hash = FNV1A_BASIS;
    let mut i = 0;
    
    while i < bytes.len() {
        let mut byte = bytes[i];
        
        // Convert to uppercase if lowercase
        if byte >= b'a' {
            byte -= 0x20;
        }
        
        hash ^= byte as u32;
        hash = hash.wrapping_mul(FNV1A_PRIME);
        
        i += 1;
    }
    
    hash
}

// Macro for creating compile-time hashes
#[macro_export]
macro_rules! hash_str {
    ($s:expr) => {
        $crate::hash::hash_const($s)
    };
}