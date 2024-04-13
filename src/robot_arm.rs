use defmt::info;
use embassy_rp::{
    peripherals::{PWM_CH6, PWM_CH7},
    pwm::{Config, Pwm},
};
use embassy_time::Timer;
use fixed::FixedU16;

// Function to convert angle to PWM duty
// TODO: double check the math on min and max
fn angle_to_duty(angle: i32) -> u16 {
    let min_duty = 175.0;
    let max_duty = 1300.0;
    (min_duty + (angle as f32 / 180.0) * (max_duty - min_duty)) as u16
}

pub async fn run(
    mut config: Config,
    mut pwm_gripper_horizontal: Pwm<'_, PWM_CH7>,
    mut pwm_vertical: Pwm<'_, PWM_CH6>,
) {
    // This is a magic number that I need to better understand.
    config.top = 32768;

    // This formula is provided in the docs:
    // frequency = (c.top + 1) * (c.phase_correct ? 1 : 2) * divider;
    // Assuming 50hz, I'm solving for the divider value.
    config.divider = FixedU16::from_num(config.top / 50000);
    loop {
        // Sweep from 0 degrees to 180 degrees
        for angle in 0..180 {
            let duty_cycle = angle_to_duty(angle);
            info!("Current Angle {} Degrees", angle);
            info!("duty_cycle {}", duty_cycle);

            // Hello world will just make all the servos wipe back and forth.
            config.compare_b = duty_cycle;
            config.compare_a = duty_cycle;

            pwm_gripper_horizontal.set_config(&config);
            pwm_vertical.set_config(&config);

            Timer::after_millis(100).await;
        }
    }
}
