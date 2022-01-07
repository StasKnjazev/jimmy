use crate::data::{InstallOptions, Partition};

#[allow(dead_code)]
impl InstallOptions
{
    /// Return the list of shell commands that create the partitions with `fdisk`
    pub fn fdisk_cmd(&self) -> Vec<String>
    {
        let disks = self.unique_disks_used();

        let mut cmds = Vec::new();
        for disk in disks {
            let partitions = self.partitions_on_disk(&disk);

            let mut cmd = String::from("echo -n \"o\\n");
            let mut i = 1;
            while i <= partitions.len() as u32 {
                cmd += partitions[i as usize - 1].fdisk_script_string(i).as_str();
                i += 1;
            }
            cmd += &format!("\\nw\\n\" | fdisk {} &>/dev/null", disk);
            cmds.push(cmd);
        }
        cmds
    }

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

#[allow(dead_code)]
impl Partition
{
    /// Return the string that can be `echo`ed into `fdisk` to create this Partition
    pub fn fdisk_script_string(&self, number: u32) -> String
    {
        format!(
            // n: create new partition
            // p: primary partition
            // use partition number specified
            // next line: default first sector
            // use partition size specified in instance
            r"n\np\n{}\n\n{}\n",
            number,
            &self.size,
        )
    }
}