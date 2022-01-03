# jimmy

An Arch installer with a funny name.

It's not interactive. It takes a YAML file as input, checks if it's valid,
generates a Shell script based on it and runs that.

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

<!--
### Usage

(How is this software used?)
-->

## Roadmap

- [ ] provide example YAML file
- [ ] parse YAML file (with `serde`)
- [ ] check if the input file is valid
    - [ ] error if e.g. `editor` isn't specified
- [ ] prepare installation
    - [ ] update system clock
    - [ ] partitions
        - [ ] create with FDISK
        - [ ] format
        - [ ] mount filesystems
    - [ ] install *all* specified packages
- [ ] configure the system
    - [ ] generate an fstab file
    - [ ] set timezone
    - [ ] set locales
    - [ ] configure network
    - [ ] create initramfs
    - [ ] configure users + root
    - [ ] configure bootloader
        - [ ] GRUB
        - [ ] efistub

## Contributing

Pull requests are welcome. For major changes, please open an issue first to
discuss what you would like to change.

Please make sure to update tests as appropriate.
