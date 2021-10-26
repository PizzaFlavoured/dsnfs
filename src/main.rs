#![allow(dead_code, unused_variables)]
use anyhow::Result;

mod arguments;
mod mode_receive;
mod mode_send;

fn main() -> Result<()> {
	let config = arguments::ProgramConfig::from_matches();
	println!("{:#?}", config);
	match config.clone().get_mode() {
		arguments::ProgramMode::Receiving(_) => {
			mode_receive::listen(config)?;
		}
		arguments::ProgramMode::Sending(_) => {
			mode_send::send(config)?;
		}
	}
	Ok(())
}
