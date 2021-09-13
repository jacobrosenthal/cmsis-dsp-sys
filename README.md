# cmsis-dsp-sys

rust bindings for the prebuilt cmsis-dsp math

Note downloads ~100mb [CMSIS_5](https://github.com/ARM-software/CMSIS_5/releases/download/5.7.0/ARM.CMSIS.5.7.0.pack) during build phase in order select and link the correct math.lib

## prereqs

* Rust edition 2021 or Rust >=1.51 with [resolver set to 2](https://doc.rust-lang.org/cargo/reference/resolver.html#feature-resolver-version-2)
* [bindgen requirements (llvm/clang)](https://github.com/rust-lang/rust-bindgen/blob/master/book/src/requirements.md)

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
use cmsis_dsp_sys::{arm_cfft_sR_f32_len16, arm_cfft_f32, arm_cfft_instance_f32, arm_cmplx_mag_f32};
use micromath::F32Ext;

//C needs access to a sqrt fn, lets use micromath
#[no_mangle]
pub extern "C" fn sqrtf(x: f32) -> f32 {
    x.sqrt()
}
...
    //your data as stored as real and imaginary pairs here
    let mut dtfsecoef = [0f32; 32];

    let mut mag = [0f32; 16];

    //Coefficient calculation with CFFT function
    unsafe {

        //CFFT calculation
        arm_cfft_f32(&arm_cfft_sR_f32_len16, dtfsecoef.as_mut_ptr(), 0, 1);

        // Magnitude calculation
        arm_cmplx_mag_f32(s.as_ptr(), mag.as_mut_ptr(), N::to_usize() as uint32_t);

    }
...
```
