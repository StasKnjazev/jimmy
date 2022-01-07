use crate::data::{InstallOptions, Partition};

#[allow(dead_code)]
impl InstallOptions
{
    /// Return the list of all unique disks used in the configuration
    fn unique_disks_used(&self) -> Vec<String>
    {
        let mut disks: Vec<String> = self.partitions.iter()
            .map(|p| p.disk.clone())
            .collect();
        disks.sort();
        disks.dedup();
        disks
    }

    /// Given the name of a disk, return a list of all partitions that would be on it
    fn partitions_on_disk(&self, disk: &str) -> Vec<&Partition>
    {
        self.partitions
            .iter()
            .filter(|x| x.disk == disk)
            .collect()
    }
}
