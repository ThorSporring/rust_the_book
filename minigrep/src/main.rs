use std::env;
use std::process;
use minigrep::Config;


fn main() {
    let args: Vec<String> = env::args().collect();

    let config: Config = Config::build(&args).unwrap_or_else(|err| {
        process::exit(1);
    });
    
    if let Err(e) = minigrep::run(config) {
        process::exit(1);
    }
}

