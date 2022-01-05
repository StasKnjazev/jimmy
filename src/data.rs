#[allow(dead_code)]

use serde::Deserialize;

/// *Potentially* valid installation options. Everything is wrapped in `Option<T>` because serde
/// would error if the property isn't found.
#[derive(Deserialize, Debug)]
pub struct ParsedInstallOptions
{
    pub username: Option<String>,
    pub hostname: Option<String>,
    pub kernel: Option<String>,
    pub extra: Option<String>,
    pub bootloader: Option<String>,
    pub partitions: Option<Vec<ParsedPartition>>,
}

/// *Potentially* valid partition options. Everything is wrapped in `Option<T>` because serde would
/// error if the property isn't found.
#[derive(Deserialize, Debug)]
pub struct ParsedPartition
{
    pub format: Option<String>,
    pub disk: Option<String>,
    pub size: Option<String>,
    pub mount: Option<String>,
}

/// Guaranteed valid installation options. Can only be constructed from a `struct
/// ParsedInstallOptions`
#[derive(Debug)]
pub struct InstallOptions
{
    pub username: String,
    pub hostname: String,
    pub kernel: String,
    pub extra: String,
    pub bootloader: String,
    pub partitions: Vec<Partition>,
}

/// Guaranteed valid partition. Can only be constructed from a `struct ParsedPartition`
#[derive(Debug)]
pub struct Partition
{
    pub format: String,
    pub disk: String,
    pub size: String,
    pub mount: String,
}

impl Partition
{
    pub fn new(raw: ParsedPartition) -> Self
    {
        let mut format = "";
        if raw.format.is_none() || raw.mount.as_ref().unwrap() == "" {
            println!("warning: partition format not specified; defaulting to 'ext4'");
            format = "ext4";
        }
        if raw.mount.is_none() || raw.mount.as_ref().unwrap() == "" {
            println!("warning: partition mount not specified; it's not going to be mounted");
        }
        Self {
            format: format.to_string(),
            disk: raw.disk.expect("error: partition disk not specified"),
            size: raw.size.unwrap_or("".to_string()),
            mount: raw.mount.unwrap_or("".to_string()),
        }
    }
}
