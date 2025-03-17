use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    let target = env::var("TARGET").unwrap();
    let out_dir = env::var("OUT_DIR").unwrap();
    
    // Compile x64 or x86 assembly files based on target
    if target.contains("x86_64") {
        // Assemble x64 files
        let entry_obj = format!("{}/entry.o", out_dir);
        let utils_obj = format!("{}/utils.o", out_dir);
        
        let status = Command::new("nasm")
            .args(&[
                "-f", "win64",
                "asm/x64/entry.asm",
                "-o", &entry_obj
            ])
            .status()
            .expect("Failed to assemble entry.x64.asm");
        
        if !status.success() {
            panic!("Failed to assemble entry.x64.asm");
        }
        
        let status = Command::new("nasm")
            .args(&[
                "-f", "win64",
                "asm/x64/utils.asm",
                "-o", &utils_obj
            ])
            .status()
            .expect("Failed to assemble utils.x64.asm");
        
        if !status.success() {
            panic!("Failed to assemble utils.x64.asm");
        }

        // Use cc to link the object files
        cc::Build::new()
            .object(&entry_obj)
            .object(&utils_obj)
            .compile("asm");
        
    } else if target.contains("i686") {
        // Assemble x86 files
        let entry_obj = format!("{}/entry.o", out_dir);
        let utils_obj = format!("{}/utils.o", out_dir);
        
        let status = Command::new("nasm")
            .args(&[
                "-f", "win32",
                "asm/x86/entry.asm",
                "-o", &entry_obj
            ])
            .status()
            .expect("Failed to assemble entry.x86.asm");
        
        if !status.success() {
            panic!("Failed to assemble entry.x86.asm");
        }
        
        let status = Command::new("nasm")
            .args(&[
                "-f", "win32",
                "asm/x86/utils.asm",
                "-o", &utils_obj
            ])
            .status()
            .expect("Failed to assemble utils.x86.asm");
        
        if !status.success() {
            panic!("Failed to assemble utils.x86.asm");
        }

        // Use cc to link the object files
        cc::Build::new()
            .object(&entry_obj)
            .object(&utils_obj)
            .compile("asm");
    } else {
        panic!("Unsupported target: {}", target);
    }
    
    // Tell cargo to rerun if assembly files change
    println!("cargo:rerun-if-changed=asm/x64/entry.asm");
    println!("cargo:rerun-if-changed=asm/x64/utils.asm");
    println!("cargo:rerun-if-changed=asm/x86/entry.asm");
    println!("cargo:rerun-if-changed=asm/x86/utils.asm");
    println!("cargo:rerun-if-changed=scripts/linker.ld");
}