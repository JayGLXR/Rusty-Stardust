#![allow(dead_code)]

// FNV1a hash constants
pub const FNV1A_PRIME: u32 = 0x01000193;
pub const FNV1A_BASIS: u32 = 0x811c9dc5;

// Constants for shellcode
#[cfg(target_arch = "x86_64")]
pub const END_OFFSET: usize = 0x10;

#[cfg(target_arch = "x86")]
pub const END_OFFSET: usize = 0x10;

// Windows constants
pub const IMAGE_DOS_SIGNATURE: u16 = 0x5A4D;
pub const IMAGE_NT_SIGNATURE: u32 = 0x00004550;
pub const IMAGE_DIRECTORY_ENTRY_EXPORT: usize = 0;
pub const MB_OK: u32 = 0x00000000;