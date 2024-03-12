#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::info;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_rp::adc::{Adc, Channel, Config, InterruptHandler};
use embassy_rp::bind_interrupts;
use embassy_rp::gpio::Pull;
use embassy_time::{Duration, Timer};
use panic_probe as _;

/*
* This macro defines the right interrupt handlers, creates a unit struct and implements the right Bindings for it.
* You can pass this struct to drivers to prove at compile-time that the right interrupts have been bound.
*/
bind_interrupts!(struct Irqs {
    ADC_IRQ_FIFO => InterruptHandler;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("Initializing...");

    let peripherals = embassy_rp::init(Default::default());

    let mut adc = Adc::new(peripherals.ADC, Irqs, Config::default());

    /* ---------------
     * Notes on servos
     * ---------------
     * Since just about every GPIO on the Pico is appropriate for PWM, I can put the servos
     * anywhere. I plan to connect motors to the following pins 18, 19, 20 & 21 because it's a
     * block of 4 that fits between 2 GNDs on the side of the board that happens to be near the
     * robot as my breadboard is mounted. That is to say, no great reason for this choice.
     */

    /* ------------------------------------
     * Notes on Potentiometers and the Pico
     * ------------------------------------
     * Potentiometers need to be on ADC Ports so 26, 27 and 28 are the only appropriate pins.
     *
     * DON'T USE ADC_VREF!
     * Use the standard 3v3 pin for power.
     */

    let mut pot1 = Channel::new_pin(peripherals.PIN_26, Pull::None);

    info!("Initialized.");

    loop {
        let level = adc.read(&mut pot1).await.unwrap();
        info!("Pot1 ADC: {:?}", level);

        Timer::after(Duration::from_millis(1000)).await;
    }
}
