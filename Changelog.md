# Changelog

## 0.3.2 - 2021-01-07

- add usage section to README.md
- print warnings during parse time to stderr
- add examples where an error/a warning is purposefully thrown
- fix bug caused by using the wrong variable

## 0.3.1 - 2021-01-07

- create `mount` calls only if the mount point is specified for a partition

## 0.3.0 - 2021-01-07

- add examples with multiple partitions and multiple disks
- add functions to generate code for:
    - `fdisk` calls, to create partitions
    - `mkfs` calls, to format the partitions
    - `mount` calls, to mount the partitions-proper
- generate a shell script and print it to stdout

## 0.2.0 - 2021-01-05

- add definitions for both parsed and definite data
- in main(): read file specified in CLI args as parsed data
- add functions to convert parsed data into definite data

## 0.1.0 - 2022-01-03

- add example input file
- parse CLI arguments with clap-rs

## 0.0.0 - 2022-01-03

- create project (README, Changelog, `cargo init` etc)
