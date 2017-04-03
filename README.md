# rpizw-rover
A raspberry pi zero w based rover.

## Cross Compile Setup

[cross compile for arm](https://github.com/japaric/rust-cross)

```shell
# ubuntu
sudo apt-get install -qq gcc-arm-linux-gnueabihf
# arch aur package: arm-linux-gnueabihf-gcc
rustup update stable
rustup override stable
rustup target add arm-unknown-linux-gnueabihf
```

## Building The Raspberry Pi Image

```shell
cargo build --release --target=arm-unknown-linux-gnueabihf
cd ui
npm install && npm run build
cd -
sudo ./create-image
```

## USB Serial Interface And Connecting To Wifi

Once the pi has booted you can connect to it via a serial interface over a usb
cable by running `picocom /dev/ttyAMA0`, or using putty on windows. From here
you can set the wireless credentials by running `wpa_passphrase "<SSID>"
"<PASSPHRASE>" >> /etc/wpa_supplicant/wpa_supplicant-wlan0.conf`. Once done you
can ssh to the pi with `ssh rpizw-rover.local` if you have zero-conf/avahi
configured or you can get the ip address over the serial connection by running
`ip addr`.

The webserver is available at http://rpizw-rover.local:3000, or the ip address
obtained above.
