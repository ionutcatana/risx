use std::process::Command;

const BINUTILS_MISSING: &str = "binutils-riscv64-unknown-elf toolchain is required to build risx";

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    assemble_libsbi(&out_dir);
    assemble_entry(&out_dir);

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rustc-link-search=native={}", out_dir);

    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let ld_script = format!("{}/src/risx.ld", manifest_dir);
    println!("cargo:rerun-if-changed={}", ld_script);
    println!("cargo:rustc-link-arg=-T{}", ld_script);
}

fn assemble_libsbi(out_dir: &str) {
    let mut assembler = Command::new("riscv64-unknown-elf-as");
    assembler
        .args(&[
            "-march=rv64imac",
            "-mabi=lp64",
            "-o",
            &format!("{}/sbi.o", out_dir),
            "src/riscv/sbi.s",
        ])
        .status()
        .expect(BINUTILS_MISSING);

    let mut archiver = Command::new("riscv64-unknown-elf-ar");
    archiver
        .args(&[
            "crus",
            &format!("{}/libsbi.a", out_dir),
            &format!("{}/sbi.o", out_dir),
        ])
        .status()
        .expect(BINUTILS_MISSING);

    println!("cargo:rerun-if-changed=src/riscv/sbi.s");
    println!("cargo:rustc-link-lib=static=sbi");
}

fn assemble_entry(out_dir: &str) {
    let mut assembler = Command::new("riscv64-unknown-elf-as");
    assembler
        .args(&[
            "-march=rv64imac",
            "-mabi=lp64",
            "-o",
            &format!("{}/entry.o", out_dir),
            "src/riscv/entry.s",
        ])
        .status()
        .expect(BINUTILS_MISSING);

    let mut archiver = Command::new("riscv64-unknown-elf-ar");
    archiver
        .args(&[
            "crus",
            &format!("{}/libentry.a", out_dir),
            &format!("{}/entry.o", out_dir),
        ])
        .status()
        .expect(BINUTILS_MISSING);

    println!("cargo:rerun-if-changed=src/riscv/entry.s");
    println!("cargo:rustc-link-lib=static=entry");
}
