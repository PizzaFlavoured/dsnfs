use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;

use crate::arguments::{ProgramConfig, ProgramMode};

// TODO: Figure IPv6 out?
pub fn listen(cfg: ProgramConfig) {
	let port = &cfg.get_port();
	let (listener, destination) = match cfg.get_mode() {
		ProgramMode::Receiving(data) => (
			TcpListener::bind(format!("0.0.0.0:{}", &port))
				.expect("Error: could not bind to the port."),
			{
				let d = data.get_destination();

				if d.exists() {
					if let Ok(iter) = d.read_dir() {
						if iter.count() > 0 {
							panic!("Error: the target directory must be empty!");
						}
					} else {
						panic!("Error: the destination must be an empty directory.");
					}
				} else {
					panic!("Error: the target directory does not exist.");
				}

				d
			},
		),
		ProgramMode::Sending(_) => panic!("Unreachable code."),
	};

	let (mut stream, addr) = listener
		.accept()
		.expect("Error: could not accept the incoming connection.");
	println!("Connection established with {}.", &addr);
	receive(&mut stream, destination)
}

fn receive(stream: &mut TcpStream, destination: PathBuf) {
	println!("{:#?}\n{:#?}", stream, destination);

	let filename = {
		let mut s = [0 as u8; 256];
		let mut clean_s = Vec::<u8>::with_capacity(256);
		stream
			.read_exact(&mut s)
			.expect("Failed to read the incoming file\'s name.");

		s.iter().for_each(|b| {
			if *b != b'\0' {
				clean_s.push(*b);
			}
		});

		clean_s
	};

	println!(
		"filename: {:?}",
		std::str::from_utf8(&filename).expect("Invalid filename received.")
	);

	let mut buffer = Vec::<u8>::new();
	stream
		.read_to_end(&mut buffer)
		.expect("Error: failed to read the file data");

	println!("{:?}", buffer);
	fs::write(
		&destination
			.canonicalize()
			.unwrap()
			.join(PathBuf::from(std::str::from_utf8(&filename).unwrap())),
		buffer,
	)
	.unwrap();
}
