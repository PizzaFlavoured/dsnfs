#![allow(dead_code, unused_variables)]

use std::error::Error;

mod arguments;

fn main() -> Result<(), Box<dyn Error>> {
	let config = arguments::ProgramConfig::from_matches();
	println!("{:#?}", config);
	Ok(())
}
