use std::process::exit;
use clap::{App, Arg};

mod data;
mod install;
use data::*;

/// Determine if the given path exists *and* is a file
fn is_file(path: &str) -> bool
{
    std::path::Path::new(path).is_file()
}

/// Read a file's contents and return a String or, if reading failed, an Error.
fn read_file(path: &str) -> Result<String, std::io::Error>
{
    std::fs::read_to_string(path)
}

fn main() -> Result<(), std::io::Error>
{
    let cli_args = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author("xylous <xylous.e@gmail.com>")
        .about("Arch installer using YAML files")
        .arg(Arg::new("FILE")
            .short('f')
            .long("--file")
            .takes_value(true)
            .help("sets the input file"))
        .arg(Arg::new("flag_sample_file")
            .short('s')
            .long("--sample")
            .help("prints a sample file to stdout"))
        .get_matches();

    if cli_args.is_present("FILE") {
        let path = cli_args.value_of("FILE").unwrap();
        if !is_file(path) {
            eprintln!("error: provided path is not a file");
            exit(1);
        }

        let contents = read_file(path)?;
        let parsed: ParsedInstallOptions = serde_yaml::from_str(&contents).unwrap();
        let proper = InstallOptions::from(parsed);
        print!("{}", proper.generate_shellscript());
    } else if cli_args.is_present("flag_sample_file") {
        print!("{}", sample_input_file());
    }

    Ok(())
}
