use anyhow::Result;

use std::fs;
use std::io::Write;
use std::net::TcpStream;
use std::path::{Path, PathBuf};

use crate::arguments::{ProgramConfig, ProgramMode};

#[repr(u8)]
pub enum Packet {
	Data = 0,
	DataEnd = 1,
	Name = 2,
}

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
		let handle = fs::read(f).expect("Error: unable to read the file.");

		// File name
		send_packet(
			&mut stream,
			Packet::Name,
			f.file_name().unwrap().to_str().unwrap().as_bytes(),
		);

		// Data
		let mut chunks = handle.as_slice().chunks_exact(256);
		for chunk in &mut chunks {
			send_packet(&mut stream, Packet::Data, chunk);
		}

		// Now that the "full packets" have been sent, send the remaining one
		send_packet(&mut stream, Packet::DataEnd, &chunks.remainder());
	});

	Ok(stream.shutdown(std::net::Shutdown::Both)?)
}

fn send_packet(stream: &mut TcpStream, t: Packet, contents: &[u8]) {
	let packet: Vec<u8> = {
		let len = contents.len();
		let mut packet = Vec::with_capacity(len + 2);

		packet[0] = t as u8;
		packet[1] = len as u8;

		for i in 0..len {
			packet[i + 2] = contents[i];
		}

		packet
	};

	stream
		.write_all(&packet)
		.expect("Error: unable to send packet.");
	stream.flush().expect("Error: unable to flush stream.");
}
