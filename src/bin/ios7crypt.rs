//! CLI IOS7Crypt tool

extern crate ios7crypt;

extern crate rand;

extern crate getopts;
use getopts::Options;

use std::process;
use std::env;

// Show short CLI spec
fn usage(brief : &str, opts : &getopts::Options) {
    println!("{}", (*opts).usage(brief));
}

// CLI entry point
fn main() {
    let mut rng = rand::thread_rng();

    let args : Vec<String> = env::args().collect();

    let program : &str = args[0].as_ref();

    let brief = format!("Usage: {} [options]", program);

    let mut opts : getopts::Options = Options::new();
    opts.optflag("h", "help", "print usage info");
    opts.optflag("v", "version", "print version info");
    opts.optopt("e", "encrypt", "encrypt a password", "VAL");
    opts.optopt("d", "decrypt", "decrypt a hash", "VAL");

    let optresult : Result<getopts::Matches, getopts::Fail> = opts.parse(&args[1..]);

    if optresult.is_err() {
        usage(&brief, &opts);
        process::abort();
    }

    let optmatches : getopts::Matches = optresult.unwrap();

    if optmatches.opt_present("e") {
        let password = optmatches.opt_str("encrypt").unwrap();

        println!("{}", ios7crypt::encrypt(&mut(rng), &password));
    }
    else if optmatches.opt_present("d") {
        let hash = optmatches.opt_str("decrypt").unwrap();

        let password_option = ios7crypt::decrypt(&hash);

        if password_option.is_none() {
            println!("Invalid hash");
            process::abort();
        }

        println!("{}", password_option.unwrap())
    }
    else if optmatches.opt_present("v") {
        println!("{} {}", program, env!("CARGO_PKG_VERSION"));
    } else if optmatches.opt_present("h") {
        usage(&brief, &opts);
        process::exit(0);
    } else {
        usage(&brief, &opts);
        process::abort();
    }
}
