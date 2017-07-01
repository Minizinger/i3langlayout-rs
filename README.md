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
