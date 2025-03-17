# Rusty-Stardust

A modern and easy to use 32/64-bit shellcode template, ported to Rust from the original [Stardust](https://github.com/Cracked5pider/Stardust) project by [C5pider](https://twitter.com/C5pider).

![8D0BEA19-E728-42F4-A625-797765F4993B_1_201_a](https://github.com/user-attachments/assets/f687aa32-d9f1-46f0-9216-b139cbe344ac)


## About

This project is a Rust implementation of the Stardust design, which provides a modern approach to position-independent code development. The original Stardust project introduced several key innovations in shellcode design:

- Separation of code and data into distinct sections
- Global instance access in position-independent code
- Raw string embedding capabilities
- Compile-time string hashing for stealth
- Exception handling and debug information stripping

Rusty-Stardust maintains these core features while leveraging Rust's safety guarantees and modern tooling. This implementation serves as both a practical tool and an educational resource for those interested in learning about position-independent code development in Rust.

## Features

- Raw string embedding in shellcode
- Compile-time string hashing with fnv1a for both function and module resolution
- No_std implementation for minimal footprint
- Cross-compilation for both x86 and x64 Windows targets
- Docker-based build environment for cross-platform development
- Position-independent code using RIP-relative addressing
- Exception handling and debug information stripping for minimal footprint
- Proper MinGW toolchain integration for reliable shellcode extraction

## Building

### Prerequisites

- Docker (for cross-compilation)
- Rust (for local development)
- NASM (assembled automatically in Docker)
- MinGW toolchain (provided in Docker environment)

### Building with Docker

Build for release:
```shell
$ make
```

Build for debug mode (with DbgPrint support):
```shell
$ make debug
```

### Build Outputs

Output files will be placed in the `bin` directory:
- `stardust.x64.bin`: 64-bit shellcode (586 bytes)
- `stardust.x86.bin`: 32-bit shellcode (8.2KB)

The shellcode is extracted using MinGW's objcopy tool and contains:
- Stack setup code
- DLL loading functionality
- Function resolution code
- Architecture-specific instructions

### Build Configuration

The project uses several key configurations for reliable shellcode generation:

1. **Linker Script** (`scripts/linker.ld`):
   - Discards exception handling sections (`.pdata`, `.xdata`)
   - Removes debug information (`.debug*`)
   - Strips unnecessary sections (`.eh_frame`, `.note*`, `.comment*`)

2. **Cargo Config** (`.cargo/config.toml`):
   - Position Independent Code (PIC) enabled
   - Structured Exception Handling (SEH) disabled
   - Windows subsystem configuration
   - Custom linker settings for both architectures

3. **Assembly Integration**:
   - Assembly files are compiled with NASM
   - RIP-relative addressing for position independence
   - Proper section alignment and linking

## Usage Example

### Module Resolution

Resolving modules from PEB:
```rust
// Resolve modules from PEB using hash
let ntdll_hash = hash_str!("ntdll.dll");
let ntdll_handle = resolve::module(ntdll_hash);

let kernel32_hash = hash_str!("kernel32.dll");
let kernel32_handle = resolve::module(kernel32_hash);
```

### API Resolution

Resolving function APIs:
```rust
// Resolve LoadLibraryA from kernel32
let load_library_fn: *mut FnLoadLibraryA = resolve::api(
    kernel32_handle,
    hash_str!("LoadLibraryA") as usize
);

// Load user32.dll
let user32 = unsafe { (*load_library_fn)(symbol("user32.dll".as_ptr())) };

// Resolve MessageBoxA from user32
let msgbox_fn: *mut FnMessageBoxA = resolve::api(
    user32 as usize,
    hash_str!("MessageBoxA") as usize
);

// Display message box
unsafe {
    (*msgbox_fn)(
        core::ptr::null_mut(),
        symbol("Hello world".as_ptr()),
        symbol("caption".as_ptr()),
        MB_OK
    );
}
```

### Debugging Output

When built in debug mode:
```rust
#[cfg(feature = "debug")]
{
    dbg_printf!(instance, "Shellcode @ %p [%d bytes]\n", base_addr, size);
}
```

## Testing

A test program called "stomper" is included to load and execute the shellcode:

```shell
$ cd tests
$ cargo build
$ ./target/debug/stomper ../bin/stardust.x64.bin
```

## Architecture

This Rust port maintains the same architecture as the original C++ version:
- Assembly files are kept intact and linked using build.rs
- Memory layouts match the original for PE file parsing
- Windows structures are defined in Rust to match their C/C++ counterparts
- Function resolution uses the same hash-based approach for stealth
- MinGW toolchain integration for reliable shellcode extraction
- Exception handling and debug information stripping for minimal footprint

## Credits

This project is a Rust implementation of the original Stardust project. Special thanks to:

- [C5pider](https://twitter.com/C5pider) for the original Stardust design and implementation
- [Modexp](https://twitter.com/modexpblog) for his work on Windows PIC
- [Austin Hudson](https://twitter.com/ilove2pwn_) for his work on titanldr-ng
- [Kyle Avery](https://twitter.com/kyleavery_) for his work on AceLdr
- [x86matthew](https://twitter.com/x86matthew) for assembly guidance
- [mrexodia](https://twitter.com/mrexodia) for linker script insights

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

The original Stardust project and its design concepts are the work of [C5pider](https://github.com/Cracked5pider/Stardust). This Rust implementation is provided as an educational resource and alternative implementation for those interested in learning about position-independent code development in Rust.
