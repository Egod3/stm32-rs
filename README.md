# License

This template is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

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
02/21/2022 - I got the NUCLEO64-L276RG STM32 dev board!  I have been able to load and run
code on the device with GDB/OpenOCD working following this tutorial:
https://docs.rust-embedded.org/book/start/hardware.html

### Running on STM32L476RG Nucleo-64 board ###
### NUCLEO-L476RG board for STM32L476RGT6 MCU with 80 MHz Cortex-M4F core, ###
### 1024 KB flash (HW ECC), 96 KB SRAM, 32 KB SRAM (HW parity), ###
### external quad-SPI memory interface, external static memory interface. ###

Option One - Using cargo run
1.) Open at least two shells using tmux.
2.) In one shell run:
    a.) openocd
3.) In one shell run the command:
    a.) cargo run
4.) If everything is setup properly the above command 3.a) will start gdb and run our app.
    a.) Look here for more info: https://docs.rust-embedded.org/book/start/hardware.html

Option Two (a bit more manual)
1.) Open at least two shells using tmux.
2.) In one shell run:
    a.) openocd
3.) In one shell run the command:
    a.) gdb-multiarch -x openocd.gdb target/thumbv7em-none-eabi/debug/stm32-discovery-app
4.) If everything is setup properly the above command 3.a) will start gdb and run our app.


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

