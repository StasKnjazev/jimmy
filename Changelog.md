# Changelog

## 0.8.0 - 2022-01-23

- change: use `-f`, `--file` option instead of passing filename directly
- add: `-s`, `--sample` option to print a simple file it accepts as input

## 0.7.2 - 2022-01-23

- fix: make second script executable before running arch-chroot
- fix: if partition size is not specified, don't put a plus sign
- fix: add `/mnt` before mount point
- fix: fdisk selecting wrong disk type
- fix: don't unwrap `extra` every time

## 0.7.1 - 2022-01-22

- panic if zoneinfo isn't valid
- add example files related to zoneinfo
- refactor: rename files in `examples/` directory

## 0.7.0 - 2022-01-16

- fix: put shebang at the start of the script so it runs
- add code for configuring GRUB
- add example with EFISTUB as bootloader
- add code for configuring EFISTUB

## 0.6.0 - 2022-01-10

- add code to set up networking by using NetworkManager
- add code to prompt the user to set the root user's password
- fix logical mistake where `locale-gen` wasn't being ran after setting up the locales

## 0.5.0 - 2022-01-10

- add `locales` property as *optional*; if not specified, `en_US.UTF-8` is
assumed to be the only locale
- fix spacing error in arch-chroot command
- make README more legible

## 0.4.0 - 2022-01-09

- add shell code (in the output) for:
    - genfstab
    - arch-chroot
    - setting timezones
- add properties `region` and `city` as mandatory in the input files & update
tests
- improve readability of some code

## 0.3.5 - 2022-01-08

- add shell code to update system time
- add shell code to install specified packages

## 0.3.4 - 2022-01-08

- add support for swap partitions
- refactor code

## 0.3.3 - 2022-01-08

- when generating fdisk commands:
    - create a GPT partition table by default
    - change the type of the partitions so that they can be used with the format
    - use partition sizes relative from the start sector, instead of absolute
    sizes

## 0.3.2 - 2022-01-07

- add usage section to README.md
- print warnings during parse time to stderr
- add examples where an error/a warning is purposefully thrown
- fix bug caused by using the wrong variable

## 0.3.1 - 2022-01-07

- create `mount` calls only if the mount point is specified for a partition

## 0.3.0 - 2022-01-07

- add examples with multiple partitions and multiple disks
- add functions to generate code for:
    - `fdisk` calls, to create partitions
    - `mkfs` calls, to format the partitions
    - `mount` calls, to mount the partitions-proper
- generate a shell script and print it to stdout

## 0.2.0 - 2022-01-05

- add definitions for both parsed and definite data
- in main(): read file specified in CLI args as parsed data
- add functions to convert parsed data into definite data

## 0.1.0 - 2022-01-03

- add example input file
- parse CLI arguments with clap-rs

## 0.0.0 - 2022-01-03

- create project (README, Changelog, `cargo init` etc)
