use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=wrapper.h");

    let sysroot_path = env::var("SYSROOT_PATH").expect(
        "You need to set the environment variable 'SYSROOT_PATH' to point to an installation of the a C library (e.g. /usr/include/newlib)");

    let sysroot_dir = Path::new(&sysroot_path);

    let cmsis_path = env::var("CMSIS_PATH").expect(
        "You need to set the environment variable 'CMSIS_PATH' to point to a git lfs clone of https://github.com/ARM-software/CMSIS_5");

    let cmsis_dir = Path::new(&cmsis_path);

    let target = env::var("TARGET").unwrap();
    // let (lib, arch) = ("arm_cortexM4lf_math", "__ARM_ARCH_7EM__");
    let (lib, arch, libc) = match target.as_ref() {
        //Bare Cortex-M0, M0+, M1
        "thumbv6m-none-eabi" => ("arm_cortexM0l_math", "__ARM_ARCH_6M__", "v6-m"),
        //Bare Cortex-M3
        "thumbv7m-none-eabi" => ("arm_cortexM3l_math", "__ARM_ARCH_7M__", "v7-m"),
        //Bare Cortex-M4, M7
        "thumbv7em-none-eabi" => ("arm_cortexM4l_math", "__ARM_ARCH_7EM__", "v7e-m"),
        //todo .. when
        // "thumbv7em-none-eabi" => "arm_cortexM7l_math",
        //Bare Cortex-M4F, M7F, FPU, hardfloat
        //todo this libc might be wrong?
        "thumbv7em-none-eabihf" => ("arm_cortexM4lf_math", "__ARM_ARCH_7EM__", "v7e-m"),
        //todo .. when
        // "thumbv7em-none-eabihf" => "arm_cortexM7lfdp_math",
        // "thumbv7em-none-eabihf" => "arm_cortexM7lfsp_math",
        //// https://github.com/rust-embedded/wg/issues/88 ??
        "thumbv8m.base-none-eabi" => ("arm_ARMv8MBLl_math", "__ARM_ARCH_8M_BASE__", "v8-m.base"),
        "thumbv8m.main-none-eabi" => ("arm_ARMv8MMLl_math", "__ARM_ARCH_8M_MAIN__", "v8-m.main"),
        //todo .. when
        // "thumbv8m.main-none-eabi" => "arm_ARMv8MMLld_math",
        "thumbv8m.main-none-eabihf" => {
            ("arm_ARMv8MMLlfsp_math", "__ARM_ARCH_8M_MAIN__", "v8-m.main")
        }
        _ => panic!("no known arm math library for target {}", target),
    };

    // Link against prebuilt cmsis dsp
    println!(
        "cargo:rustc-link-search={}",
        cmsis_dir.join("CMSIS/DSP/Lib/GCC").display()
    );
    println!("cargo:rustc-link-lib=static={}", lib);

    // Link against libm
    // println!(
    //     "cargo:rustc-link-search={}",
    //     sysroot_dir.join("lib/thumb").join(libc).display()
    // );
    // println!("cargo:rustc-link-lib=static=m");

    let outdir = PathBuf::from(env::var("OUT_DIR").unwrap());

    let mut cmd = Command::new("bindgen");
    cmd.arg("wrapper.h");
    cmd.arg("--verbose");

    cmd.arg("--no-derive-default");
    cmd.arg("--ctypes-prefix=cty");

    cmd.arg("--use-core");

    cmd.arg("--output");
    cmd.arg(outdir.join("bindings.rs"));

    cmd.arg("--blacklist-function");
    cmd.arg("sqrtf");

    cmd.arg("--");

    cmd.arg("-target");
    cmd.arg(target);

    cmd.arg(format!("-I{}", sysroot_dir.join("include").display()));

    //cmsis stuff
    cmd.arg(format!("-D{}=1", arch));

    cmd.arg(format!(
        "-I{}",
        cmsis_dir.join("CMSIS/Core/Include").display()
    ));

    cmd.arg(format!(
        "-I{}",
        cmsis_dir.join("CMSIS/DSP/Include").display()
    ));

    // cmd.arg("-DNDEBUG");
    // cmd.arg("-g");
    // cmd.arg("-O3");
    // cmd.arg("-fno-rtti");
    // cmd.arg("-fmessage-length=0");
    // cmd.arg("-fno-exceptions");
    // cmd.arg("-fno-unwind-tables");
    cmd.arg("-fno-builtin");
    cmd.arg("-ffunction-sections");
    cmd.arg("-fdata-sections");
    // cmd.arg("-funsigned-char");
    // cmd.arg("-MMD");
    // cmd.arg("-mcpu=cortex-m3");
    // cmd.arg("-mthumb");
    // cmd.arg("-Wvla");
    // cmd.arg("-Wall");
    // cmd.arg("-Wextra");
    // cmd.arg("-Wno-unused-parameter");
    // cmd.arg("-Wno-missing-field-initializers");
    // cmd.arg("-Wno-write-strings");
    // cmd.arg("-Wno-sign-compare");
    // cmd.arg("-fno-delete-null-pointer-checks");
    // cmd.arg("-fomit-frame-pointer");
    // cmd.arg("-fpermissive");
    cmd.arg("-nostdlib");
    // cmd.arg("-g");
    // cmd.arg("-Os");
    //todo dont hardcode
    cmd.arg("-DARM_MATH_CM3");
    cmd.arg("-DARM_CMSIS_NN_M3");

    assert!(cmd.status().expect("failed to build cmsis").success());
}
