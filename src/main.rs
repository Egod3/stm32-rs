#![no_std]
#![no_main]

//use core::fmt::Write;
use cortex_m_rt::entry; //, exception, ExceptionFrame};
use cortex_m_semihosting::hprintln;
use panic_halt as _;
use stm32l4xx_hal::gpio::{Alternate, OpenDrain, Output, AF4, PB8, PB9};
use stm32l4xx_hal::{i2c::I2c, pac, prelude::*};

const I2C_SECOND_ADDR: u8 = 0x40;

#[allow(clippy::type_complexity)]
pub fn i2c_write_read(
    mut i2c_dev: stm32l4xx_hal::i2c::I2c<
        stm32l4xx_hal::pac::I2C1,
        (
            PB8<Alternate<AF4, Output<OpenDrain>>>,
            PB9<Alternate<AF4, Output<OpenDrain>>>,
        ),
    >,
    buf_o: &mut [u8; 6],
    buf_i: [u8; 6],
) {
    // write data from buf_o to device then read data from device into buf_i
    i2c_dev.write_read(I2C_SECOND_ADDR, &buf_i, buf_o).unwrap();
}

#[allow(clippy::type_complexity)]
pub fn get_fw_version(
    mut i2c_dev: stm32l4xx_hal::i2c::I2c<
        stm32l4xx_hal::pac::I2C1,
        (
            PB8<Alternate<AF4, Output<OpenDrain>>>,
            PB9<Alternate<AF4, Output<OpenDrain>>>,
        ),
    >,
) -> u8 {
    // Read FW version command
    let buffer = [0x84u8, 0xB8u8];
    let mut read_buf = [0u8, 2];
    i2c_dev
        .write_read(I2C_SECOND_ADDR, &buffer, &mut read_buf)
        .unwrap();
    let fw_ver: u8 = read_buf[0];
    fw_ver
}

fn print_fw_version(fw_ver: u8) {
    hprintln!().unwrap();
    if fw_ver == 0x20 {
        hprintln!("Device FW version: 2.0").unwrap();
    } else if fw_ver == 0xFF {
        hprintln!("Device FW version: 1.0").unwrap();
    } else {
        hprintln!("Device FW version: unknown").unwrap();
    }
    hprintln!().unwrap();
}

#[entry]
fn main() -> ! {
    //let _periph = cortex_m::Peripherals::take().unwrap();
    //let dp = stm32::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let mut power = dp.PWR.constrain(&mut rcc.apb1r1);
    let clocks = rcc.cfgr.freeze(&mut flash.acr, &mut power);

    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb2);

    let scl = gpiob
        .pb8
        .into_open_drain_output(&mut gpiob.moder, &mut gpiob.otyper);
    let scl = scl.into_af4(&mut gpiob.moder, &mut gpiob.afrh);

    let sda = gpiob
        .pb9
        .into_open_drain_output(&mut gpiob.moder, &mut gpiob.otyper);
    let sda = sda.into_af4(&mut gpiob.moder, &mut gpiob.afrh);

    let i2c1 = I2c::i2c1(dp.I2C1, (scl, sda), 400.khz(), clocks, &mut rcc.apb1r1);

    let fw_ver = get_fw_version(i2c1);
    print_fw_version(fw_ver);

    panic!("End of the line chap... (main)");
}

/*
 * Formula for temperature conversion
 * ( (175.72 * Temp_code) / 65536) - 46.85 = Temperature in Celcius
 *
 */
