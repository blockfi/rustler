// build.rs
//
// Execute Erlang script to generate API lists and extract config.
//

use std::process::Command;
use std::env;
use std::path::Path;

fn main() {
	// use environment escript if available
	let escript = env::var("ESCRIPT").unwrap_or("escript".to_string());

	// emit generated files to OUT_DIR
	let out_dir = env::var("OUT_DIR")
		.map_err(|_|"Can't read OUT_DIR env variable.")
		.unwrap();

	let dst = Path::new(&out_dir);
	match Command::new(escript).arg("gen_api.erl").arg(dst).status()
		.map_err(|_|"Failed to start gen_api.erl.  Is 'escript' available in the path?")
		.unwrap().success() {
			true  => (),
			false => panic!("gen_api.erl encountered an error.")
		}
}
