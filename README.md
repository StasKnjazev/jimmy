# jimmy

Have you ever considered just how boring entering your options manually into an
interactive installer is? And what if you forget something? What if you change
your mind about a single, perhaps small, thing? You're most likely going to have
to start from scratch again.

The alternative is to write your own shell script, which is often long and
confusing. Plus, if you aren't really tidy, you're going to have a hard time
changing things.

Enter: `jimmy`, an Arch installer unlike any other. The idea is simple: store
all preferences/settings in a YAML file and generate a shell script from it.
Both problems are solved: your configuration is now *very* concise. If you want
to change something, simply edit the file and re-run jimmy.

There's one clear drawback: that it's not as customisable as writing your own
shell script. Sure, it might seem limiting. But 1) it can generate shell script
for most things you'd need in a minimal Arch installation and 2) if you want
customisation, you can edit the output shell script anyways

What it can do:
- print a template YAML file that you can then edit and feed it
- partition disks (this includes creating the partitions, formatting, mounting
them, and creating the fstab file)
- install the packages you tell it to
- set timezone and generate locales
- set up NetworkManager
- prompt you for a root password
- install and configure GRUB *or* EFISTUB
- create users (usernames, groups, etc.). Note that the `wheel` group is always
    capable of using sudo.
- set a default shell for a user

What it can't do:
- connect to the internet (you must do that youself)
- set up mirrors and gpg keys (you must also do that)
- set up graphical environments

## Getting started

### Requirements

- cargo, rust's package manager

### Installation

Install from crates.io directly:

```
cargo install jimmy
```

### Usage

Synopsis:

```
jimmy [-f | --file | -s | --sample] [<ARGS>]
```

`jimmy` will then proceed to generate a shell script and print it to `stdout`,
warning you of missing properties, and error if some vital ones (such as
`hostname`) aren't specified. It's up to you to redirect the output to a file
and execute it with a shell.

Here's an example using concrete commands:

```
jimmy --sample >input.yaml
vim input.yaml
jimmy --file input.yaml >script.sh
chmod +x script.sh
```

WARNING: Do NOT run it, except in an Arch live system! You *can* lose data!

## Roadmap

- [x] provide example YAML file
- [x] parse YAML file (with `serde`)
- [x] check if the input file is valid
    - [x] error if e.g. `username` isn't specified
    - [x] panic if zoneinfo isn't valid
- [x] prepare installation
    - [x] update system clock
    - [x] partitions
        - [x] create with fdisk
        - [x] format
        - [x] mount filesystems
    - [x] install *all* specified packages
- [x] configure the system
    - [x] generate an fstab file
    - [x] set timezone
    - [x] set locales
    - [x] configure network
    - [x] configure users
    - [x] set root's password
    - [x] configure bootloader
        - [x] GRUB
        - [x] efistub
- [x] generate template YAML file on the spot
- [x] print status messages while installing

## Contributing

Pull requests are welcome. For major changes, please open an issue first to
discuss what you would like to change.

Please make sure to update tests as appropriate.
