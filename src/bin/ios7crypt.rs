//! CLI IOS7Crypt tool

extern crate ios7crypt;

extern crate rand;

extern crate getopts;
use getopts::Options;

use std::process;
use std::env;

// Show short CLI spec
fn usage(brief : &String, opts : &getopts::Options) {
    println!("{}", (*opts).usage(brief));
    process::exit(0);
}

// CLI entry point
fn main() {
  let mut rng = rand::thread_rng();

  let args : Vec<String> = env::args().collect();

  let program : &str = args[0].as_ref();

  let brief : String = format!("Usage: {} [options]", program);

  let mut opts : getopts::Options = Options::new();
  opts.optflag("h", "help", "print usage info");
  opts.optflag("v", "version", "print version info");
  opts.optopt("e", "encrypt", "encrypt a password", "VAL");
  opts.optopt("d", "decrypt", "decrypt a hash", "VAL");

  let optresult : Result<getopts::Matches, getopts::Fail> = opts.parse(&args[1..]);

  if optresult.is_err() {
    usage(&brief, &opts);
  }

  let optmatches : getopts::Matches = optresult.unwrap();

  if optmatches.opt_present("e") || optmatches.opt_present("encrypt") {
    let password = optmatches.opt_str("encrypt").unwrap();

    println!("{}", ios7crypt::encrypt(&mut(rng), &password));
  }
  else if optmatches.opt_present("d") || optmatches.opt_present("decrypt") {
    let hash = optmatches.opt_str("decrypt").unwrap();

    match ios7crypt::decrypt(&hash) {
      Some(password) => println!("{}", password),
      _ => panic!("Invalid hash")
    };
  }
  else if optmatches.opt_present("v") || optmatches.opt_present("version") {
    println!("{} {}", program, env!("CARGO_PKG_VERSION"));
  }

  usage(&brief, &opts);
}
