extern crate sysfs_pwm;
use sysfs_pwm::{Pwm, Result};

// PIN: EHRPWM0A (P9_22)
const BB_PWM_CHIP: u32 = 0;
const BB_PWM_NUMBER: u32 = 0;

fn pwm_increase_to_max(pwm: &Pwm, duration_ms: u32, update_period_ms: u32) -> Result<()> {
    let step: f32 = duration_ms as f32 / update_period_ms as f32;
    let mut duty_cycle = 0.0;
    let period_ns: u32 = try!(pwm.get_period_ns());
    while duty_cycle < 1.0 {
        try!(pwm.set_duty_cycle_ns((duty_cycle * period_ns as f32) as u32));
        duty_cycle += step;
    }
    pwm.set_duty_cycle_ns(period_ns)
}

fn pwm_decrease_to_minimum(pwm: &Pwm, duration_ms: u32, update_period_ms: u32) -> Result<()> {
    let step: f32 = duration_ms as f32 / update_period_ms as f32;
    let mut duty_cycle = 1.0;
    let period_ns: u32 = try!(pwm.get_period_ns());
    while duty_cycle > 0.0 {
        try!(pwm.set_duty_cycle_ns((duty_cycle * period_ns as f32) as u32));
        duty_cycle -= step;
    }
    pwm.set_duty_cycle_ns(0)
}

/// Make an LED "breathe" by increasing and
/// decreasing the brightness
fn main() {
    let pwm = Pwm::new(BB_PWM_CHIP, BB_PWM_NUMBER).unwrap(); // number depends on chip, etc.
    pwm.with_exported(|| {
            pwm.enable(true).unwrap();
            pwm.set_period_ns(20_000).unwrap();
            loop {
                pwm_increase_to_max(&pwm, 1000, 20).unwrap();
                pwm_decrease_to_minimum(&pwm, 1000, 20).unwrap();
            }
        })
        .unwrap();
}
