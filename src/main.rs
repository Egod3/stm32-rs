#![no_std]
#![no_main]

use cortex_m::delay::Delay;
use cortex_m::iprintln;
use cortex_m_rt::entry;
use panic_halt as _;
use stm32l4xx_hal::gpio::GpioExt;
use stm32l4xx_hal::gpio::*;
use stm32l4xx_hal::rcc::RccExt;
use stm32l4xx_hal::stm32::Peripherals;

#[entry]
fn main() -> ! {
    let mut i2c1 = si70xx::i2c_init();

    unsafe {
        let mut p = cortex_m::Peripherals::steal();
        let stim = &mut p.ITM.stim[0];
        if !stim.is_fifo_ready() {
            //iprintln!(stim, "stim.is_fifo_ready() returned false...\n");
        }
        while !stim.is_fifo_ready() {}

        iprintln!(
            stim,
            "stm32-rs application which displays relative temp/humidity, via ITM\n"
        );

        // initialize the LED
        let mut led = led_init();

        let fw_ver = si70xx::get_fw_version(&mut i2c1);
        si70xx::iprint_fw_version(stim, fw_ver);
        let sensor_id = si70xx::get_sensor_id(&mut i2c1);
        si70xx::iprint_sensor_id(stim, sensor_id);

        // TODO: find out how to convert from Hertz to u32 here... so we can use clocks.hclk() in place
        // of 16_000_000
        let cp = cortex_m::Peripherals::steal();
        let mut _timer = Delay::new(cp.SYST, 16_000_000);
        led.set_high();

        let mut led_state: bool = false;
        loop {
            let temperature = si70xx::get_rel_temperature(&mut i2c1);
            let humidity = si70xx::get_rel_humidity(&mut i2c1);
            si70xx::iprint_temperature(stim, temperature);
            si70xx::iprint_humidity(stim, humidity);
            si70xx::hprint_temperature(temperature);
            si70xx::hprint_humidity(humidity);

            _timer.delay_ms(1000_u32);
            if led_state {
                toggle_led(stim, &mut led, PinState::High);
                led_state = false;
            } else {
                toggle_led(stim, &mut led, PinState::Low);
                led_state = true;
            }
            iprintln!(
                stim,
                "stm32-rs application which displays relative temp/humidity, via ITM\n"
            );
        }
    }
}

fn led_init() -> stm32l4xx_hal::gpio::PA5<stm32l4xx_hal::gpio::Output<stm32l4xx_hal::gpio::PushPull>>
{
    unsafe {
        // TODO: Is there a safe way to take the dp pac::Peripherals::take().unwrap()  ?!?
        let dp = Peripherals::steal();
        let mut rcc = dp.RCC.constrain();
        let mut gpioa = dp.GPIOA.split(&mut rcc.ahb2);

        gpioa
            .pa5
            .into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper)
    }
}

fn toggle_led(
    _stim: &mut cortex_m::peripheral::itm::Stim,
    led: &mut PA5<Output<PushPull>>,
    state: PinState,
) {
    led.set_state(state);
    //let result = led.set_state(state);
    //if result.is_err() {
    //    iprintln!(_stim, "Error setting led state: {:?}", result.err());
    //}
}

// TODO: Get tests working..
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // fw_ver_sensor_id_test_Si7021_A20
    fn fw_ver_sensor_id_test_si7021_a20() {
        iprintln!(stim, "program start");
        let mut i2c1 = si70xx::i2c_init();

        let reset = si70xx::reset_sensor(&mut i2c1);

        iprintln!(stim, "");
        iprintln!(
            stim,
            "stm32-rs test which reads sensor ID and compares it to expected sensor ID."
        );
        iprintln!(stim, "");

        // Turn on the User LED PA5
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
        iprintln!(stim, "");
    }
}
