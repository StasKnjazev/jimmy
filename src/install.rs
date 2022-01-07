use crate::data::{InstallOptions, Partition};
use regex::Regex;

#[allow(dead_code)]
impl InstallOptions
{
    pub fn generate_shellscript(&self) -> String
    {
        let mut code = "#/bin/sh\n".to_string();
        code += "# Script automatically generated by jimmy-rs\n\n";
        code += &self.fdisk_cmds().join("\n");
        code += "\n\n";
        code += &self.mkfs_cmds().join("\n");
        code += "\n\n";
        code += &self.mount_cmds().join("\n");
        code += "\n";
        code
    }

    /// Generate a list of shell commands that mount every partition
    fn mount_cmds(&self) -> Vec<String>
    {
        let disks = self.unique_disks_used();

        let mut cmds = Vec::new();
        for disk in disks {
            let partitions = self.partitions_on_disk(&disk);
            let mut i = 0;
            while i < partitions.len() as u32 {
                let cmd = partitions[i as usize].mount_cmd(i);
                if cmd.is_some() {
                    cmds.push(cmd.unwrap());
                }
                i += 1;
            }
        }
        cmds
    }

    /// Generate a list of shell commands that format the partitions with `mkfs`, but only for the
    /// partitions whose format has been recognised
    fn mkfs_cmds(&self) -> Vec<String>
    {
        let disks = self.unique_disks_used();

        let mut cmds = Vec::new();
        for disk in disks {
            let partitions = self.partitions_on_disk(&disk);
            let mut i = 0;
            while i < partitions.len() as u32 {
                let cmd = partitions[i as usize].mkfs_cmd(i);
                if cmd.is_some() {
                    cmds.push(cmd.unwrap());
                }
                i += 1;
            }
        }
        cmds
    }

    /// Return the list of shell commands that create the partitions with `fdisk`
    fn fdisk_cmds(&self) -> Vec<String>
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

    /// Return the `mkfs` command that can format this partition, or `None` if the format of the
    /// partition wasn't recognised.
    pub fn mkfs_cmd(&self, number: u32) -> Option<String>
    {
        let cmd = match self.format.as_str() {
            "ext2" => "mkfs.ext2",
            "ext3" => "mkfs.ext3",
            "ext4" => "mkfs.ext4",
            "fat32" => "mkfs.fat -F 32",
            _ => ""
        }.to_string();
        if cmd.is_empty() { // if true, then we didn't recognised the format
            None
        } else {
            Some(cmd + " " + &self.get_partition_file(number))
        }
    }

    /// Return a shell command containing a `mkdir -p` call for the mounting point of the
    /// partition, and then a `mount` call to actually mount the partition, or `None` if the mount
    /// point wasn't specified
    pub fn mount_cmd(&self, number: u32) -> Option<String>
    {
        if self.mount.is_empty() {
            None
        } else {
            Some(format!(
                "mkdir -p /mnt{} && mount {} {}",
                self.mount,
                self.get_partition_file(number),
                self.mount,
            ))
        }
    }

    /// Return the path to the partition file (e.g. `/dev/sda1`, if provided `0`)
    fn get_partition_file(&self, number: u32) -> String
    {
        let disk = self.disk.clone();
        let n = &(number + 1).to_string();
        // NVME naming patterns are... abnormal.
        let re = Regex::new(r"/dev/nvme\d+n\d+").unwrap();
        if re.is_match(&disk) {
            return disk + "p" + n;
        }
        disk + n
    }
}
