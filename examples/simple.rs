// A simple example extracted from Haiku's rune tool.
// locates a partition containing a FAT filesystem named "HAIKU"
//
// Likely won't compile without additional testing.

extern crate mbr;
extern crate fatfs;

use std::error::Error;
use std::path::PathBuf;
use std::io;
use std::io::{Seek, SeekFrom};
use std::fs;
use std::fs::File;
use fatfs::{FileSystem, FsOptions, BufStream};
use mbr::partition;

/// Write file at source to dest
pub fn write(source: PathBuf, dest: PathBuf) -> io::Result<u64> {
	return fs::copy(source.as_path(), dest.as_path());
}

/// Validate disk as containing Haiku and locate "boot" partition.
pub fn locate_boot_partition(disk: PathBuf) -> Result<partition::Partition,Box<dyn Error>> {
	let partitions = partition::read_partitions(disk.clone())?;
	for (_, partition) in partitions.iter().enumerate() {
		let sector_size = 512;
		if partition.p_type != 12 {
			// Ignore non-fat partitions
			continue;
		}
		let disk_handle = File::open(disk.clone())?;
		let mut buf_rdr = BufStream::new(disk_handle);
		buf_rdr.seek(SeekFrom::Start(partitions[0].p_lba as u64 * sector_size))?;
		let fs = match FileSystem::new(&mut buf_rdr, FsOptions::new()) {
			Ok(x) => x,
			Err(_) => continue,
		};
		if !fs.volume_label().to_uppercase().contains("HAIKU") {
			continue;
		}
		// TODO: More checks?
		return Ok(partition.clone());
	}
	return Err(From::from("no Haiku boot partitions"));
}

fn main() {
	let disk = PathBuf::from("/dev/sda");
	match locate_boot_partition(disk) {
		Ok(_x) => println!("Located HAIKU partition!"),
		Err(e) => println!("Unable to locate HAIKU partition: {}", e),
	}
}
