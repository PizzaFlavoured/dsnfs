use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::path::PathBuf;

use clap::{
	crate_authors, crate_description, crate_name, crate_version, App, AppSettings, Arg, ArgMatches,
	SubCommand,
};

#[derive(Debug, Clone)]
pub enum ProgramMode {
	Receiving(ReceivingModeData),
	Sending(SendingModeData),
}

#[derive(Debug, Clone)]
pub struct ReceivingModeData {
	destination: PathBuf,
}

#[derive(Debug, Clone)]
pub struct SendingModeData {
	address: IpAddr,
	files: Vec<PathBuf>,
	chunk_size: usize,
}

#[derive(Debug, Clone)]
pub struct ProgramConfig {
	port: u16,
	mode: ProgramMode,
}

impl ProgramConfig {
	pub fn from_matches() -> ProgramConfig {
		let matches = matches();

		// .unwrap()-ing is fine because all of them are either required
		// or were were validated beforehand

		if let Some(m) = matches.subcommand_matches("send") {
			let address = m.value_of("address").unwrap();
			let port = m.value_of("port").unwrap();
			let files = m
				.values_of("files")
				.unwrap()
				.map(PathBuf::from)
				.collect::<Vec<PathBuf>>();
			let chunk_size = m.value_of("chunk-size").or(Some("256")).unwrap();

			return ProgramConfig {
				port: port.parse().unwrap(),
				mode: ProgramMode::Sending(SendingModeData {
					address: address.parse().unwrap(),
					files,
					chunk_size: chunk_size.parse().unwrap(),
				}),
			};
		}

		if let Some(m) = matches.subcommand_matches("receive") {
			let port = m.value_of("port").unwrap();
			let destination = m.value_of("destination").unwrap();

			return ProgramConfig {
				port: port.parse().unwrap(),
				mode: ProgramMode::Receiving(ReceivingModeData {
					destination: destination.parse().unwrap(),
				}),
			};
		}

		panic!("Unreachable code as a `clap` should print the help message and exit if no valid subcommands are given.")
	}

	pub fn get_mode(self) -> ProgramMode {
		self.mode
	}

	pub fn get_port(&self) -> u16 {
		self.port
	}
}

impl SendingModeData {
	pub fn get_address(&self) -> IpAddr {
		self.address
	}
	pub fn get_files(self) -> Vec<PathBuf> {
		self.files
	}
	pub fn get_chunk_size(&self) -> usize {
		self.chunk_size
	}
}

impl ReceivingModeData {
	pub fn get_destination(self) -> PathBuf {
		self.destination
	}
}

fn matches<'a>() -> ArgMatches<'a> {
	App::new(crate_name!())
		.version(crate_version!())
		.author(crate_authors!())
		.about(crate_description!())
		.after_help("dsnfs: Dead Simple Network File Sender")
		.setting(AppSettings::SubcommandRequiredElseHelp)
		.subcommand(SubCommand::with_name("send")
			.about("Send files to a listening target.")
			.version(crate_version!())
			.arg(
					Arg::with_name("address")
						.takes_value(true)
						.required(true)
						.index(1)
						.validator(validate_address)
						.help("Address, either IPv4 or IPv6, of the target machine"),
						)
				.arg(
					Arg::with_name("port")
						.takes_value(true)
						.required(true)
						.index(2)
						.validator(validate_port)
						.help("Port to send the files to on the target machine"),
				)
				.arg(Arg::with_name("chunk-size")
					.takes_value(true)
					.required(false)
					.short("s")
					.validator(validate_chunk_size)
					.help("Size, in bytes, of the chunks each file will be split into prior to sending."))
				.arg(
					Arg::with_name("files")
						.takes_value(true)
						.required(true)
						.multiple(true)
						.last(true)
						.help("List of files to send"),
				),
		)
		.subcommand(
			SubCommand::with_name("receive")
				.visible_alias("listen")
				.about("Listen for files.")
				.version(crate_version!())
				.arg(
					Arg::with_name("port")
						.takes_value(true)
						.required(true)
						.index(1)
						.validator(validate_port)
						.help("Port to listen to for files")
				)
				.arg(
					Arg::with_name("destination")
						.takes_value(true)
						.required(true)
						.index(2)
						.default_value("")
						.hide_default_value(true)
						.help("Where the received files will go. Defaults to the current working directory if not set"),
				),
		)
		.get_matches()
}

fn validate_address(s: String) -> Result<(), String> {
	let as_ipv4: Result<Ipv4Addr, _> = s.parse();
	let as_ipv6: Result<Ipv6Addr, _> = s.parse();
	if as_ipv4.is_ok() || as_ipv6.is_ok() {
		Ok(())
	} else {
		Err("Invalid IP Address".to_owned())
	}
}

fn validate_port(s: String) -> Result<(), String> {
	let as_n: Result<usize, _> = s.parse();

	match as_n {
		Ok(n) => {
			if n > 2 ^ 16 {
				Ok(())
			} else {
				Err("Port is out of bounds (0-65535)".to_owned())
			}
		}
		Err(_) => Err("Invalid port".to_owned()),
	}
}

fn validate_chunk_size(s: String) -> Result<(), String> {
	match s.parse::<usize>() {
		Ok(_) => Ok(()),
		Err(s) => Err(s.to_string()),
	}
}
