#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::Timer;
use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl, peripherals::Peripherals, prelude::*, system::SystemControl,
    timer::systimer::SystemTimer,
};
use esp_println::println;

#[main]
async fn main(_spawner: Spawner) {
    let peripherals = Peripherals::take();
    // initialize the peripherals
    let system = SystemControl::new(peripherals.SYSTEM);
    let clock = ClockControl::boot_defaults(system.clock_control).freeze();
    let timer = SystemTimer::new_async(peripherals.SYSTIMER); // async because embassy needs async

    esp_hal_embassy::init(&clock, timer);

    let mut x = 0;
    // TODO: somehow enable the clock?
    loop {
        println!("Hello, world! x = {}", x);
        Timer::after_millis(500).await;
        x += 1;
    }
}
