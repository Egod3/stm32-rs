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
    i2c_dev: &mut stm32l4xx_hal::i2c::I2c<
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
pub fn get_sensor_id(
    i2c_dev: &mut stm32l4xx_hal::i2c::I2c<
        stm32l4xx_hal::pac::I2C1,
        (
            PB8<Alternate<AF4, Output<OpenDrain>>>,
            PB9<Alternate<AF4, Output<OpenDrain>>>,
        ),
    >,
) -> u64 {
    let mut buf_i = [0xFAu8, 0x0Fu8];
    let mut buf_o = [0u8; 4];
    i2c_dev
        .write_read(I2C_SECOND_ADDR, &buf_i, &mut buf_o)
        .unwrap();
    let mut sensor_id: u64 = ((buf_o[0] as u64) << 56)
        | ((buf_o[1] as u64) << 48)
        | ((buf_o[2] as u64) << 40)
        | ((buf_o[3] as u64) << 32);
    buf_i[0] = 0xFC;
    buf_i[1] = 0xC9;
    i2c_dev
        .write_read(I2C_SECOND_ADDR, &buf_i, &mut buf_o)
        .unwrap();
    sensor_id = (sensor_id + ((buf_o[0] as u64) << 24))
        | ((buf_o[1] as u64) << 16)
        | ((buf_o[2] as u64) << 8)
        | (buf_o[3] as u64);
    sensor_id
}

#[allow(clippy::type_complexity)]
pub fn get_fw_version(
    i2c_dev: &mut stm32l4xx_hal::i2c::I2c<
        stm32l4xx_hal::pac::I2C1,
        (
            PB8<Alternate<AF4, Output<OpenDrain>>>,
            PB9<Alternate<AF4, Output<OpenDrain>>>,
        ),
    >,
) -> u8 {
    let buf_i = [0x84u8, 0xB8u8];
    let mut buf_o = [0u8, 2];
    i2c_dev
        .write_read(I2C_SECOND_ADDR, &buf_i, &mut buf_o)
        .unwrap();
    let fw_ver: u8 = buf_o[0];
    fw_ver
}

#[allow(clippy::type_complexity)]
pub fn get_temperature(
    i2c_dev: &mut stm32l4xx_hal::i2c::I2c<
        stm32l4xx_hal::pac::I2C1,
        (
            PB8<Alternate<AF4, Output<OpenDrain>>>,
            PB9<Alternate<AF4, Output<OpenDrain>>>,
        ),
    >,
) -> u16 {
    let buf_i = [0xE3u8, 0];
    let mut buf_o = [0u8; 2];
    i2c_dev
        .write_read(I2C_SECOND_ADDR, &buf_i, &mut buf_o)
        .unwrap();
    let temperature: u16 = (0xFF_00 & (buf_o[0] as u16) << 8) | 0x00_FF & (buf_o[1] as u16);
    temperature
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

fn print_sensor_id(sensor_id: u64) {
    let snb_3 = (0x0000_0000_FF00_0000 & sensor_id) >> 24;
    hprintln!().unwrap();
    if snb_3 == 0x15 {
        hprintln!("Temperature Device ID: Si7021").unwrap();
    } else if snb_3 == 0x14 {
        hprintln!("Temperature Device ID: Si7020").unwrap();
    } else if snb_3 == 0x0D {
        hprintln!("Temperature Device ID: Si7013").unwrap();
    } else {
        hprintln!("Temperature Device ID: unknown").unwrap();
    }
    hprintln!("Device sensor ID: {:#08X}", sensor_id).unwrap();
    hprintln!().unwrap();
}

/*
 * Formula for temperature conversion
 * ( (175.72 * Temp_code) / 65536) - 46.85 = Temperature in Celcius
 *
 */
fn print_temperature(temperature: u16) {
    hprintln!().unwrap();
    let temper_c = ((175.72 * temperature as f32) / 65536.0) - 46.85;
    let temper_f = (temper_c * 1.8) + 32.0;
    hprintln!("Temperature: {}C {}F", temper_c, temper_f).unwrap();
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

    let mut i2c1 = I2c::i2c1(dp.I2C1, (scl, sda), 400.khz(), clocks, &mut rcc.apb1r1);

    let fw_ver = get_fw_version(&mut i2c1);
    print_fw_version(fw_ver);
    let sensor_id = get_sensor_id(&mut i2c1);
    print_sensor_id(sensor_id);
    let mut _temperature = get_temperature(&mut i2c1);
    _temperature = get_temperature(&mut i2c1);
    print_temperature(_temperature);

    panic!("End of the line chap... (main)");
}
