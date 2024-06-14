#![no_std]
#![no_main]

mod xchg;

use defmt::{debug, info, trace, warn};
use esp_backtrace as _;
use esp_println as _;

use embassy_executor::Spawner;
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, channel::Channel};
use embassy_time::Timer;
use esp_hal::{
    aes::{Aes, Mode},
    clock::ClockControl,
    peripherals::Peripherals,
    prelude::*,
    rng::Rng,
    sha::{Sha, ShaMode},
    system::SystemControl,
    timer::timg::TimerGroup,
};
use heapless::{String, Vec};
use xchg::XchgPipeTx;

static SHARED: Channel<CriticalSectionRawMutex, (&'static str, u32), 8> = Channel::new();

#[embassy_executor::task]
async fn producer(tx: XchgPipeTx<'static, (&'static str, u32), 8>) {
    let mut x = 0;
    loop {
        tx.send(("producer1", x)).await;
        trace!("producer1 produced value = {}", x);
        Timer::after_millis(200 + x as u64).await;
        x += 1;
    }
}

#[embassy_executor::task]
async fn producer2(tx: XchgPipeTx<'static, (&'static str, u32), 8>) {
    let mut x = 0;
    loop {
        tx.send(("producer2", x)).await;
        trace!("producer2 produced value = {}", x);
        Timer::after_millis(300 + x as u64).await;
        x += 1;
    }
}

#[embassy_executor::task]
async fn producer3(tx: XchgPipeTx<'static, (&'static str, u32), 8>) {
    for i in (0..10).rev() {
        tx.send(("producer3", i)).await;
        trace!("producer3 produced value = {}", i);
        Timer::after_millis(200).await;
    }
    tx.send(("producer3 exiting", 0)).await;
}

#[main]
async fn main(spawner: Spawner) {
    let peripherals = Peripherals::take();
    // initialize the peripherals
    let system = SystemControl::new(peripherals.SYSTEM);
    let clock = ClockControl::boot_defaults(system.clock_control).freeze();
    let timer = TimerGroup::new_async(peripherals.TIMG0, &clock);

    esp_hal_embassy::init(&clock, timer);
    defmt::println!("Starting: use DEFMT_LOG to see logs!");

    warn!("initializing the RNG!");
    // create an RNG instance
    let mut rng = Rng::new(peripherals.RNG);

    // create a key buffer and fill it
    let mut key = [0u8; 32];
    rng.read(&mut key);
    trace!("Key: {:?}", key);

    // create a string buffer
    let mut buf = *b"string buffer 19";

    // encrypt and decrypt
    let mut aes = Aes::new(peripherals.AES);
    aes.process(&mut buf, Mode::Encryption256, key);
    info!("AES[enc]: {:?}", buf);
    aes.process(&mut buf, Mode::Decryption256, key);
    debug!("AES[dec]: {:?}", buf);

    // can we get back the string?
    let buf_as_vec = Vec::<_, 16>::from_slice(&buf).unwrap();
    let buf_as_str = String::<16>::from_utf8(buf_as_vec).unwrap();
    info!("decoded AES[dec]: {}", &*buf_as_str);

    // compute hash
    let mut sha = Sha::new(peripherals.SHA, ShaMode::SHA256, None);
    sha.update(&buf).ok();
    sha.finish(&mut buf).ok();
    info!("SHA: {:?}", buf);

    let (tx, rx) = xchg::channel(&SHARED);

    spawner.spawn(producer(tx)).unwrap();
    spawner.spawn(producer2(tx)).unwrap();
    spawner.spawn(producer3(tx)).unwrap();

    loop {
        let (produer, value) = rx.recv().await;
        trace!("[{}] produced value = {}", produer, value);
    }
}
