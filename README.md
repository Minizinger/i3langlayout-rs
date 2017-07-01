# i3langlayout-rs
Language layout manager for i3 windows manager written in rust

## Instalation
Available on AUR for arch linux users: https://aur.archlinux.org/packages/i3langlayout/  
For other distros follow these simple steps:  
* git clone  
* cargo build --release  
* copy ./target/release/i3langlayout to wherever your binary files go  

To start it up automatically, add this line to your i3 config  
```bash
exec --no-startup-id i3langlayout
```

## Important
```bash
usermod -aG input your-user
```
For program to work you have to either be user that can read input events (usually input group, but may vary from distro to distro, just check access rights for events in /dev/input/) or you can start the program with superuser rights, but this is not recommended

## Config
Config file is located in ~/.config/i3langlayout/config.toml  
Right now it has only 3 fields:  
* layouts - list of keyboard layouts to cycle through, values are strings of xkblayouts, full list of options you can find in /usr/share/X11/xkb/symbols  
* separate_workspaces - boolean value that indicates if each workspace should have separate layout  
* hotkey - list of two values that indicate key code to switch layout. Full list of keycodes you can find in [linux headers](https://github.com/torvalds/linux/blob/master/include/uapi/linux/input-event-codes.h)

Example of config file
```toml
layouts=["us", "il"]
separate_workspaces=true
hotkey=[56, 42]
```
It will cycle between English and Hebrew layouts on key combination Left Alt+Shift, and each workspace will have separate layout settings
