#![no_std]
#![no_main]

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use panic_halt as _;
use stm32l4xx_hal::gpio::{Alternate, OpenDrain, Output, AF4, PB8, PB9};
use stm32l4xx_hal::{i2c::I2c, pac, prelude::*};

const I2C_SECOND_ADDR: u8 = 0x40;

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
    let mut sensor_id: u64 = 0;
    let mut result = i2c_dev.write_read(I2C_SECOND_ADDR, &buf_i, &mut buf_o);
    if let Ok(_val) = result {
        sensor_id = ((buf_o[0] as u64) << 56)
            | ((buf_o[1] as u64) << 48)
            | ((buf_o[2] as u64) << 40)
            | ((buf_o[3] as u64) << 32);
    } else {
        hprintln!("error getting sensor ID: {:?}", result).unwrap();
    }
    buf_i[0] = 0xFC;
    buf_i[1] = 0xC9;
    result = i2c_dev.write_read(I2C_SECOND_ADDR, &buf_i, &mut buf_o);
    if let Ok(_val) = result {
        sensor_id = (sensor_id + ((buf_o[0] as u64) << 24))
            | ((buf_o[1] as u64) << 16)
            | ((buf_o[2] as u64) << 8)
            | (buf_o[3] as u64);
    } else {
        hprintln!("error getting sensor ID: {:?}", result).unwrap();
    }
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
    let mut fw_ver: u8 = 0;
    let result = i2c_dev.write_read(I2C_SECOND_ADDR, &buf_i, &mut buf_o);
    if let Ok(_val) = result {
        fw_ver = buf_o[0];
    } else {
        hprintln!("error getting firmware version: {:?}", result).unwrap();
    }
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
    let result = i2c_dev.write_read(I2C_SECOND_ADDR, &buf_i, &mut buf_o);
    let mut temperature: u16 = 0;
    if let Ok(_val) = result {
        temperature = (0xFF_00 & (buf_o[0] as u16) << 8) | 0x00_FF & (buf_o[1] as u16);
    } else {
        hprintln!("error getting temperature: {:?}", result).unwrap();
    }
    temperature
}

#[allow(clippy::type_complexity)]
pub fn get_humidity(
    i2c_dev: &mut stm32l4xx_hal::i2c::I2c<
        stm32l4xx_hal::pac::I2C1,
        (
            PB8<Alternate<AF4, Output<OpenDrain>>>,
            PB9<Alternate<AF4, Output<OpenDrain>>>,
        ),
    >,
) -> u16 {
    let buf_i = [0xE5u8, 0];
    let mut buf_o = [0u8; 2];
    let result = i2c_dev.write_read(I2C_SECOND_ADDR, &buf_i, &mut buf_o);
    let mut humidity: u16 = 0;
    if let Ok(_val) = result {
        humidity = (0xFF_00 & (buf_o[0] as u16) << 8) | 0x00_FF & (buf_o[1] as u16);
    } else {
        hprintln!("error getting humidity: {:?}", result).unwrap();
    }
    humidity
}

fn print_fw_version(fw_ver: u8) {
    if fw_ver == 0x20 {
        hprintln!("Device FW version: 2.0").unwrap();
    } else if fw_ver == 0xFF {
        hprintln!("Device FW version: 1.0").unwrap();
    } else {
        hprintln!("Device FW version: unknown").unwrap();
    }
}

fn print_sensor_id(sensor_id: u64) {
    let snb_3 = (0x0000_0000_FF00_0000 & sensor_id) >> 24;
    let mut _dev_id: &str = Default::default();
    if snb_3 == 0x15 {
        _dev_id = "Si7020";
    } else if snb_3 == 0x14 {
        _dev_id = "Si7021";
    } else if snb_3 == 0x0D {
        _dev_id = "Si7013";
    } else {
        _dev_id = "unknown";
    }
    hprintln!("Device ID: {}", _dev_id).unwrap();
    hprintln!("RAW sensor ID: {:#08X}", sensor_id).unwrap();
}

/*
 * Formula for temperature conversion
 * ( (175.72 * Temp_code) / 65536) - 46.85 = Temperature in Celcius
 */
fn print_temperature(temperature: u16) {
    let temper_c = ((175.72 * temperature as f32) / 65536.0) - 46.85;
    let temper_f = (temper_c * 1.8) + 32.0;
    hprintln!("Temperature: {} Celcius {} Fahrenheit", temper_c, temper_f).unwrap();
}

/*
 * Formula for Relative Humidity % conversion
 * ( (125 * RH_code) / 65536) - 6 = % Relative Humidity
 */
fn print_humidity(humidity: u16) {
    let percent_rh: f32 = ((125.0 * humidity as f32) / 65536.0) - 6.0;
    hprintln!("% Relative Humidity: {} % RH ", percent_rh).unwrap();
}

#[entry]
fn main() -> ! {
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

    hprintln!().unwrap();

    let fw_ver = get_fw_version(&mut i2c1);
    print_fw_version(fw_ver);
    let sensor_id = get_sensor_id(&mut i2c1);
    print_sensor_id(sensor_id);

    let temperature = get_temperature(&mut i2c1);
    print_temperature(temperature);
    let humidity = get_humidity(&mut i2c1);
    print_humidity(humidity);

    panic!("End of the line chap... (main)");
}
