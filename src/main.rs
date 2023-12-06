#![no_std]
#![no_main]

use esp32c6_hal::{clock::ClockControl, peripherals::Peripherals, prelude::*, timer::TimerGroup};
use esp_backtrace as _;
use esp_println as _;
use nb::block;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    let timer_group0 = TimerGroup::new(peripherals.TIMG0, &clocks);
    let mut timer0 = timer_group0.timer0;
    timer0.start(1u64.secs());

    loop {
        defmt::println!("Hello world!");
        block!(timer0.wait()).unwrap();
    }
}
