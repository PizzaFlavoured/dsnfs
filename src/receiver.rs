use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;

use crate::arguments::{ProgramConfig, ProgramMode};
use crate::sender::{Packet, PacketType};

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
	//println!("{:#?}\n{:#?}", stream, destination);
	loop {
		let first_packet = receive_packet(stream);
		if let PacketType::Terminate = first_packet.get_type() {
			println!("Got packet to terminate; all files should have been transferred.");
			std::process::exit(0);
		};

		let filename = {
			let p = first_packet;
			match p.get_type() {
				PacketType::Name => {}
				_ => panic!("Error: expected a packet with the filename but got something else."),
			};

			let s = std::str::from_utf8(&p.get_data_slice()[..p.get_data_length() as usize])
				.expect("Error: unable to obtain a filename from the received packet.");

			s.to_owned()
		};
		println!("filename: {:?}", filename);

		let mut handle = fs::File::create(destination.join(&filename))
			.expect("Error: could not create the file to store the data in.");
		println!("\nDEBUG: `{:?}`\n", handle);

		// Loop until a packet of type PacketType::DataEnd is received
		'file: loop {
			let p = receive_packet(stream);
			match p.get_type() {
				PacketType::Data => {
					handle
						.write_all(p.get_data_slice())
						.expect("Error: could not write the received data to the file.");
					handle
						.flush()
						.expect("Error: could not flush the file handle after writing to it.");
				}
				PacketType::DataEnd => {
					handle
						.write_all(&p.get_data_slice()[..p.get_data_length() as usize])
						.expect("Error: could not write the received data to the file.");
					handle
						.flush()
						.expect("Error: could not flush the file handle after writing to it.");
					break 'file;
				}
				_ => {
					panic!("Error: expected a Data(End) packet but got something else instead.")
				}
			}
		}
		std::thread::sleep(std::time::Duration::from_millis(100));
	}
}

fn receive_packet(stream: &mut TcpStream) -> Packet {
	let mut t = [0; 1];
	let mut len = [0; 2];
	stream
		.read_exact(&mut t)
		.expect("Error: could not read the packet type.");
	stream
		.read_exact(&mut len)
		.expect("Error: could not read the packet length.");
	let t = PacketType::from_u8(t[0]);
	let len = u16::from_be_bytes(len);

	let mut data: Vec<u8> = vec![0; len as usize];
	stream
		.read_exact(&mut data)
		.expect("Error: could not read the packet data.");

	let a = Packet::new(t, &data);
	println!("Received: {:?}", a);
	a
}
