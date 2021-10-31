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
Note that the destination folder can be ommited, defaulting to the current working directory.

`dsnsfw send 0.0.0.0 8081 -- "foo" "bar"` - sending end, will send the files `foo` and `bar` to the the program listening on 8081 at address 0.0.0.0 (example for testing)


### Dependencies
`clap` - for argument processing
`anyhow` - error handling. Might remove it later though, as I barely use it
here.


### Current problems
* I *may* have messed up something in the connection logic and it currently dies after sending the first file.
* No error is given if the destination folder does not exist.
* Not the prettiest code.
* "Protocol", if it can be called that, needs to be refactored into something more sane.

# License
MIT.
