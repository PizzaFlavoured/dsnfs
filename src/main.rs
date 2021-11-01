#![allow(dead_code, unused_variables)]
use anyhow::Result;

mod arguments;
mod receiver;
mod sender;

fn main() -> Result<()> {
	let config = arguments::ProgramConfig::from_matches();
	println!("{:#?}", config);
	match config.clone().get_mode() {
		arguments::ProgramMode::Receiving(_) => {
			receiver::listen(config)?;
		}
		arguments::ProgramMode::Sending(_) => {
			sender::send(config)?;
		}
	}
	Ok(())
}
