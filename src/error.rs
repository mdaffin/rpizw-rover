use sysfs_pwm;

error_chain!{
    foreign_links {
        PWM(sysfs_pwm::Error);
    }
}