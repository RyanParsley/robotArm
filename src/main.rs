#![no_std]
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

//This macro defines the right interrupt handlers, and creates a unit struct (like struct Irqs;) and implements the right Bindings for it.
//You can pass this struct to drivers to prove at compile-time that the right interrupts have been bound. 
bind_interrupts!(struct Irqs {
    ADC_IRQ_FIFO => InterruptHandler;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("Initializing...");

    let peripherals = embassy_rp::init(Default::default());

    let mut adc = Adc::new(peripherals.ADC, Irqs, Config::default());

    // Notes on servos
    // Since just about every GPIO on the Pico is appropriate for PWM, I can put the servos
    // anywhere. I plan to connect motors to the following pins 18, 19, 20 & 21 because it's a
    // block of 4 that fits between 2 GNDs on the side of the board that happens to be near the
    // robot as my breadboard is mounted. That is to say, no great reason for this choice. 
 
    // Notes on Potentiometers and the Pico.
    // Potentiometers need to be on ADC Ports so 26, 27 and 28 are the only appropriate pins.
    // ADC_VREF and AGND are special power and ground for working with ADC. 

    let mut pot1 = Channel::new_pin(peripherals.PIN_26, Pull::None);

    info!("Initialized.");

    loop {
        // TODO: When I use the 100k potentiometer, this unwrap fails gloriously
        // [issue](https://github.com/RyanParsley/robotArm/issues/1).
        // What's the right way to handle that?
        // Why does it happen?
        //
        // ERROR panicked at src/main.rs:37:47:
        // called `Result::unwrap()` on an `Err` value: ConversionFailed

        let level = adc.read(&mut pot1).await.unwrap();
        info!("Pot1 ADC: {}", level);

        Timer::after(Duration::from_millis(1000)).await;
    }
}
