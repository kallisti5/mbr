
/*
 * Copyright, 2017, Alexander von Gluck IV. All rights reserved.
 * Released under the terms of the MIT license.
 *
 * Authors:
 *   Alexander von Gluck IV <kallisti5@unixzen.com>
 */


use std::fmt;
use std::path::PathBuf;
use std::fs::File;
use std::io::{Read,SeekFrom,Error};
use std::io::prelude::*;
use std::vec::Vec;

// Start +446
#[derive(Debug, Clone)]
pub struct Partition {
	pub p_status: u8,
	pub p_cyl_begin: u8,
	pub p_head_begin: u8,
	pub p_sect_begin: u8,
	pub p_type: u8,
	pub p_cyl_end: u8,
	pub p_head_end: u8,
	pub p_sect_end: u8,
	pub p_lba: u32,
	pub p_size: u32,
}

fn read1<R: Read>(r: &mut R) -> u8 {
	let mut buf = [0];
	r.read(&mut buf).unwrap();
	buf[0]
}

fn read4<R: Read>(r: &mut R) -> u32 {
	let mut buf = [0, 0, 0, 0];
	r.read(&mut buf).unwrap();
	// TODO: Endian issues on non-x86 platforms? (maybe use byteorder crate?)
	//original: (buf[0] as u32) << 24 | (buf[1] as u32) << 16 | (buf[2] as u32) << 8 | (buf[3] as u32)
	(buf[3] as u32) << 24 | (buf[2] as u32) << 16 | (buf[1] as u32) << 8 | (buf[0] as u32)
}

fn read_partition(path: PathBuf, index: u8) -> Result<Partition, Error> {
	let mut f = File::open(&path.as_path())?;
	assert!(index < 4);

	let position: u64 = 446 + (16 * (index as u64));

	f.seek(SeekFrom::Start(position))?;
	let b = &mut f;

	let new_part = Partition {
		p_status: read1(b),
		p_head_begin: read1(b),
		p_sect_begin: read1(b),
		p_cyl_begin: read1(b),
		p_type: read1(b),
		p_head_end: read1(b),
		p_sect_end: read1(b),
		p_cyl_end: read1(b),
		p_lba: read4(b),
		p_size: read4(b),
	};

	return Ok(new_part);
}

impl fmt::Display for Partition {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut part_size = self.p_size;
		if self.p_size > 0 {
			part_size -= 1;
		}
		let end = self.p_lba + part_size;
		write!(f, "Partition. Type: 0x{:X}. Location: {}-{}.", self.p_type, self.p_lba, end)
	}
}

pub fn read_from_file(path: PathBuf) -> Result<Vec<Partition>, Error> {
	let mut partitions: Vec<Partition> = Vec::new();

	for i in [0,1,2,3].iter() {
		partitions.push(read_partition(path.clone(), *i)?);
	}

	return Ok(partitions);
}

pub fn table_dump(partitions: Vec<Partition>) {
	for i in partitions.iter() {
		print!("{}\n", i);
	}
}
