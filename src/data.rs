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
#[derive(Debug)]
pub enum Kernel {
    Latest,
    Lts,
}

/// Struct that contains the minimum needed to create a functioning Arch installation
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

impl InstallOptions
{
    /// Create a new instance of `InstallOptions` from an instance of `ParsedInstallOptions`
    pub fn new(raw: ParsedInstallOptions) -> Self
    {
        let kernel = match raw.kernel.unwrap_or_default().as_str() {
            "latest" => Kernel::Latest,
            _ => Kernel::Lts, // assume LTS kernel at all times
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

/// Struct that contains the minimum needed to create a partition on disk
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
    /// Create a new instance of `Partition` from an instance of `ParsedPartition`
    pub fn new(raw: ParsedPartition) -> Self
    {
        let format: String;
        if raw.format.is_none() || raw.format.as_ref().unwrap() == "" {
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
            size: raw.size.unwrap_or_else(|| "".to_string()),
            mount: raw.mount.unwrap_or_else(|| "".to_string()),
        }
    }
}
