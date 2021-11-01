#![allow(dead_code, unused_variables)]
mod arguments;
mod receiver;
mod sender;

fn main() {
	let config = arguments::ProgramConfig::from_matches();
	println!("{:#?}", config);
	match config.clone().get_mode() {
		arguments::ProgramMode::Receiving(_) => {
			receiver::listen(config);
		}
		arguments::ProgramMode::Sending(_) => {
			sender::send(config);
		}
	}
}
