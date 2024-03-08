#![no_std]
// Why do I get an error about "can't find crate for `test`?" on no_std?
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_rp::gpio::Pull;
use embassy_rp::adc::{Adc, Channel, Config, InterruptHandler};
use embassy_rp::bind_interrupts;
use embassy_time::{ Duration, Timer};
use panic_probe as _;
use defmt::info;

// TODO: research all of these
bind_interrupts!(struct Irqs {
    ADC_IRQ_FIFO => InterruptHandler;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("Initializing...");

    // I get that this is of type Peripherals, but what is it?
    let peripherals = embassy_rp::init(Default::default());

    let mut adc = Adc::new(peripherals.ADC, Irqs, Config::default());

    // Motors 18, 19, 20 & 21
    // Potentiometers need to be on ADC Ports so 26, 27 and 28 are the only appropriate pins.
    // ADC_VREF and AGND are special power and ground for working with ADC. 

    let mut pot1 = Channel::new_pin(peripherals.PIN_26, Pull::None);

    info!("Initialized.");

    loop {
        // When I use the 100k potentiometer, this unwrap fails gloriously.
        // What's the right way to handle that?
        // Why does it happen?
        // ERROR panicked at src/main.rs:37:47:
        // called `Result::unwrap()` on an `Err` value: ConversionFailed

        let level = adc.read(&mut pot1).await.unwrap();
        info!("Pot1 ADC: {}", level);

        Timer::after(Duration::from_millis(1000)).await;
    }
}
