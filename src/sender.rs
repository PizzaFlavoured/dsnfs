use std::fs;
use std::io::Write;
use std::net::TcpStream;
use std::path::PathBuf;

use crate::arguments::{ProgramConfig, ProgramMode};

#[repr(u8)]
pub enum PacketType {
	Data = 1,
	DataEnd = 2,
	Name = 3,
}
impl PacketType {
	// Not a fan of doing this but it's better than adding a whole dependency
	// just for one enum
	pub fn from_u8(v: u8) -> PacketType {
		match v {
			1 => PacketType::Data,
			2 => PacketType::DataEnd,
			3 => PacketType::Name,
			_ => panic!("Error: invalid packet type."),
		}
	}
}

pub struct Packet {
	packet: Vec<u8>,
}
impl Packet {
	pub fn new(t: PacketType, contents: &[u8]) -> Packet {
		let mut p = Vec::<u8>::with_capacity(contents.len() + 2);
		let len = contents.len();
		p.push(t as u8);
		p.push(len as u8);

		p[2..].copy_from_slice(contents);

		Packet { packet: p }
	}

	// Convenience functions
	pub fn get_type(&self) -> PacketType {
		PacketType::from_u8(self.packet[0])
	}
	pub fn get_data_length(&self) -> u8 {
		self.packet[1]
	}
	pub fn get_data_slice(&self) -> &[u8] {
		&self.packet[2..]
	}
}

pub fn send(cfg: ProgramConfig) {
	let port = &cfg.get_port();

	let (mut stream, chunk_size, data) = match cfg.get_mode() {
		ProgramMode::Sending(data) => (
			TcpStream::connect(format!("{}:{}", &data.get_address(), &port))
				.expect("Error: could not establish a connection."),
			data.get_chunk_size(),
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
			if c.exists() {
				if c.is_dir() {
					panic!("Error: will not try to send directories for security reasons. (Try using `<dir>/*` and your shell should turn that into a list of the files in that directory.)");	
				} else {
					out.push(c)
				}
			}
		});
		out
	};
	//println!("{:#?}", files);

	files.iter().for_each(|f| {
		println!("Now sending: {:?}", f.file_name().unwrap());
		let handle = fs::read(f).expect("Error: unable to read the file.");

		// File name
		send_packet(
			&mut stream,
			Packet::new(
				PacketType::Name,
				f.file_name().unwrap().to_str().unwrap().as_bytes(),
			),
		);

		// Data
		let mut chunks = handle.as_slice().chunks_exact(chunk_size);
		for chunk in &mut chunks {
			send_packet(&mut stream, Packet::new(PacketType::Data, chunk));
		}

		// Now that the "full packets" have been sent, send the remaining one
		send_packet(
			&mut stream,
			Packet::new(PacketType::DataEnd, &chunks.remainder()),
		);
	});

	stream
		.shutdown(std::net::Shutdown::Both)
		.expect("Error: could not close the stream.");
}

fn send_packet(stream: &mut TcpStream, p: Packet) {
	stream
		.write_all(&p.packet)
		.expect("Error: unable to send packet.");
	stream.flush().expect("Error: unable to flush stream.");
}
