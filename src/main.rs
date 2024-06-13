#![no_std]
#![no_main]

use embassy_executor::Spawner;
use esp_backtrace as _;
use esp_hal::{peripherals::Peripherals, prelude::*, system::SystemControl};
use esp_println::println;

#[main]
async fn main(spawner: Spawner) {
    let peripherals = Peripherals::take();
    let _system = SystemControl::new(peripherals.SYSTEM);

    println!("Hello world!");

    loop {}
}
