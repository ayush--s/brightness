# Brightness

##### A cli tool to alter screen brightness in linux systems with intel_backlight

(needs root access to run)

### usage:
```bash
$ sudo brightness -i 5  # 5% increase
$ sudo brightness -d 5  # 5% decrease
```

### Installation
Get the latest build from [releases](https://github.com/ayush--s/brightness/releases) page. Or,

```bash
$ git clone https://github.com/ayush--s/brightness
$ cd brightness && cargo install
```

### full usage:
```bash
$ sudo brightness -h
brightness 0.1
change brightness of intel backlight on linux

USAGE:
    brightness [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --decrease <dec>    decrease brightness
    -i, --increase <inc>    increase brightness
```
