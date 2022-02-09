#![no_std]
#![no_main]

// use esp8266_hal::prelude::*;
// use esp8266_hal::target::Peripherals;
use esp8266_nonos_sys;
use panic_halt as _;

// #[entry]
// pub fn main() {
// let dp = Peripherals::take().unwrap();
// let pins = dp.GPIO.split();
// let mut led = pins.gpio2.into_push_pull_output();
// let (mut timer1, _) = dp.TIMER.timers();

// loop {
//     timer1.delay_ms(250);
//     led.toggle().unwrap();
// }
// }

#[no_mangle]
// #[link_section = ".irom0"]
pub extern "C" fn user_init() {
    unsafe {
        esp8266_nonos_sys::system_get_sdk_version();
    }
}

#[no_mangle]
// #[link_section = ".irom0"]
pub extern "C" fn user_pre_init() {}
