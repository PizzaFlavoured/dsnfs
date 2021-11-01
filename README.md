# dsnfs
Dead Simple Network File Sender: a simple CLI utility capable of sending a file
via a network to another instance of the program listening for connections.

Not intended for actual use as it's really simple and written for the sake of
learning. It should work, however, should you want to use it over `rsync` for some
reason.

Written in Rust.


## Usage
`dsnfs --help`

Ok, for an actual example:
`dsnfs receive 8081 folder` - receiving end, will listen for connections on port 8081
The destination folder must be an empty directory.

`dsnsfs send 0.0.0.0 8081 -- "foo" "bar"` - sending end, will send the files `foo` and `bar` to the the program listening on 8081 at address 0.0.0.0 (example for local testing)
Optionally, `-s <u16>` may be provided, which tells the program how big each chunk the files will be split into prior to sending should be.


### Dependencies
`clap` - for argument processing


# License
MIT.
