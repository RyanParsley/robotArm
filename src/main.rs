#![no_std]
#![no_main]

mod robot_arm;

use embassy_executor::Spawner;
use embassy_rp::pwm::{Config, Pwm};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let c: Config = Default::default();

    // set up one channel to control 2 pins via PWM.
    let pwm_gripper_horizontal = Pwm::new_output_ab(p.PWM_CH7, p.PIN_14, p.PIN_15, c.clone());

    // set up another channel to control the 2 pins responsible for vertical movement.
    let pwm_vertical = Pwm::new_output_ab(p.PWM_CH6, p.PIN_12, p.PIN_13, c.clone());

    robot_arm::run(c, pwm_gripper_horizontal, pwm_vertical).await;
}
