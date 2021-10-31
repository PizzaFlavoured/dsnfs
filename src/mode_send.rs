use anyhow::Result;

use std::fs;
use std::io::Write;
use std::net::TcpStream;
use std::path::{Path, PathBuf};

use crate::arguments::{ProgramConfig, ProgramMode};

pub fn send(cfg: ProgramConfig) -> Result<()> {
	let port = &cfg.get_port();

	let (mut stream, data) = match cfg.get_mode() {
		ProgramMode::Sending(data) => (
			TcpStream::connect(format!("{}:{}", &data.get_address(), &port))?,
			data,
		),
		ProgramMode::Receiving(_) => panic!("Unreachable code."),
	};

	let files = {
		let v = data.get_files();
		let mut out = Vec::<PathBuf>::new();

		v.iter().for_each(|f| {
			let c = fs::canonicalize(f).unwrap_or_else(|_| {
				panic!(
					"Error: could not canonicalize `{:?}`.",
					f.file_name().unwrap()
				)
			});
			if Path::exists(&c) {
				out.push(c);
			}
		});
		out
	};

	println!("{:#?}", files);
	files.iter().for_each(|f| {
		println!("Now sending: {:?}", f.file_name().unwrap());
		let file = fs::read(f).expect("Failed to read the file.");

		// Filename
		// TODO: switch to a method wherein the filename's length is prepended
		// to the the actual filename instead of using a fixed size and padding
		// it...
		let name_array = {
			let n = f.file_name().unwrap().to_str().unwrap().as_bytes();
			let mut new: Vec<u8> = Vec::with_capacity(256);

			if n.len() > 256 {
				panic!("The filename, somehow, is >256 bytes long?");
			} else {
				for i in 0..256 {
					match n.get(i) {
						Some(b) => new.push(*b),
						None => new.push(0),
					}
				}
			}

			new
		};
		stream
			.write_all(name_array.as_slice())
			.expect("Failed to send the file name");
		stream
			.flush()
			.expect("Failed to flush the stream after writing the file name.");

		// Data
		stream
			.write_all(file.as_slice())
			.expect("Failed to send the data.");
		stream.write_all(&[0]).expect("Failed to send a 0");
		stream
			.flush()
			.expect("Failed to flush the stream after sending the data.");
	});

	Ok(stream.shutdown(std::net::Shutdown::Both)?)
}
