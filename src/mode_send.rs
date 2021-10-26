use anyhow::Result;

use std::net::TcpStream;
use std::path::PathBuf;

use crate::arguments::{ProgramConfig, ProgramMode};

pub fn send(cfg: ProgramConfig) -> Result<()> {
	let port = &cfg.get_port();
	let stream = match cfg.get_mode() {
		ProgramMode::Sending(data) => {
			let stream = TcpStream::connect(format!("{}:{}", data.get_address(), &port))?;
		}
		ProgramMode::Receiving(_) => panic!("Unreachable code."),
	};

	Ok(())
}
