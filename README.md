# cmsis-dsp-sys

rust bindings for the prebuilt cmsis-dsp math

## prereqs

Currently only requires `unzip` and `curl` cli commands. Happy to take prs to use pure rust instead. Downloads the cmsis math pack file during build.

## usage

```rust
use cmsis_dsp_sys::{arm_sin_f32};

let sin = unsafe { arm_sin_f32(core::f32::consts::PI) };
assert_eq!(sin, 0f32);
```

## the dirty details

Its complicated. Im currently not building any c files that accompany the arm dsp libraries. Bringing the necessary libc, libm and abstracting against every possible architecture and build flag seems hopeless atm. As a result you're going to have to do some work on your end.

When you start to do anything complicated you're probably going to see something like:

```bash
  = note: rust-lld: error: undefined symbol: sqrtf
          >>> referenced by arm_math.h:6841 (../../Include/arm_math.h:6841)
          >>>               arm_cmplx_mag_f32.o:(arm_cmplx_mag_f32) in archive /home/jacob/Downloads/dsp-discoveryf4-rust/lab4/libarm_cortexM4lf_math.a
```

Which means you need to bring some libm type function with you. Possibilitie pure rust options I've seen include [libm](https://github.com/rust-lang/libm) or [micromath](https://github.com/NeoBirth/micromath) with differing tradeoffs. Stub one in with:

```rust
use micromath::F32Ext;

//C needs access to a sqrt fn, lets use micromath
#[no_mangle]
pub extern "C" fn sqrtf(x: f32) -> f32 {
    x.sqrt()
}
```

Further you're going to want to use something from arm_const_structs.c like arm_cfft_sR_f32_len16 which uses tables in arm_common_tables.c. I would look those up inside the cmsis pack and translate it into some arm_common_tables.rs file in your project

```rust
#![allow(clippy::excessive_precision)]
#![allow(clippy::approx_constant)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused)]

pub const ARMBITREVINDEXTABLE_16_TABLE_LENGTH: u16 = 20;

pub static armBitRevIndexTable16: &[u16] = &[
    /* 8x2, size 20 */
    8, 64, 24, 72, 16, 64, 40, 80, 32, 64, 56, 88, 48, 72, 88, 104, 72, 96, 104, 112,
];

pub static twiddleCoef_16: &[f32] = &[
    1.000000000,  0.000000000,
    0.923879533,  0.382683432,
    0.707106781,  0.707106781,
    0.382683432,  0.923879533,
    0.000000000,  1.000000000,
   -0.382683432,  0.923879533,
   -0.707106781,  0.707106781,
   -0.923879533,  0.382683432,
   -1.000000000,  0.000000000,
   -0.923879533, -0.382683432,
   -0.707106781, -0.707106781,
   -0.382683432, -0.923879533,
   -0.000000000, -1.000000000,
    0.382683432, -0.923879533,
    0.707106781, -0.707106781,
    0.923879533, -0.382683432,
];
```

```rust
mod arm_common_tables;
use arm_common_tables::{
    armBitRevIndexTable16, twiddleCoef_16, ARMBITREVINDEXTABLE_16_TABLE_LENGTH,
};
use cmsis_dsp_sys::{arm_cfft_f32, arm_cfft_instance_f32, arm_cmplx_mag_f32};

//your data as stored as real and imaginary pairs here
let mut dtfsecoef = [0f32; 32];

let cfft = arm_cfft_instance_f32 {
    fftLen: 16,
    pTwiddle: twiddleCoef_16.as_ptr(),
    pBitRevTable: armBitRevIndexTable16.as_ptr(),
    bitRevLength: ARMBITREVINDEXTABLE_16_TABLE_LENGTH,
};

let mut mag = [0f32; 16];

//Coefficient calculation with CFFT function
unsafe {

    //CFFT calculation
    arm_cfft_f32(&cfft, dtfsecoef.as_mut_ptr(), 0, 1);

    // Magnitude calculation
    arm_cmplx_mag_f32(s.as_ptr(), mag.as_mut_ptr(), N::to_usize() as uint32_t);

}
```
