#![no_std]
#![no_main]

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use panic_halt as _;

#[entry]
fn main() -> ! {
    let mut i2c1 = si70xx::i2c_init();
    //let mut gpio = si70xx::gpio_init();

    hprintln!();
    hprintln!("stm32-rs application which displays relative temp/humidity");
    hprintln!();

    let fw_ver = si70xx::get_fw_version(&mut i2c1);
    si70xx::hprint_fw_version(fw_ver);
    let sensor_id = si70xx::get_sensor_id(&mut i2c1);
    si70xx::hprint_sensor_id(sensor_id);

    loop {
        let temperature = si70xx::get_rel_temperature(&mut i2c1);
        si70xx::hprint_temperature(temperature);
        let humidity = si70xx::get_rel_humidity(&mut i2c1);
        si70xx::hprint_humidity(humidity);
        hprintln!();
    }

    //panic!("End of the line chap... (main)");
}
