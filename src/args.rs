extern crate clap;

use std::io;
use std::env;
use std::path;
use std::ffi;

pub fn get_file_name() -> io::Result<String> {
    let name_arg = clap::Arg::with_name("name")
        .short("n")
        .long("name")
        .value_name("NAME")
        .help("set the junit suite name. This is also the file name")
        .required(true);

    clap::App::new("test junit")
        .about("Outputs junit output for CI runners like Jenkins")
        .arg(name_arg)
        .get_matches()
        .value_of("name")
        .map(str::to_string)
        .ok_or(io::Error::new(io::ErrorKind::NotFound, "Name arg not provided"))
        .or_else(|_| env::current_dir().and_then(get_last_path_part))
}

fn get_last_path_part(p: path::PathBuf) -> io::Result<String> {
    p.iter()
        .last()
        .and_then(ffi::OsStr::to_str)
        .map(str::to_string)
        .ok_or(io::Error::new(io::ErrorKind::NotFound, "Could not parse current dir"))
}
