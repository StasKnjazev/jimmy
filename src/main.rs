use std::process::exit;

use clap::{App, Arg};

/// Determine if the given path exists *and* is a file
fn is_file(path: &str) -> bool
{
    std::path::Path::new(path).is_file()
}

fn main()
{
    let cli_args = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author("xylous <xylous.e@gmail.com>")
        .about("Arch installer using YAML files")
        .arg(Arg::new("FILE")
            .required(true))
        .get_matches();

    let path = cli_args.value_of("FILE").unwrap();
    if !is_file(path) {
        eprintln!("error: provided path is not a file");
        exit(1);
    }
}
