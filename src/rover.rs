use error::*;
use sysfs_pwm::Pwm;

const PERIOD: u32 = 20_000_000;
const MAX_DUTY_CYCLE: u32 = 2_000_000;
const MIN_DUTY_CYCLE: u32 = 1_000_000;

pub struct Rover {
    left: Pwm,
    right: Pwm,
}

impl Rover {
    /// Creates a new rovers with both wheels enabled but stopped. The wheels
    /// will be disabled and the underlying pwm drivers unexported when the
    /// rover is dropped.
    pub fn new(chip: u32, left_pin: u32, right_pin: u32) -> Result<Rover> {
        let left = Pwm::new(chip, left_pin).chain_err(|| "failed to create left wheel")?;
        let right = Pwm::new(chip, right_pin).chain_err(|| "failed to create right wheel")?;
        left.export().chain_err(|| "failed to export the left wheel pwm channel")?;
        right.export().chain_err(|| "failed to export the right wheel pwm channel")?;
        left.set_period_ns(PERIOD).chain_err(|| "failed to set period on left wheel")?;
        right.set_period_ns(PERIOD).chain_err(|| "failed to set period on right wheel")?;
        Ok(Rover {
            left: left,
            right: right,
        })
    }

    /// Enables/disables the wheel. When disabled they keep their current
    /// speed and their speed can still be set but they will not move until
    /// enabled.
    pub fn enable(&self, enabled: bool) -> Result<()> {
        self.left.enable(enabled).chain_err(|| "failed to enable left wheel")?;
        self.right.enable(enabled).chain_err(|| "failed to enable right wheel")
    }

    /// Converts a speed between -100 (full reverse) and 100 (full forward)
    /// to a duty cycle.
    fn speed_to_duty_cycle(speed: i8) -> u32 {
        let duty_cycle = (((speed as i32 * 10000) + MIN_DUTY_CYCLE as i32) as u32 / 2) +
                         MIN_DUTY_CYCLE;
        if duty_cycle > MAX_DUTY_CYCLE {
            return MAX_DUTY_CYCLE;
        }
        if duty_cycle < MIN_DUTY_CYCLE {
            return MIN_DUTY_CYCLE;
        }
        duty_cycle
    }

    /// Sets the speed of the left wheel. Can be any value between -100 (full
    /// reverse) and 100 (full forward), values above or below these limits will
    /// be to to the limit.
    pub fn set_left_speed(&self, speed: i8) -> Result<()> {
        self.left
            .set_duty_cycle_ns(Rover::speed_to_duty_cycle(-speed))
            .chain_err(|| "failed to set duty on left wheel")
    }

    /// Sets the speed of the right wheel. Can be any value between -100 (full
    /// reverse) and 100 (full forward), values above or below these limits will
    /// be to to the limit.
    pub fn set_right_speed(&self, speed: i8) -> Result<()> {
        self.right
            .set_duty_cycle_ns(Rover::speed_to_duty_cycle(speed))
            .chain_err(|| "failed to set duty on left wheel")
    }

    /// Stops both the wheels, equlivent to setting their speeds to 0.
    pub fn stop(&self) -> Result<()> {
        self.set_left_speed(0)?;
        self.set_right_speed(0)
    }

    /// Sets the speed of left and right wheel. Can be any value between -100 (full
    /// reverse) and 100 (full forward), values above or below these limits will
    /// be to to the limit.
    pub fn set_speed(&self, left: i8, right: i8) -> Result<()> {
        self.set_left_speed(left)?;
        self.set_right_speed(right)
    }

    /// Unexports the wheels so they can no longer be used
    pub fn unexport(self) -> Result<()> {
        self.left.enable(false).chain_err(|| "failed to disable left wheel")?;
        self.right.enable(false).chain_err(|| "failed to disable right wheel")?;
        self.left.unexport().chain_err(|| "failed to unexport left wheel")?;
        self.right.unexport().chain_err(|| "failed to unexport right wheel")
    }
}
