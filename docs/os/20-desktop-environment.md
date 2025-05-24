# Setup Desktop Environment

* Window Manager: `sway`, `swaylock`, `swayidle`, `swaybg`
* Login Screen: `greetd`, `nwg-hello`
* App Launcher: `rofi`
* Terminal Emulator: `ghostty`


```sh
pacman --sync sway swaylock swayidle swaybg greetd nwg-hello swaync xorg-xwayland waybar
```

Activate the login screen (nwg-hello) on boot with Greetd.

```sh
systemctl enable greetd
```

Set up the Pipewire sound system
```sh
pacman --sync pipewire-audio pipewire-alsa pipewire-pulse pipewire-jack 
```
