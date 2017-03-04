#!/bin/bash
set -uo pipefail
trap 's=$?; echo "$0: Error on line "$LINENO": $BASH_COMMAND"; exit $s' ERR
IFS=$'\n\t'

chip0=/sys/class/pwm/pwmchip0
pwm0=$chip0/pwm0
pwm1=$chip0/pwm1

pi_disable() {
    echo 0 > $pwm0/enable
    echo 0 > $pwm1/enable
    echo 0 > $chip0/unexport
    echo 1 > $chip0/unexport
}
trap pi_disable EXIT

pi_enable() {
    echo 0 > $chip0/export
    echo 1 > $chip0/export
    echo 50000000  > $pwm0/period
    echo 50000000  > $pwm1/period
    echo 1 > $pwm0/enable
    echo 1 > $pwm1/enable
}

pi_forward() {
    echo 1000000 > $pwm0/duty_cycle
    echo 2000000 > $pwm1/duty_cycle
}

pi_backward() {
    echo 2000000 > $pwm0/duty_cycle
    echo 1000000 > $pwm1/duty_cycle
}

pi_stop() {
    echo 1500000 > $pwm0/duty_cycle
    echo 1500000 > $pwm1/duty_cycle
}

pi_right() {
    echo 1000000 > $pwm0/duty_cycle
    echo 1000000 > $pwm1/duty_cycle
}

pi_left() {
    echo 2000000 > $pwm0/duty_cycle
    echo 2000000 > $pwm1/duty_cycle
}

pi_enable

while true; do
    pi_forward
    sleep 1
    pi_backward
    sleep 1
    pi_left
    sleep 1
    pi_right
    sleep 1
    pi_stop
    sleep 1
done
