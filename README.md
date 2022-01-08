# jimmy

An Arch installer with a funny name.

It's not interactive. It takes a YAML file as input, checks if it's valid,
generates a Shell script based on it.

## Getting started

### Requirements

- cargo, rust's package manager

### Installation

#### Manual

Clone this repository and build, e.g.:

```
git clone https://github.com/xylous/jimmy jimmy
cd jimmy
cargo build
```

### Usage

Synopsis:

```
jimmy <FILE>
```

`jimmy` will then proceed to generate a shell script based on the input file,
warn you of missing/important properties, and error if there some vital ones
(such as `hostname`) aren't specified. It is up to you to redirect the output to
a file and execute it with a shell.

## Roadmap

- [x] provide example YAML file
- [x] parse YAML file (with `serde`)
- [x] check if the input file is valid
    - [x] error if e.g. `username` isn't specified
- [x] prepare installation
    - [x] update system clock
    - [x] partitions
        - [x] create with fdisk
        - [x] format
        - [x] mount filesystems
    - [x] install *all* specified packages
- [ ] configure the system
    - [ ] generate an fstab file
    - [ ] set timezone
    - [ ] set locales
    - [ ] configure network
    - [ ] create initramfs
    - [ ] ~~configure users + root~~
    - [ ] configure bootloader
        - [ ] GRUB
        - [ ] efistub

## Contributing

Pull requests are welcome. For major changes, please open an issue first to
discuss what you would like to change.

Please make sure to update tests as appropriate.
