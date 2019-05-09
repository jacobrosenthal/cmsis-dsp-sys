# cmsis-dsp-sys

rust bindings for the prebuilt cmsis-dsp libraries

## why

Theres.. not a lot of real (non blinky) embedded work in the Rust space. We have some sensor drivers, but we dont really have any math capabilty. Its good that a [pure rust libm](https://github.com/rust-lang-nursery/libm) is being worked on. And even if that reaches some kind of parity soonish, were nowhere near to being able to replace all the plain old signal processing libraries like ffts that are built on top of fully featured libm implementations. 

And math and signal processing are just the table stakes, were not even talking about the exciting fast moving edge machine learning stuff coming from google's [tensorflow lite for microcontrollers](https://github.com/tensorflow/tensorflow/tree/master/tensorflow/lite/experimental/micro) and [microsoft edgeml](https://github.com/microsoft/EdgeML) teams. These are all based on arm's crazy competitive embedded architecture and their [cmsis](https://github.com/ARM-software/CMSIS_5) optmized math libraries.

So we need cmsis bindings right now. 

## how

Rust.. is.. complicated. Its very difficult or impossible to script any of this because cargo pollutes no_std builds. [1](https://github.com/rust-lang/cargo/issues/2589) [2](https://github.com/rust-lang/cargo/issues/2644) [3](https://github.com/rust-lang/cargo/issues/4866) [4](https://github.com/rust-lang/cargo/issues/6571) 

Git.. is also.. complicated. The [git archive command doesnt store appear to store git lfs files for releases](https://github.com/isaacs/github/issues/1392) so we cant just wget the CMSIS release archive. Instead you must use git clone with git lfs installed.

Cross compiling.. is also.. complicated. You must have a cross compile toolchain and provide it as a sysroot variable.
Install a toolchain as described in the rust embedded book [like arm-none-eabi-gcc for macosx here](https://rust-embedded.github.io/book/intro/install.html). If successful you should be able to find your sysroot:
```
$ arm-none-eabi-gcc -print-sysroot
/usr/local/Cellar/gcc-arm-none-eabi/20180627/bin/../arm-none-eabi
```

All together you need:
* install [git](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git)
* install [git lfs](https://help.github.com/en/articles/installing-git-large-file-storage)
* install a [cross compile toolchain](https://rust-embedded.github.io/book/intro/install.html)
* `git clone https://github.com/ARM-software/CMSIS_5.git --depth=1`
* `cargo install bindgen`
* `SYSROOT_PATH=/usr/local/Cellar/gcc-arm-none-eabi/20180627/bin/../arm-none-eabi CMSIS_PATH=CMSIS_5 cargo build --release --target thumbv7em-none-eabihf`

