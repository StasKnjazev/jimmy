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
#[derive(Deserialize, Debug, Clone)]
pub struct ParsedPartition
{
    pub format: Option<String>,
    pub disk: Option<String>,
    pub size: Option<String>,
    pub mount: Option<String>,
}

/// Only the Latest or the LTS kernel can be installed
#[allow(dead_code)]
#[derive(Debug)]
pub enum Kernel {
    Latest,
    LTS,
}

/// Guaranteed valid installation options. Can only be constructed from a `struct
/// ParsedInstallOptions`
#[derive(Debug)]
pub struct InstallOptions
{
    pub username: String,
    pub hostname: String,
    pub kernel: Kernel,
    pub extra: String,
    pub bootloader: String,
    pub partitions: Vec<Partition>,
}

#[allow(dead_code)]
impl InstallOptions
{
    pub fn new(raw: ParsedInstallOptions) -> Self
    {
        let kernel = match raw.kernel.unwrap_or_default().as_str() {
            "latest" => Kernel::Latest,
            _ => Kernel::LTS, // assume LTS kernel at all times
        };
        Self {
            username: raw.username.expect("error: username not specified"),
            hostname: raw.hostname.expect("error: hostname not specified"),
            kernel,
            extra: raw.extra.unwrap(),
            bootloader: raw.bootloader.expect("error: no bootloader specified"),
            // turn every `ParsedPartition` into a proper `Partition`
            partitions: raw.partitions.expect("error: no partitions specified")
                            .iter().map(|p| Partition::new(p.clone())).collect(),
        }
    }
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

#[allow(dead_code)]
impl Partition
{
    pub fn new(raw: ParsedPartition) -> Self
    {
        let format: String;
        if raw.format.is_none() || raw.mount.as_ref().unwrap() == "" {
            eprintln!("warning: partition format not specified; defaulting to 'ext4'");
            format = "ext4".to_string();
        } else {
            format = raw.format.unwrap();
        }
        if raw.mount.is_none() || raw.mount.as_ref().unwrap() == "" {
            eprintln!("warning: partition mount not specified; it's not going to be mounted");
        }
        Self {
            format,
            disk: raw.disk.expect("error: partition disk not specified"),
            size: raw.size.unwrap_or("".to_string()),
            mount: raw.mount.unwrap_or("".to_string()),
        }
    }
}
