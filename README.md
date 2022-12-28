# `cortex-m-quickstart`

> A template for building applications for ARM Cortex-M microcontrollers

This project is developed and maintained by the [Cortex-M team][team].

## Dependencies

To build embedded programs using this template you'll need:

- Rust 1.31, 1.30-beta, nightly-2018-09-13 or a newer toolchain. e.g. `rustup
  default beta`

- The `cargo generate` subcommand. [Installation
  instructions](https://github.com/ashleygwilliams/cargo-generate#installation).

- `rust-std` components (pre-compiled `core` crate) for the ARM Cortex-M
  targets. Run:

``` console
$ rustup target add thumbv6m-none-eabi thumbv7m-none-eabi thumbv7em-none-eabi thumbv7em-none-eabihf
```

## Using this template

**NOTE**: This is the very short version that only covers building programs. For
the long version, which additionally covers flashing, running and debugging
programs, check [the embedded Rust book][book].

[book]: https://rust-embedded.github.io/book

0. Before we begin you need to identify some characteristics of the target
  device as these will be used to configure the project:

- The ARM core. e.g. Cortex-M3.

- Does the ARM core include an FPU? Cortex-M4**F** and Cortex-M7**F** cores do.

- How much Flash memory and RAM does the target device has? e.g. 256 KiB of
  Flash and 32 KiB of RAM.

- Where are Flash memory and RAM mapped in the address space? e.g. RAM is
  commonly located at address `0x2000_0000`.

You can find this information in the data sheet or the reference manual of your
device.

In this example we'll be using the STM32F3DISCOVERY. This board contains an
STM32F303VCT6 microcontroller. This microcontroller has:

- A Cortex-M4F core that includes a single precision FPU

- 256 KiB of Flash located at address 0x0800_0000.

- 40 KiB of RAM located at address 0x2000_0000. (There's another RAM region but
  for simplicity we'll ignore it).

1. Instantiate the template.

``` console
$ cargo generate --git https://github.com/rust-embedded/cortex-m-quickstart
 Project Name: app
 Creating project called `app`...
 Done! New project created /tmp/app

$ cd app
```

2. Set a default compilation target. There are four options as mentioned at the
   bottom of `.cargo/config`. For the STM32F303VCT6, which has a Cortex-M4F
   core, we'll pick the `thumbv7em-none-eabihf` target.

``` console
$ tail -n6 .cargo/config
```

``` toml
[build]
# Pick ONE of these compilation targets
# target = "thumbv6m-none-eabi"    # Cortex-M0 and Cortex-M0+
# target = "thumbv7m-none-eabi"    # Cortex-M3
# target = "thumbv7em-none-eabi"   # Cortex-M4 and Cortex-M7 (no FPU)
target = "thumbv7em-none-eabihf" # Cortex-M4F and Cortex-M7F (with FPU)
```

3. Enter the memory region information into the `memory.x` file.

``` console
$ cat memory.x
/* Linker script for the STM32F303VCT6 */
MEMORY
{
  /* NOTE 1 K = 1 KiBi = 1024 bytes */
  FLASH : ORIGIN = 0x08000000, LENGTH = 256K
  RAM : ORIGIN = 0x20000000, LENGTH = 40K
}
```

4. Build the template application or one of the examples.

``` console
$ cargo build
```

# License

This template is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## Code of Conduct

Contribution to this crate is organized under the terms of the [Rust Code of
Conduct][CoC], the maintainer of this crate, the [Cortex-M team][team], promises
to intervene to uphold that code of conduct.

[CoC]: https://www.rust-lang.org/policies/code-of-conduct
[team]: https://github.com/rust-embedded/wg#the-cortex-m-team


## Personal Section
I worked my way though this book:
https://docs.rust-embedded.org/book/start/hardware.html

I am reading through this one now:
https://docs.rust-embedded.org/discovery/f3discovery/index.html

I ordered an STM32 Discovery got the board and have run embedded Rust apps on it!
I have a basic write_read i2c function working to read the firmware version of the Si7021-A20
My plan is to add support to read temperature and humidity for the Si7021-A20 sensor.
Then add support for what ever other sensors I have.
Then add support for UART/USART?
    So I can have a debug print outside of GDB.. although GDB+OpenOCD works quite well!

https://www.st.com/content/st_com/en/products/evaluation-tools/product-evaluation-tools/mcu-mpu-eval-tools/stm32-mcu-mpu-eval-tools/stm32-nucleo-boards/nucleo-l476rg.html
https://www.st.com/en/microcontrollers-microprocessors/stm32l476rg.html#documentation
02/21/2022 - I got the NUCLEO-L276RG STM32 development board!  I have been able to load and run
code on the device with GDB/OpenOCD working following this tutorial:
https://docs.rust-embedded.org/book/start/hardware.html

### Running on STM32L476 Nucleo-64 board ###

Option One (a bit more manual)
1.) Open at least two shells using tmux.
2.) In one shell run:
    a.) openocd
3.) In one shell run the command:
    a.) gdb-multiarch -x openocd.gdb target/thumbv7em-none-eabi/debug/stm32-discovery-app
4.) If everything is setup properly the above command 3.a) will start gdb and run our app.

Option Two - Using cargo run
1.) Open at least two shells using tmux.
2.) In one shell run:
    a.) openocd
3.) In one shell run the command:
    a.) cargo run
4.) If everything is setup properly the above command 3.a) will start gdb and run our app.
    a.) Look here for more info: https://docs.rust-embedded.org/book/start/hardware.html

With these modifications since I don't have an f3x board I have an l4x board:
openocd -f interface/stlink.cfg -f target/stm32l4x.cfg

in place of:
openocd -f interface/stlink.cfg -f target/stm32f3x.cfg

I also modified the memory.x file as shown below:
   FLASH : ORIGIN = 0x08000000, LENGTH = 1024K
   RAM : ORIGIN = 0x20000000, LENGTH = 64K

I found this info in the following document: RM0351 Reference manual
on page 76 there is info about the memory map

## Example Ouptut ##
Device FW version: 2.0
Device ID: Si7020
RAW sensor ID: 0x3D891CCC15FFB5FF
Temperature: 19.613434 Celcius 67.304184 Fahrenheit
% Relative Humidity: 32.311005 % RH

