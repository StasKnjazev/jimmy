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
