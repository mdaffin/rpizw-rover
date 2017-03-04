# rpizw-rover
A raspberry pi zero w based rover

To create the image run `sudo ./create-image`. You can pass in an optional size
to create an image of that size like `sudo ./create-image 4G`, it defaults to
2G. Burn the image to an sd card by running `sudo dd if=rpizw-rover.img
of=/dev/sdX` where /dev/sdX is the card you want to use.

Once the pi has booted you can connect to it via a serial interface over a usb
cable by running `picocom /dev/ttyAMA0`. From here you can set the wireless
credentials by running `wpa_passphrase "<SSID>" "<PASSPHRASE>" >
/etc/wpa_supplicant/wpa_supplicant-wlan0.conf`. `ip addr` will tell you its
ipaddress so you can ssh into it allowing you to run completly wirelessly.

Once in the pi run `sudo ./rover-test.sh` to see if it works. It should move
forward for a second, then back, turn left, turn right and finally stoping
before repeating the pattern. Hit `ctrl+c` to stop the script (and the robot).
