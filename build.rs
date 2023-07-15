use std::env;

/*
 * This build script verifies that the target system has the desired features.
 * This file verifies support for the following features:
 *  - atomics
 */

struct Atomics {
    a32: bool,
    a64: bool,
}

impl Atomics {
    fn a32(&self) -> bool {
        self.a32
    }

    fn disable_atomics_32(&mut self) {
        self.set_atomics_32(false);
    }

    fn set_atomics_32(&mut self, flag: bool) {
        self.a32 = flag;
    }

    fn a64(&self) -> bool {
        self.a64
    }

    fn disable_atomics_64(&mut self) {
        self.set_atomics_64(false);
    }

    fn set_atomics_64(&mut self, flag: bool) {
        self.a64 = flag;
    }
}

impl Default for Atomics {
    fn default() -> Self {
        Atomics { a32: true, a64: true }
    }
}

// struct Processor {
//     atomics: Atomics,
// }

// impl Processor {
//     pub fn new(atomics: Atomics) -> Self {
//         Self { atomics }
//     }

//     fn disable_atomics_64(&mut self) {
//         self.atomics.disable_atomics_64()
//     }
// }

fn main() {
    let mut atomics = Atomics::default();
    let target = env::var("TARGET").unwrap();

    // Full target triples that have specific limitations:
    match target.as_str() {
        "arm-linux-androideabi"
        | "asmjs-unknown-emscripten"
        | "wasm32-unknown-emscripten"
        | "wasm32-unknown-unknown" => atomics.disable_atomics_64(),
        _ => {}
    }

    // Architecture-specific limitations:
    let arch = target.split('-').next().unwrap_or(&target);
    match arch {
        // NOTE: Not all ARMv7 variants are listed here, as certain variants do actually provide
        // 64-bit atomics. (`armv7`, `armv7a`, and `armv7s`, specifically)
        "armv5te" | "mips" | "mipsel" | "powerpc" | "riscv32imac" | "thumbv7em" | "thumbv7m"
        | "thumbv8m.base" | "thumbv8m.main" | "armebv7r" | "armv7r" => atomics.disable_atomics_64(),
        "avr" | "riscv32i" | "riscv32im" | "riscv32imc" | "thumbv6m" => {
            atomics.disable_atomics_32();
            atomics.disable_atomics_64();
        }
        _ => {}
    }

    if atomics.a64() {
        println!("cargo:rustc-cfg=has_atomics_64");
    }

    if atomics.a32() {
        println!("cargo:rustc-cfg=has_atomics");
    }
}
