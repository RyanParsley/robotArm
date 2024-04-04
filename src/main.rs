#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::pwm::{Config, Pwm};
use embassy_time::Timer;
use fixed::FixedU16;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let mut c: Config = Default::default();
    // This is a magic number that I need to better understand.
    c.top = 32768;

    // set up one channel to control 2 pins via PWM.
    let mut pwm_gripper_horizontal = Pwm::new_output_ab(p.PWM_CH7, p.PIN_14, p.PIN_15, c.clone());

    // set up another channel to control the 2 pins responsible for vertical movement.
    let mut pwm_vertical = Pwm::new_output_ab(p.PWM_CH6, p.PIN_12, p.PIN_13, c.clone());

    // This formula is provided in the docs:
    // frequency = (c.top + 1) * (c.phase_correct ? 1 : 2) * divider;
    // Assuming 50hz, I'm solving for the divider value.
    c.divider = FixedU16::from_num(c.top / 50000);

    // Function to convert angle to PWM duty
    // TODO: double check the math on min and max
    fn angle_to_duty(angle: i32) -> u16 {
        let min_duty = 175.0;
        let max_duty = 1300.0;
        (min_duty + (angle as f32 / 180.0) * (max_duty - min_duty)) as u16
    }

    loop {
        // Sweep from 0 degrees to 180 degrees
        for angle in 0..180 {
            let duty_cycle = angle_to_duty(angle);

            println!("Current Angle {} Degrees", angle);
            println!("duty_cycle {}", duty_cycle);

            // Hello world will just make all the servos wipe back and forth.
            c.compare_b = duty_cycle;
            c.compare_a = duty_cycle;

            pwm_gripper_horizontal.set_config(&c);
            pwm_vertical.set_config(&c);

            Timer::after_millis(100).await;
        }
    }
}
