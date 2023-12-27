#![no_std]
#![no_main]

use cortex_m::delay::Delay;
use cortex_m::prelude::_embedded_hal_blocking_serial_Write;
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use panic_halt as _;
use stm32l4xx_hal::gpio::GpioExt;
use stm32l4xx_hal::gpio::*;
use stm32l4xx_hal::prelude::OutputPin;
use stm32l4xx_hal::prelude::_stm32l4_hal_FlashExt;
use stm32l4xx_hal::prelude::*;
use stm32l4xx_hal::pwr::PwrExt;
use stm32l4xx_hal::rcc::RccExt;
use stm32l4xx_hal::serial::Config;
use stm32l4xx_hal::serial::Serial;
use stm32l4xx_hal::stm32::Peripherals;

// TODO: move to a uart/usart lib when implemented.
const _FREQ: u32 = 80 * 1024 * 1024;
const _BAUD: u32 = 115_200;

#[entry]
fn main() -> ! {
    //let mut i2c1 = si70xx::i2c_init();

    //// initialize the uart
    ////let mut uart2 = uart_init();

    hprintln!();
    hprintln!("stm32-rs application which displays relative temp/humidity");
    hprintln!();

    //let fw_ver = si70xx::get_fw_version(&mut i2c1);
    //si70xx::hprint_fw_version(fw_ver);
    //let sensor_id = si70xx::get_sensor_id(&mut i2c1);
    //si70xx::hprint_sensor_id(sensor_id);
    //let temperature = si70xx::get_rel_temperature(&mut i2c1);
    //si70xx::hprint_temperature(temperature);
    //let humidity = si70xx::get_rel_humidity(&mut i2c1);
    //si70xx::hprint_humidity(humidity);
    //hprintln!();

    //// initialize the uart
    //let mut uart2 = uart_init();

    //hprintln!("Hello World, from the stm32-rs application to all you beautiful people out there");
    //write_str(
    //    "Hello World, from the stm32-rs application to all you beautiful people out there",
    //    &mut uart2,
    //);

    //loop {
    //    //let temperature = si70xx::get_rel_temperature(&mut i2c1);
    //    //si70xx::hprint_temperature(temperature);
    //    //let humidity = si70xx::get_rel_humidity(&mut i2c1);
    //    //si70xx::hprint_humidity(humidity);
    //    //hprintln!();

    //    write_str("Temp: ", &mut uart2);
    //    //write_str(temperature.try_into().unwrap(), &mut uart2);
    //    write_str("\n\r", &mut uart2);
    //    write_str("  Humidty: ", &mut uart2);
    //    //write_str(humidity, &mut uart2);
    //}

    //serial_example();

    hprintln!("take the cp and dp pointers.");
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = Peripherals::take().unwrap();

    hprintln!("get the flah rcc and power pointers");
    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let mut pwr = dp.PWR.constrain(&mut rcc.apb1r1);

    hprintln!("Split GPIOA int ports and pins");
    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb2);
    //let mut gpioa = dp.GPIOA.split(&mut rcc.ahb2);
    // let mut gpiob = dp.GPIOB.split(&mut rcc.ahb2);

    hprintln!("Try and configure the clock to 80MHz...");
    // clock configuration using the default settings (all clocks run at 16 MHz)
    //let clocks = rcc.cfgr.hclk(_FREQ).freeze(&mut flash.acr, &mut pwr);
    let clocks = rcc.cfgr.freeze(&mut flash.acr, &mut pwr);

    hprintln!("System clock running at {:?} Hz", clocks.sysclk());
    hprintln!("AHB clock running at {:?} Hz", clocks.hclk());
    hprintln!("APB 1 running at {:?} Hz", clocks.pclk1());
    hprintln!("APB 2 running at {:?} Hz", clocks.pclk2());
    //hprintln!("HSI48 running at {:?} Hz", clocks.hsi48());

    // The Serial API is highly generic
    // TRY the commented out, different pin configurations
    // let tx = gpioa.pa9.into_af7_pushpull(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrh);
    let tx = gpioa.pa2.into_af7(&mut gpioa.moder, &mut gpioa.afrl);
    //.into_alternate(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrl);
    // let tx = gpiob.pb6.into_alternate(&mut gpiob.moder, &mut gpiob.otyper, &mut gpiob.afrl);

    // let rx = gpioa.pa10.into_alternate(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrh);
    let rx = gpioa.pa3.into_af7(&mut gpioa.moder, &mut gpioa.afrl);
    //.into_alternate(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrl);
    // let rx = gpiob.pb7.into_alternate(&mut gpiob.moder, &mut gpiob.otyper, &mut gpiob.afrl);

    let uart_config: Config = Default::default();
    //uart_config.baudrate(stm32l4xx_hal::time::Bps(_BAUD));

    let serial = Serial::usart2(
        dp.USART2,
        (tx, rx),
        uart_config.baudrate(stm32l4xx_hal::time::Bps(921_600)),
        clocks,
        &mut rcc.apb1r1,
    );
    let (mut tx, _rx) = serial.split();

    let buffer = b"Hello World!\n\r";
    let _result = tx.bwrite_all(buffer);

    let mut led = gpioa
        .pa5
        .into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);

    hprintln!("Setup a system timer to delay once per sec");
    // TODO: find out how to convert from Hertz to u32 here... so we can use clocks.hclk() in place
    // of 16_000_000
    let mut timer = Delay::new(cp.SYST, 16_000_000);
    let _result = led.set_high();
    if _result.is_ok() {
        hprintln!("Setting the LED LD2 high success!");
    } else {
        hprintln!("Setting the LED LD2 high ERROR!");
    }
    loop {
        timer.delay_ms(1000_u32);
        toggle_led(&mut led, PinState::High);
        //let _ = led.set_high();
        timer.delay_ms(1000_u32);
        toggle_led(&mut led, PinState::Low);
        //let _ = led.set_low();
    }
}

fn serial_init() {}

fn toggle_led(led: &mut PA5<Output<PushPull>>, state: PinState) {
    let result = led.set_state(state);
    if result.is_err() {
        hprintln!("Error setting led state: {:?}", result.err());
    }
}

// TODO: Get tests working..
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // fw_ver_sensor_id_test_Si7021_A20
    fn fw_ver_sensor_id_test_si7021_a20() {
        hprintln!("program start");
        let mut i2c1 = si70xx::i2c_init();

        let reset = si70xx::reset_sensor(&mut i2c1);

        println!();
        println!("stm32-rs test which reads sensor ID and compares it to expected sensor ID.");
        println!();

        // Turn on the User LED PA5
        //gpio_init();
        let fw_ver = si70xx::get_fw_version(&mut i2c1);
        si70xx::hprint_fw_version(fw_ver);
        assert_eq!(fw_ver, 0x20);
        let sensor_id = si70xx::get_sensor_id(&mut i2c1);
        si70xx::hprint_sensor_id(sensor_id);
        // 0x3D89_1CCC_15FF_B5FF
        assert_eq!(sensor_id, 0x3D89_1CCC_15FF_B5FF);

        let temperature = si70xx::get_rel_temperature(&mut i2c1);
        si70xx::hprint_temperature(temperature);
        let humidity = si70xx::get_rel_humidity(&mut i2c1);
        si70xx::hprint_humidity(humidity);
        hprintln!();
    }
}
