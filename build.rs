use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=wrapper.h");

    let manifest = env::var("CARGO_MANIFEST_DIR").unwrap();
    let outdir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // todo pure rust or make sure to use cross platform commands
    Command::new("curl")
        .args(&[
            "-L",
            "https://github.com/ARM-software/CMSIS_5/releases/download/5.7.0/ARM.CMSIS.5.7.0.pack",
            "--output",
            outdir.join("CMSIS.zip").to_str().unwrap(),
        ])
        .output()
        .expect("curl CMSIS failed");

    Command::new("unzip")
        .args(&[
            outdir.join("CMSIS.zip").to_str().unwrap(),
            "-d",
            outdir.join("CMSIS").to_str().unwrap(),
        ])
        .output()
        .expect("unzip CMSIS failed");

    let manifest_dir = Path::new(&manifest);

    // * Here is the list of pre-built libraries :
    // * - arm_cortexM7lfdp_math.lib (Cortex-M7, Little endian, Double Precision Floating Point Unit)
    // * - arm_cortexM7lfsp_math.lib (Cortex-M7, Little endian, Single Precision Floating Point Unit)
    // * - arm_cortexM7l_math.lib (Cortex-M7, Little endian)
    // * - arm_cortexM4lf_math.lib (Cortex-M4, Little endian, Floating Point Unit)
    // * - arm_cortexM4l_math.lib (Cortex-M4, Little endian)
    // * - arm_cortexM3l_math.lib (Cortex-M3, Little endian)
    // * - arm_cortexM0l_math.lib (Cortex-M0 / Cortex-M0+, Little endian)
    // * - arm_ARMv8MBLl_math.lib (Armv8-M Baseline, Little endian)
    // * - arm_ARMv8MMLl_math.lib (Armv8-M Mainline, Little endian)
    // * - arm_ARMv8MMLlfsp_math.lib (Armv8-M Mainline, Little endian, Single Precision Floating Point Unit)
    // * - arm_ARMv8MMLld_math.lib (Armv8-M Mainline, Little endian, DSP instructions)
    // * - arm_ARMv8MMLldfsp_math.lib (Armv8-M Mainline, Little endian, DSP instructions, Single Precision Floating Point Unit)
    let target = env::var("TARGET").unwrap();
    let lib = match target.as_ref() {
        //Bare Cortex-M0, M0+, M1
        "thumbv6m-none-eabi" => "arm_cortexM0l_math",
        //Bare Cortex-M3
        "thumbv7m-none-eabi" => "arm_cortexM3l_math",
        //Bare Cortex-M4, M7
        "thumbv7em-none-eabi" => "arm_cortexM4l_math",
        //Bare Cortex-M4F, M7F, FPU, hardfloat
        "thumbv7em-none-eabihf" => "arm_cortexM4lf_math",
        _ => panic!("no known arm math library for target {}", target),
    };

    // Link against prebuilt cmsis math
    println!(
        "cargo:rustc-link-search={}",
        outdir.join("CMSIS/CMSIS/DSP/Lib/GCC").display()
    );
    println!("cargo:rustc-link-lib=static={}", lib);

    let mut cmd = Command::new("bindgen");
    //bindgen args
    cmd.arg("wrapper.h");
    cmd.arg("--verbose");
    cmd.arg("--no-derive-default");
    cmd.arg("--ctypes-prefix=cty");
    cmd.arg("--use-core");
    cmd.arg("--output");
    cmd.arg(outdir.join("bindings.rs"));

    //clang args
    cmd.arg("--");
    cmd.arg(format!("-I{}", manifest_dir.join("include").display()));
    cmd.arg(format!(
        "-I{}",
        outdir.join("CMSIS/CMSIS/Core/Include").display()
    ));
    cmd.arg(format!(
        "-I{}",
        outdir.join("CMSIS/CMSIS/DSP/Include").display()
    ));
    cmd.arg("-nostdinc");

    eprintln!("{:?}", cmd);
    assert!(cmd.status().expect("failed to build cmsis").success());
}
