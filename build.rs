use bindgen::builder;
use reqwest::blocking;
use std::env;
use std::fs::File;
use std::path::{Path, PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=wrapper.h");

    let manifest = env::var("CARGO_MANIFEST_DIR").unwrap();
    let outdir = PathBuf::from(env::var("OUT_DIR").unwrap());

    let filepath = outdir.join("CMSIS.zip");
    {
        let mut file = File::create(filepath.clone()).unwrap();
        blocking::get(
            // "https://github.com/ARM-software/CMSIS_5/releases/download/5.7.0/ARM.CMSIS.5.7.0.pack",
            "https://github.com/ARM-software/CMSIS_5/releases/download/5.9.0/ARM.CMSIS.5.9.0.pack"
        )
        .unwrap()
        .copy_to(&mut file)
        .unwrap();
    }

    let file = File::open(filepath).unwrap();

    let mut archive = zip::ZipArchive::new(file).unwrap();
    archive.extract(outdir.join("CMSIS")).unwrap();

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
        //Bare Cortex-M8F, 4M8F, FPU, hardfloat
        "thumbv8m.main-none-eabihf" => "arm_cortexM4lf_math",
        _ => panic!("no known arm math library for target {}", target),
    };

    // Link against prebuilt cmsis math
    println!(
        "cargo:rustc-link-search={}",
        outdir.join("CMSIS/CMSIS/DSP/Lib/GCC").display()
    );
    println!("cargo:rustc-link-lib=static={}", lib);

    let bb = builder()
        .header("wrapper.h")
        .derive_default(false)
        .ctypes_prefix("cty")
        .use_core()
        .generate_comments(true)
        .rustfmt_bindings(true)
        .clang_arg(format!("-I{}", manifest_dir.join("include").display()))
        .clang_arg(format!(
            "-I{}",
            outdir.join("CMSIS/CMSIS/Core/Include").display()
        ))
        .clang_arg(format!(
            "-I{}",
            outdir.join("CMSIS/CMSIS/DSP/Include").display()
        ))
        .clang_arg("-nostdinc");

    let cmd = bb.command_line_flags().join(" ");
    eprintln!("{:?}", cmd);

    let bindings = bb.generate().expect("Unable to generate bindings");
    bindings
        .write_to_file(outdir.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
