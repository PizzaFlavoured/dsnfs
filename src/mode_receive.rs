use anyhow::Result;

use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;

use crate::arguments::{ProgramConfig, ProgramMode};

// TODO: Figure IPv6 out?
pub fn listen(cfg: ProgramConfig) -> Result<()> {
	let port = &cfg.get_port();
	let (listener, destination) = match cfg.get_mode() {
		ProgramMode::Receiving(data) => (
			TcpListener::bind(format!("0.0.0.0:{}", &port)),
			data.get_destination(),
		),
		ProgramMode::Sending(_) => panic!("Unreachable code."),
	};

	let (stream, addr) = listener?.accept()?;
	println!("Connection established with {}", &addr);
	receive(stream, destination)
}

fn receive(stream: TcpStream, destination: PathBuf) -> Result<()> {
	println!("{:#?}\n{:#?}", stream, destination);
	Ok(())
}
